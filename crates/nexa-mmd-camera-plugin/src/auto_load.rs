//! Completion from our own child pipe; all MMD delivery stays on its UI thread.
use super::*;
use std::{io::Read, path::Path, sync::{Arc,atomic::{AtomicBool,AtomicUsize}},os::windows::io::AsRawHandle};

static NEXT:AtomicUsize=AtomicUsize::new(1);
pub(super) struct Pending {window:usize,token:usize,result:Option<Result<Option<PathBuf>,String>>,cancelled:Arc<AtomicBool>}
impl Drop for Pending {fn drop(&mut self){self.cancelled.store(true,Ordering::Relaxed);}}
#[link(name="user32")] unsafe extern "system" {
    fn PostMessageW(h:H,m:u32,w:usize,l:isize)->i32;
    fn SendMessageW(h:H,m:u32,w:usize,l:isize)->isize;
    fn IsWindowEnabled(h:H)->i32;
    fn EnumChildWindows(h:H,p:unsafe extern "system" fn(H,isize)->i32,l:isize)->i32;
    fn GetClassNameW(h:H,s:*mut u16,n:i32)->i32;
}
#[link(name="kernel32")] unsafe extern "system" {
    fn GlobalAlloc(flags:u32,size:usize)->H;
    fn GlobalLock(h:H)->*mut c_void;
    fn GlobalUnlock(h:H)->i32;
    fn GlobalFree(h:H)->H;
    fn PeekNamedPipe(h:H,b:*mut c_void,size:u32,read:*mut u32,available:*mut u32,left:*mut u32)->i32;
}
pub(super) fn message_id()->u32 {
    unsafe{RegisterWindowMessageW(wide("Nexa.Camera.ConversionCompleted.v1").as_ptr())}
}
pub(super) fn start(window:H,mut child:Child)->Result<Pending,String>{
    let token=NEXT.fetch_add(1,Ordering::Relaxed);
    let window=window as usize;
    let message=message_id();
    let mut stdout=child.stdout.take().ok_or("worker completion pipe unavailable")?;
    let cancelled=Arc::new(AtomicBool::new(false));let signal=Arc::clone(&cancelled);
    std::thread::Builder::new().name("nexa-camera-completion".into()).spawn(move||{
        // Read only available pipe bytes; cancellation never waits for Blender/GUI exit.
        let mut bytes=Vec::new();
        let result=loop {
            if signal.load(Ordering::Relaxed){return;}
            let mut available=0;
            let open=unsafe{PeekNamedPipe(stdout.as_raw_handle(),null_mut(),0,null_mut(),&mut available,null_mut())}!=0;
            if open&&available>0{
                if bytes.len()+available as usize>131_072{break Err("変換完了通知が大きすぎます。".into());}
                let mut buffer=vec![0u8;(available as usize).min(8192)];
                match stdout.read(&mut buffer){Ok(0)=>{},Ok(n)=>bytes.extend_from_slice(&buffer[..n]),Err(e)=>break Err(e.to_string())}
                continue;
            }
            match child.try_wait(){
                Ok(Some(status))=>break if status.success(){parse_result(&bytes)}else{Err("変換に失敗しました。VMDの自動読み込みは行っていません。".into())},
                Ok(None)=>std::thread::park_timeout(std::time::Duration::from_millis(50)),
                Err(e)=>break Err(e.to_string()),
            }
        };
        if let Ok(mut current)=WORKER.lock(){
            if let Some(p)=current.as_mut().filter(|p|p.window==window&&p.token==token){
                p.result=Some(result);
                if unsafe{PostMessageW(window as H,message,token,0)}==0{*current=None;}
            }
        }
    }).map_err(|e|format!("completion monitor: {e}"))?;
    Ok(Pending{window,token,result:None,cancelled})
}
pub(super) fn cancel(window:H){
    if let Ok(mut p)=WORKER.lock(){if p.as_ref().is_some_and(|p|p.window==window as usize){*p=None;}}
}
fn take_result(p:&mut Option<Pending>,window:usize,token:usize)->Option<Result<Option<PathBuf>,String>>{
    if !p.as_ref().is_some_and(|p|p.window==window&&p.token==token&&p.result.is_some()){return None;}
    p.take().and_then(|mut p|p.result.take())
}
pub(super) fn complete(window:H,token:usize)->bool{
    let result=WORKER.lock().ok().and_then(|mut p|take_result(&mut p,window as usize,token));
    let Some(result)=result else{return false;};
    let (outcome,path)=match result{
        Ok(Some(path))=>(validate_camera(&path).and_then(|_|deliver(window,&path)),Some(path)),
        Ok(None)=>return true,
        Err(e)=>(Err(e),None),
    };
    let (text,flags)=match outcome {
        Ok(())=>("変換したカメラVMDをMMDの標準読み込み経路へ渡しました。\nタイムラインと構図を確認してください。".to_owned(),0x40),
        Err(e)=>{
            let saved=path.filter(|p|p.is_file()).map(|p|format!("\n変換済みVMD: {}\n標準のモーション読み込みを使用できます。",p.display())).unwrap_or_default();
            (format!("自動読み込みを完了できませんでした。\n{e}{saved}"),0x10)
        },
    };
    if unsafe{IsWindow(window)}!=0{unsafe{MessageBoxW(window,wide(&text).as_ptr(),wide("Nexa Camera").as_ptr(),flags);}}
    true
}
fn parse_result(bytes:&[u8])->Result<Option<PathBuf>,String>{
    if bytes==b"NEXACANCEL1"{return Ok(None);}
    if bytes.len()>131_072||!bytes.starts_with(b"NEXAVMD1\n"){return Err("不正な変換完了通知です。".into());}
    let path=std::str::from_utf8(&bytes[9..]).map_err(|_|"完了通知のパスが不正です。")?;
    let path=PathBuf::from(path);
    if !path.is_absolute()||path.as_os_str().is_empty()||path.to_string_lossy().contains('\0'){
        return Err("完了通知には絶対パスが必要です。".into());
    }
    Ok(Some(path))
}
fn validate_camera(path:&Path)->Result<(),String>{
    if !path.extension().is_some_and(|s|s.eq_ignore_ascii_case("vmd")){return Err("出力がVMDではありません。".into());}
    let metadata=std::fs::symlink_metadata(path).map_err(|e|e.to_string())?;
    if !metadata.is_file()||metadata.file_type().is_symlink()||metadata.len()>8_000_000{return Err("通常のカメラVMDファイルではありません。".into());}
    let bytes=std::fs::read(path).map_err(|e|e.to_string())?;
    validate_bytes(&bytes)
}
fn validate_bytes(b:&[u8])->Result<(),String>{
    let invalid=||"生成ファイルがcamera-only VMD形式ではありません。".to_owned();
    if b.len()<74||&b[..25]!=b"Vocaloid Motion Data 0002"||b[50..58]!=[0;8]{return Err(invalid());}
    let n=u32::from_le_bytes(b[58..62].try_into().unwrap()) as usize;
    if n==0||n>108_001||b.len()!=74+n*61||b[62+n*61..]!=[0;12]{return Err(invalid());}
    let mut previous=None;
    for key in b[62..62+n*61].chunks_exact(61){
        let frame=u32::from_le_bytes(key[..4].try_into().unwrap());
        if previous.is_some_and(|p|p>=frame)||key[4..32].chunks_exact(4).any(|v|!f32::from_le_bytes(v.try_into().unwrap()).is_finite())
            || !(1..=135).contains(&u32::from_le_bytes(key[56..60].try_into().unwrap()))||key[60]>1{return Err(invalid());}
        previous=Some(frame);
    }
    Ok(())
}
struct Context {candidates:u32,camera:bool}
unsafe extern "system" fn find_context(h:H,data:isize)->i32{
    let mut class=[0u16;64];
    let n=unsafe{GetClassNameW(h,class.as_mut_ptr(),64)}.max(0) as usize;
    if String::from_utf16_lossy(&class[..n])!="ComboBox"{return 1;}
    let n=unsafe{SendMessageW(h,0x149,0,0)}; // CB_GETLBTEXTLEN: first entry
    if !(1..=128).contains(&n){return 1;}
    let mut label=vec![0u16;n as usize+1];
    let copied=unsafe{SendMessageW(h,0x148,0,label.as_mut_ptr() as isize)};
    if copied!=n{return 1;}
    let label=String::from_utf16_lossy(&label[..n as usize]);
    if label=="camera/light/accessory"||label=="カメラ・照明・アクセサリ"{
        let context=unsafe{&mut *(data as *mut Context)};
        context.candidates+=1;context.camera=unsafe{SendMessageW(h,0x147,0,0)}==0;
    }
    1
}
fn deliver(window:H,path:&Path)->Result<(),String>{
    if unsafe{IsWindow(window)}==0||unsafe{IsWindowEnabled(window)}==0{return Err("MMDが閉じられているか、別のダイアログが開いています。".into());}
    let mut context=Context{candidates:0,camera:false};
    unsafe{EnumChildWindows(window,find_context,&mut context as *mut Context as isize);}
    if context.candidates!=1||!context.camera{return Err("MMDをカメラ・照明・アクセサリ操作に切り替えてください。".into());}
    use std::os::windows::ffi::OsStrExt;
    let path:Vec<u16>=path.as_os_str().encode_wide().chain([0,0]).collect();
    if path.len()>32768{return Err("パスが長すぎます。".into());}
    unsafe{
        let memory=GlobalAlloc(0x42,20+path.len()*2);
        if memory.is_null(){return Err("ファイル通知のメモリを確保できません。".into());}
        let data=GlobalLock(memory).cast::<u8>();
        if data.is_null(){GlobalFree(memory);return Err("ファイル通知を作成できません。".into());}
        data.cast::<u32>().write(20);data.add(16).cast::<u32>().write(1);
        std::ptr::copy_nonoverlapping(path.as_ptr(),data.add(20).cast::<u16>(),path.len());
        GlobalUnlock(memory);
        // Standard HDROP ownership transfers to the target, which calls DragFinish.
        SendMessageW(window,0x233,memory as usize,0);
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn completion_is_bound_and_consumed_once(){
        let signal=Arc::new(AtomicBool::new(false));
        let mut p=Some(Pending{window:1,token:42,result:Some(Ok(Some(PathBuf::from("C:/camera.vmd")))),cancelled:Arc::clone(&signal)});
        assert!(take_result(&mut p,2,42).is_none());assert!(take_result(&mut p,1,41).is_none());
        assert!(take_result(&mut p,1,42).unwrap().unwrap().is_some());assert!(take_result(&mut p,1,42).is_none());assert!(signal.load(Ordering::Relaxed));
        let signal=Arc::new(AtomicBool::new(false));
        let mut p=Some(Pending{window:1,token:43,result:None,cancelled:Arc::clone(&signal)});assert!(take_result(&mut p,1,43).is_none());
        p=None;assert!(take_result(&mut p,1,43).is_none());
        assert!(signal.load(Ordering::Relaxed));
    }
    #[test] fn cancellation_and_foreign_stdout_cannot_supply_a_path(){
        assert_eq!(parse_result(b"NEXACANCEL1").unwrap(),None);
        assert!(parse_result(b"Camera VMD: C:/foreign.vmd").is_err());
        assert!(parse_result(b"NEXAVMD1\nrelative.vmd").is_err());
        assert!(parse_result(b"NEXAVMD1\nC:/a\0.vmd").is_err());
        assert!(parse_result(b"NEXAVMD1\nC:/camera.vmd").unwrap().is_some());
        let mut p=Some(Pending{window:1,token:1,result:Some(Err("failed".into())),cancelled:Arc::new(AtomicBool::new(false))});
        assert!(take_result(&mut p,1,1).unwrap().is_err());
    }
    #[test] fn camera_validation_rejects_model_light_and_truncated_records(){
        let mut b=vec![0;135];b[..25].copy_from_slice(b"Vocaloid Motion Data 0002");
        b[58..62].copy_from_slice(&1u32.to_le_bytes());b[118..122].copy_from_slice(&40u32.to_le_bytes());
        assert!(validate_bytes(&b).is_ok());
        b[50]=1;assert!(validate_bytes(&b).is_err());b[50]=0;
        b[123]=1;assert!(validate_bytes(&b).is_err());b[123]=0;
        b.pop();assert!(validate_bytes(&b).is_err());
    }
}
