//! Small native settings window for the independent camera worker.
use crate::{worker::{Options,convert_localized},i18n::{Language,Text,external}};
use std::sync::{Arc,Mutex,atomic::{AtomicBool,Ordering}};
use std::{ffi::{c_void,OsString},mem::{size_of,zeroed},os::windows::ffi::OsStringExt,path::{Path,PathBuf},ptr::{null,null_mut}};
type H=*mut c_void;
type Proc=unsafe extern "system" fn(H,u32,usize,isize)->isize;
#[repr(C)] struct Class {style:u32,proc:Option<Proc>,cls:i32,wnd:i32,instance:H,icon:H,cursor:H,background:H,menu:*const u16,name:*const u16}
#[repr(C)] struct Msg {hwnd:H,message:u32,w:usize,l:isize,time:u32,point:[i32;2],private:u32}
#[repr(C)] struct Rect {left:i32,top:i32,right:i32,bottom:i32}
#[repr(C)] struct OpenFile {
    size:u32,owner:H,instance:H,filter:*const u16,custom:*mut u16,max_custom:u32,index:u32,
    file:*mut u16,max_file:u32,title_file:*mut u16,max_title:u32,initial:*const u16,title:*const u16,
    flags:u32,offset:u16,extension:u16,default_ext:*const u16,data:isize,
    hook:Option<unsafe extern "system" fn(H,u32,usize,isize)->usize>,template:*const u16,reserved:*mut c_void,reserved_word:u32,flags_ex:u32,
}
const _:()={assert!(size_of::<Class>()==72);assert!(size_of::<Msg>()==48);assert!(size_of::<OpenFile>()==152);};
#[link(name="user32")] unsafe extern "system" {
    fn RegisterClassW(c:*const Class)->u16; fn UnregisterClassW(c:*const u16,i:H)->i32;
    fn CreateWindowExW(ex:u32,c:*const u16,t:*const u16,s:u32,x:i32,y:i32,w:i32,h:i32,p:H,m:H,i:H,a:*mut c_void)->H;
    fn DefWindowProcW(h:H,m:u32,w:usize,l:isize)->isize;
    fn SetWindowLongPtrW(h:H,i:i32,v:isize)->isize; fn GetWindowLongPtrW(h:H,i:i32)->isize;
    fn AdjustWindowRectEx(r:*mut Rect,s:u32,m:i32,e:u32)->i32;
    fn ShowWindow(h:H,c:i32)->i32; fn UpdateWindow(h:H)->i32;fn IsWindow(h:H)->i32;fn DestroyWindow(h:H)->i32;
    fn PostQuitMessage(c:i32);fn GetMessageW(m:*mut Msg,h:H,a:u32,b:u32)->i32;
    fn TranslateMessage(m:*const Msg)->i32;fn DispatchMessageW(m:*const Msg)->isize;fn IsDialogMessageW(h:H,m:*mut Msg)->i32;
    fn GetDlgItem(h:H,i:i32)->H;fn GetWindowTextLengthW(h:H)->i32;fn GetWindowTextW(h:H,t:*mut u16,n:i32)->i32;
    fn SetWindowTextW(h:H,t:*const u16)->i32;fn SendMessageW(h:H,m:u32,w:usize,l:isize)->isize;
    fn SetFocus(h:H)->H;fn LoadCursorW(i:H,n:*const u16)->H;
    fn EnableWindow(h:H,enable:i32)->i32;fn SetTimer(h:H,id:usize,ms:u32,p:*const c_void)->usize;fn KillTimer(h:H,id:usize)->i32;
    fn MessageBoxW(h:H,t:*const u16,c:*const u16,f:u32)->i32;
}
#[link(name="kernel32")] unsafe extern "system" {fn GetModuleHandleW(n:*const u16)->H;}
#[link(name="gdi32")] unsafe extern "system" {fn GetStockObject(i:i32)->H;}
#[link(name="comdlg32")] unsafe extern "system" {
    fn GetOpenFileNameW(o:*mut OpenFile)->i32;fn GetSaveFileNameW(o:*mut OpenFile)->i32;fn CommDlgExtendedError()->u32;
}
struct Job {options:Options,cancel:Arc<AtomicBool>,result:Arc<Mutex<Option<Result<String,String>>>>}
impl Drop for Job {fn drop(&mut self){self.cancel.store(true,Ordering::Relaxed);}}
struct State {base:PathBuf,value:Option<(Options,String)>,blenders:Vec<crate::blender_detection::Candidate>,language:Language,job:Option<Job>,cancel_requested:bool}
fn language(h:H)->Language{
    if h.is_null(){return Language::Ja;}
    let state=unsafe{GetWindowLongPtrW(h,-21)} as *const State;
    if state.is_null(){Language::Ja}else{unsafe{(*state).language}}
}
struct Window {hwnd:H,instance:H,class:Vec<u16>}
impl Drop for Window {fn drop(&mut self){unsafe {if IsWindow(self.hwnd)!=0 {DestroyWindow(self.hwnd);}UnregisterClassW(self.class.as_ptr(),self.instance);}}}
fn wide(s:&str)->Vec<u16>{s.encode_utf16().chain(Some(0)).collect()}
fn error(language:Language,api:&str)->String{external(language,Text::GuiFailed,format!("{api}: {}",std::io::Error::last_os_error()))}
fn show_message(owner:H,text:&str,failed:bool){unsafe{MessageBoxW(owner,wide(text).as_ptr(),wide("Nexa Blend Camera").as_ptr(),if failed{0x10}else{0x40});}}
pub fn message(text:&str,failed:bool){show_message(null_mut(),text,failed);}
fn control(h:H,c:&str,t:&str,id:i32,b:[i32;4],s:u32)->Result<(),String>{
    let [x,y,w,height]=b;
    let child=unsafe{CreateWindowExW(if c=="EDIT"{0x200}else{0},wide(c).as_ptr(),wide(t).as_ptr(),0x50000000|s,x,y,w,height,h,id as usize as H,GetModuleHandleW(null()),null_mut())};
    if child.is_null(){return Err(error(language(h),"CreateWindowExW(control)"));}
    unsafe{SendMessageW(child,0x30,GetStockObject(17) as usize,1);if c=="EDIT"{SendMessageW(child,0xc5,32767,0);}}
    Ok(())
}
fn read(h:H,id:i32)->Vec<u16>{unsafe{
    let child=GetDlgItem(h,id);let length=GetWindowTextLengthW(child).max(0) as usize;
    let mut value=vec![0;length+1];let copied=GetWindowTextW(child,value.as_mut_ptr(),value.len() as i32);
    value.truncate(copied.max(0) as usize);value
}}
fn text(h:H,id:i32)->Result<String,String>{String::from_utf16(&read(h,id)).map_err(|_|language(h).text(Text::Unicode).into())}
fn optional(h:H,id:i32)->Result<Option<String>,String>{let s=text(h,id)?;Ok(if s.is_empty(){None}else{Some(s)})}
fn number(h:H,id:i32,name:&str)->Result<Option<f64>,String>{
    let s=text(h,id)?;if s.trim().is_empty(){return Ok(None);}
    let n=s.trim().parse::<f64>().map_err(|_|language(h).format(Text::Number,&[("name",name)]))?;
    if !n.is_finite(){return Err(language(h).format(Text::FiniteNumber,&[("name",name)]));}Ok(Some(n))
}
fn dimension(h:H,id:i32)->Result<u32,String>{text(h,id)?.trim().parse::<u32>().ok().filter(|n|(1..=16384).contains(n)).ok_or(language(h).text(Text::Dimensions).into())}
fn checked(h:H,id:i32)->bool{unsafe{SendMessageW(GetDlgItem(h,id),0xf0,0,0)==1}}
fn ext(p:&Path,e:&str)->bool{p.extension().and_then(|v|v.to_str()).is_some_and(|v|v.eq_ignore_ascii_case(e))}
fn collect(h:H,base:&Path)->Result<Options,String>{
    let path=|id|->Result<PathBuf,String>{let v=read(h,id);if v.is_empty(){return Err(language(h).text(Text::SelectFile).into());}
        let p=PathBuf::from(OsString::from_wide(&v));Ok(if p.is_absolute(){p}else{base.join(p)})};
    let blender=path(10)?;let source=path(11)?;let output=path(12)?;
    if !blender.is_file()||!ext(&blender,"exe"){return Err(language(h).text(Text::SelectBlender).into());}
    if !source.is_file()||!ext(&source,"blend"){return Err(language(h).text(Text::SelectBlend).into());}
    if !ext(&output,"vmd")||!output.parent().is_some_and(|p|p.is_dir()){return Err(language(h).text(Text::SelectOutput).into());}
    let start=number(h,15,language(h).text(Text::Start))?;let end=number(h,16,language(h).text(Text::End))?;
    if start.zip(end).is_some_and(|(a,b)|a>b){return Err(language(h).text(Text::StartEnd).into());}
    let units_per_meter=number(h,17,language(h).text(Text::Units))?.filter(|v|*v>0.0).ok_or(language(h).text(Text::PositiveUnits))?;
    Ok(Options{blender,source,output,scene:optional(h,13)?,camera:optional(h,14)?,start,end,units_per_meter,
        width:dimension(h,18)?,height:dimension(h,19)?,allow_fov_approximation:checked(h,20),overwrite:checked(h,21)})
}
fn select_file(h:H,id:i32,current:&[u16],lang:Language)->Result<Option<PathBuf>,String>{
    let mut file=vec![0u16;32768];
    if current.len()>=file.len(){return Err(lang.text(Text::FilenameLong).into());}file[..current.len()].copy_from_slice(&current);
    let filter=wide(lang.text(match id{10=>Text::BlenderFilter,11=>Text::BlendFilter,_=>Text::VmdFilter}));
    let title=wide(lang.text(match id{10=>Text::Blender,11=>Text::Input,_=>Text::Output}));
    let extension=wide(match id{10=>"exe",11=>"blend",_=>"vmd"});
    let mut o:OpenFile=unsafe{zeroed()};o.size=size_of::<OpenFile>() as u32;o.owner=h;o.filter=filter.as_ptr();o.index=1;
    o.title=title.as_ptr();o.file=file.as_mut_ptr();o.max_file=file.len() as u32;o.default_ext=extension.as_ptr();
    o.flags=0x0208080c|if id==12{0x10000}else{0x1000};
    let ok=unsafe{if id==12{GetSaveFileNameW(&mut o)}else{GetOpenFileNameW(&mut o)}};
    if ok==0{let code=unsafe{CommDlgExtendedError()};return if code==0{Ok(None)}else{Err(lang.format(Text::DialogFailed,&[("code",&format!("{code:08x}"))]))};}
    let end=file.iter().position(|v|*v==0).ok_or(lang.text(Text::DialogPath))?;
    let path=PathBuf::from(OsString::from_wide(&file[..end]));
    if id==11&&(!path.is_file()||!ext(&path,"blend")){return Err(lang.text(Text::SelectBlend).into());}
    Ok(Some(path))
}
fn set_path(h:H,id:i32,path:&Path)->Result<(),String>{
    let value:Vec<u16>=std::os::windows::ffi::OsStrExt::encode_wide(path.as_os_str()).chain(Some(0)).collect();
    if unsafe{SetWindowTextW(GetDlgItem(h,id),value.as_ptr())}==0{return Err(error(language(h),"SetWindowTextW"));}Ok(())
}
fn browse(h:H,id:i32)->Result<(),String>{
    if let Some(path)=select_file(h,id,&read(h,id),language(h))?{set_path(h,id,&path)?;}Ok(())
}
fn fill_blenders(h:H,candidates:&[crate::blender_detection::Candidate])->Result<(),String>{
    if let Some(first)=candidates.first(){
        for candidate in candidates{unsafe{SendMessageW(GetDlgItem(h,22),0x143,0,wide(&candidate.label()).as_ptr() as isize);}}
        unsafe{SendMessageW(GetDlgItem(h,22),0x160,700,0);SendMessageW(GetDlgItem(h,22),0x14e,0,0);}
        set_path(h,10,&first.path)?;
    }else{unsafe{SendMessageW(GetDlgItem(h,22),0x143,0,wide(language(h).text(Text::NotDetected)).as_ptr() as isize);SendMessageW(GetDlgItem(h,22),0x14e,0,0);}}
    Ok(())
}
fn set_busy(h:H,busy:bool){
    for id in [1,10,11,12,13,14,15,16,17,18,19,20,21,22,110,111,112]{unsafe{EnableWindow(GetDlgItem(h,id),i32::from(!busy));}}
    unsafe{SetWindowTextW(GetDlgItem(h,23),wide(if busy{language(h).text(Text::Working)}else{""}).as_ptr());}
}
fn request_cancel(h:H,state:&mut State){
    if let Some(job)=&state.job{
        state.cancel_requested=true;job.cancel.store(true,Ordering::Relaxed);
        unsafe{SetWindowTextW(GetDlgItem(h,23),wide(state.language.text(Text::Cancelling)).as_ptr());}
    }else{unsafe{DestroyWindow(h);}}
}
fn begin_conversion(h:H,state:&mut State)->Result<(),String>{
    if state.job.is_some(){return Ok(());}
    let options=collect(h,&state.base)?;
    crate::worker::validate(&options,state.language)?;
    let cancel=Arc::new(AtomicBool::new(false));let signal=Arc::clone(&cancel);
    let result=Arc::new(Mutex::new(None));let completed=Arc::clone(&result);
    let input=options.clone();let language=state.language;
    if unsafe{SetTimer(h,1,100,null())}==0{return Err(error(language,"SetTimer"));}
    if let Err(e)=std::thread::Builder::new().name("nexa-camera-conversion".into()).spawn(move||{
        let value=convert_localized(&input,language,Some(&signal));
        if let Ok(mut result)=completed.lock(){*result=Some(value);}
    }){unsafe{KillTimer(h,1);}return Err(external(language,Text::ThreadFailed,e));}
    state.cancel_requested=false;state.job=Some(Job{options,cancel,result});set_busy(h,true);Ok(())
}
unsafe extern "system" fn procedure(h:H,m:u32,w:usize,l:isize)->isize{
    if m==2{unsafe{KillTimer(h,1);PostQuitMessage(0);}return 0;}
    let pointer=unsafe{GetWindowLongPtrW(h,-21)} as *mut State;
    if !pointer.is_null(){
        let state=unsafe{&mut *pointer};
        if m==0x10{request_cancel(h,state);return 0;}
        if m==0x113&&w==1{
            let result=state.job.as_ref().and_then(|job|job.result.lock().ok().and_then(|mut r|r.take()));
            if let Some(result)=result{
                unsafe{KillTimer(h,1);}let job=state.job.take().unwrap();
                match result{
                    // Success means the output is already published; cancellation cannot undo it.
                    Ok(report)=>{state.value=Some((job.options.clone(),report));unsafe{DestroyWindow(h);}},
                    Err(_) if state.cancel_requested=>{unsafe{DestroyWindow(h);}},
                    Err(e)=>{set_busy(h,false);show_message(h,&e,true);}
                }
            }return 0;
        }
        if m==0x111&&(w&0xffff)==22&&(w>>16)==1&&state.job.is_none(){
            let selected=unsafe{SendMessageW(GetDlgItem(h,22),0x147,0,0)};
            if selected>=0{if let Some(path)=state.blenders.get(selected as usize).map(|c|c.path.clone()){
                if let Err(e)=set_path(h,10,&path){show_message(h,&e,true);}
            }}return 0;
        }
        if m==0x111&&(w>>16)==0{
            match (w&0xffff) as i32{
                id@110..=112 if state.job.is_none()=>{if let Err(e)=browse(h,id-100){show_message(h,&e,true);}return 0;}
                1=>{if let Err(e)=begin_conversion(h,state){show_message(h,&e,true);}return 0;}
                2=>{request_cancel(h,state);return 0;}_=>{}
            }
        }
    }
    unsafe{DefWindowProcW(h,m,w,l)}
}
pub fn show(language:Language)->Result<Option<(Options,String)>,String>{
    let Some(source)=select_file(null_mut(),11,&[],language)? else{return Ok(None);};
    show_settings(&source,language)
}
fn show_settings(source:&Path,language:Language)->Result<Option<(Options,String)>,String>{
    let mut state=Box::new(State{base:std::env::current_dir().map_err(|e|external(language,Text::Io,e))?,value:None,blenders:crate::blender_detection::detect(),language,job:None,cancel_requested:false});
    let name=wide("NexaMmdCameraWorkerWindow");let instance=unsafe{GetModuleHandleW(null())};
    if instance.is_null(){return Err(error(language,"GetModuleHandleW"));}
    let class=Class{style:0,proc:Some(procedure),cls:0,wnd:0,instance,icon:null_mut(),cursor:unsafe{LoadCursorW(null_mut(),32512usize as _)},background:16usize as H,menu:null(),name:name.as_ptr()};
    if unsafe{RegisterClassW(&class)}==0{return Err(error(language,"RegisterClassW"));}
    let style=0x00ca0000;let ex=0x00010000;
    let mut r=Rect{left:0,top:0,right:760,bottom:512};
    if unsafe{AdjustWindowRectEx(&mut r,style,0,ex)}==0{let e=error(language,"AdjustWindowRectEx");unsafe{UnregisterClassW(name.as_ptr(),instance);}return Err(e);}
    let hwnd=unsafe{CreateWindowExW(ex,name.as_ptr(),wide("Nexa Blend Camera").as_ptr(),style,i32::MIN,i32::MIN,r.right-r.left,r.bottom-r.top,null_mut(),null_mut(),instance,null_mut())};
    if hwnd.is_null(){let e=error(language,"CreateWindowExW");unsafe{UnregisterClassW(name.as_ptr(),instance);}return Err(e);}
    let _window=Window{hwnd,instance,class:name};let pointer=&mut *state as *mut State as isize;
    unsafe{SetWindowLongPtrW(hwnd,-21,pointer);}
    if unsafe{GetWindowLongPtrW(hwnd,-21)}!=pointer{return Err(language.text(Text::GuiState).into());}
    populate(hwnd,&state,source)?;
    unsafe{ShowWindow(hwnd,5);UpdateWindow(hwnd);SetFocus(GetDlgItem(hwnd,10));}
    let mut msg:Msg=unsafe{zeroed()};
    loop{match unsafe{GetMessageW(&mut msg,null_mut(),0,0)}{
        -1=>return Err(error(language,"GetMessageW")),0=>break,
        _=>unsafe{if IsDialogMessageW(hwnd,&mut msg)==0{TranslateMessage(&msg);DispatchMessageW(&msg);}}
    }}
    Ok(state.value.take())
}
fn populate(hwnd:H,state:&State,source:&Path)->Result<(),String>{
    let language=state.language;
    for (row,(id,label)) in [(10,Text::Blender),(11,Text::Input),(12,Text::Output)].into_iter().enumerate(){
        let y=16+row as i32*58;control(hwnd,"STATIC",language.text(label),0,[20,y,720,20],0)?;
        control(hwnd,"EDIT","",id,[20,y+22,if id==10{435}else{610},26],0x10080)?;control(hwnd,"BUTTON",language.text(Text::Browse),id+100,[640,y+21,100,28],0x10000)?;
    }
    control(hwnd,"COMBOBOX","",22,[465,38,165,220],0x10003)?;
    fill_blenders(hwnd,&state.blenders)?;
    set_path(hwnd,11,source)?;
    set_path(hwnd,12,&source.with_extension("camera.vmd"))?;
    for (id,label,x,y,width,value) in [
        (13,Text::Scene,20,198,340,""),(14,Text::Camera,390,198,350,""),
        (15,Text::Start,20,262,165,""),(16,Text::End,205,262,165,""),
        (17,Text::Units,390,262,350,"12.5"),(18,Text::Width,20,326,165,"1920"),(19,Text::Height,205,326,165,"1080")]{
        control(hwnd,"STATIC",language.text(label),0,[x,y,width,20],0)?;control(hwnd,"EDIT",value,id,[x,y+22,width,26],0x10080)?;
    }
    control(hwnd,"STATIC",language.text(Text::MatchDimensions),0,[390,351,350,22],0)?;
    control(hwnd,"BUTTON",language.text(Text::Approximation),20,[20,382,720,24],0x10003)?;
    control(hwnd,"BUTTON",language.text(Text::Overwrite),21,[20,411,720,24],0x10003)?;
    control(hwnd,"BUTTON",language.text(Text::Convert),1,[525,466,100,30],0x10001)?;control(hwnd,"BUTTON",language.text(Text::Cancel),2,[640,466,100,30],0x10000)?;
    control(hwnd,"STATIC","",23,[20,451,495,44],0)?;
    Ok(())
}

#[cfg(test)] mod detection_gui_tests{
    use super::*;
    #[test] fn hidden_disappearing_candidate_and_input_guards_do_not_start_worker(){
        fn rejected(h:H,state:&mut State,expected:&str){
            assert_eq!(begin_conversion(h,state).unwrap_err(),expected);
            assert!(state.job.is_none());assert!(state.value.is_none());
            assert_ne!(unsafe{IsWindow(h)},0,"invalid input must leave the same window open");
        }
        let folder=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/c2-input-finish")
            .join(format!("input-guards-{}-{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let preferred=folder.join("Blender 4.5/blender.exe");let selected=folder.join("Blender 4.2/blender.exe");
        for path in [&preferred,&selected]{std::fs::create_dir_all(path.parent().unwrap()).unwrap();std::fs::write(path,b"not executed").unwrap();}
        let source=folder.join("scene.blend");let wrong_source=folder.join("scene.txt");
        let output=source.with_extension("camera.vmd");let directory_output=folder.join("directory.vmd");
        std::fs::write(&source,b"unchanged source").unwrap();std::fs::write(&wrong_source,b"wrong extension").unwrap();
        std::fs::create_dir(&directory_output).unwrap();
        for language in [Language::Ja,Language::En]{
            let name=wide(&format!("NexaInputGuardsTest-{}-{}",language.code(),std::process::id()));let instance=unsafe{GetModuleHandleW(null())};
            let class=Class{style:0,proc:Some(procedure),cls:0,wnd:0,instance,icon:null_mut(),cursor:null_mut(),background:16usize as H,menu:null(),name:name.as_ptr()};
            assert_ne!(unsafe{RegisterClassW(&class)},0);
            let h=unsafe{CreateWindowExW(0,name.as_ptr(),name.as_ptr(),0,0,0,760,512,null_mut(),null_mut(),instance,null_mut())};assert!(!h.is_null());
            let mut state=Box::new(State{base:folder.clone(),value:None,language,job:None,cancel_requested:false,blenders:vec![
                crate::blender_detection::Candidate{path:preferred.clone(),version:vec![4,5]},
                crate::blender_detection::Candidate{path:selected.clone(),version:vec![4,2]},
            ]});
            let window=Window{hwnd:h,instance,class:name};unsafe{SetWindowLongPtrW(h,-21,&mut *state as *mut State as isize);}
            populate(h,&state,&source).unwrap();
            unsafe{SendMessageW(GetDlgItem(h,22),0x14e,1,0);SendMessageW(h,0x111,(1<<16)|22,0);}
            assert_eq!(collect(h,&folder).unwrap().blender,selected);
            std::fs::remove_file(&selected).unwrap();
            rejected(h,&mut state,language.text(Text::SelectBlender));
            assert!(preferred.is_file());assert_eq!(text(h,10).unwrap(),selected.display().to_string());
            assert_eq!(unsafe{SendMessageW(GetDlgItem(h,22),0x147,0,0)},1,"must not select a fallback");
            std::fs::write(&selected,b"not executed").unwrap();
            for invalid in [wrong_source.clone(),folder.join("missing.blend")]{
                set_path(h,11,&invalid).unwrap();rejected(h,&mut state,language.text(Text::SelectBlend));
            }
            set_path(h,11,&source).unwrap();
            for invalid in [folder.join("camera.blend"),folder.join("missing/camera.vmd")]{
                set_path(h,12,&invalid).unwrap();rejected(h,&mut state,language.text(Text::SelectOutput));
            }
            set_path(h,12,&directory_output).unwrap();rejected(h,&mut state,language.text(Text::RegularOutput));
            set_path(h,12,&output).unwrap();
            for (value,key) in [("invalid",Text::Number),("NaN",Text::FiniteNumber)]{
                set_path(h,15,Path::new(value)).unwrap();
                rejected(h,&mut state,&language.format(key,&[("name",language.text(Text::Start))]));
            }
            set_path(h,15,Path::new("2")).unwrap();set_path(h,16,Path::new("1")).unwrap();
            rejected(h,&mut state,language.text(Text::StartEnd));
            for id in [15,16]{set_path(h,id,Path::new("")).unwrap();}
            for (id,value,restore,key) in [(17,"0","12.5",Text::PositiveUnits),(18,"0","1920",Text::Dimensions),(19,"16385","1080",Text::Dimensions)]{
                set_path(h,id,Path::new(value)).unwrap();rejected(h,&mut state,language.text(key));
                set_path(h,id,Path::new(restore)).unwrap();
            }
            std::fs::write(&output,b"unchanged output").unwrap();
            rejected(h,&mut state,language.text(Text::OutputExists));
            unsafe{SendMessageW(GetDlgItem(h,21),0xf1,1,0);}
            let options=collect(h,&folder).unwrap();assert!(options.overwrite);
            crate::worker::validate(&options,language).unwrap();
            assert!(state.job.is_none(),"validation alone must not launch a worker");
            assert_eq!(std::fs::read(&source).unwrap(),b"unchanged source");assert_eq!(std::fs::read(&output).unwrap(),b"unchanged output");
            std::fs::remove_file(&output).unwrap();drop(window);
        }
        for path in [&preferred,&selected,&source,&wrong_source]{std::fs::remove_file(path).unwrap();}
        for path in [preferred.parent().unwrap(),selected.parent().unwrap(),directory_output.as_path(),folder.as_path()]{std::fs::remove_dir(path).unwrap();}
    }
    #[test] fn hidden_cancel_completion_preserves_published_success(){
        // Exercise the real private-window callbacks, without running Blender or MMD.
        let folder=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/locale-cancel-race-20261021")
            .join(format!("hidden-cancel-{}-{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&folder).unwrap();
        let temporary=folder.join("camera.tmp");let output=folder.join("camera.vmd");
        let mut file=std::fs::File::create_new(&temporary).unwrap();
        crate::vmd::write_camera_vmd(&mut file,&[crate::vmd::CameraKey{
            frame:0,distance:0.0,center:[0.0;3],rotation:[0.0;3],fov_deg:30,perspective:true,
        }]).unwrap();
        file.sync_all().unwrap();drop(file);std::fs::rename(&temporary,&output).unwrap();
        let published=std::fs::read(&output).unwrap();
        for language in [Language::Ja,Language::En]{
            for outcome in 0..3{
                let name=wide(&format!("NexaCancelCompletionTest-{}-{outcome}",language.code()));
                let instance=unsafe{GetModuleHandleW(null())};
                let class=Class{style:0,proc:Some(procedure),cls:0,wnd:0,instance,icon:null_mut(),cursor:null_mut(),background:16usize as H,menu:null(),name:name.as_ptr()};
                assert_ne!(unsafe{RegisterClassW(&class)},0);
                let h=unsafe{CreateWindowExW(0,name.as_ptr(),name.as_ptr(),0,0,0,760,512,null_mut(),null_mut(),instance,null_mut())};assert!(!h.is_null());
                let report=language.format(Text::Diagnostic,&[("path",&folder.display().to_string())]);
                let options=Options{blender:folder.join("blender.exe"),source:folder.join("scene.blend"),output:output.clone(),
                    scene:Some("Scene".into()),camera:None,start:None,end:None,units_per_meter:12.5,width:1920,height:1080,allow_fov_approximation:false,overwrite:false};
                let cancel=Arc::new(AtomicBool::new(false));
                let result=match outcome{0=>Ok(report.clone()),1=>Err(language.text(Text::Cancelled).into()),_=>Err(language.text(Text::PublishFailed).into())};
                let mut state=Box::new(State{base:folder.clone(),value:None,blenders:vec![],language,cancel_requested:false,
                    job:Some(Job{options,cancel:Arc::clone(&cancel),result:Arc::new(Mutex::new(Some(result)))})});
                let window=Window{hwnd:h,instance,class:name};
                unsafe{SetWindowLongPtrW(h,-21,&mut *state as *mut State as isize);}
                control(h,"STATIC","",23,[20,451,495,44],0).unwrap();
                request_cancel(h,&mut state);
                assert!(state.cancel_requested&&cancel.load(Ordering::Relaxed));
                assert_eq!(text(h,23).unwrap(),language.text(Text::Cancelling));
                assert_ne!(unsafe{IsWindow(h)},0);
                assert_eq!(unsafe{SendMessageW(h,0x113,1,0)},0);
                assert!(state.job.is_none());assert_eq!(unsafe{IsWindow(h)},0);
                if outcome==0{
                    let (returned,returned_report)=state.value.as_ref().expect("published success must survive a late cancellation request");
                    assert_eq!(returned.output,output);assert_eq!(returned.scene.as_deref(),Some("Scene"));assert_eq!(returned_report,&report);
                }else{assert!(state.value.is_none());}
                assert_eq!(std::fs::read(&output).unwrap(),published);
                drop(window);
            }
        }
        std::fs::remove_file(&output).unwrap();std::fs::remove_dir(&folder).unwrap();
    }
    #[test] fn hidden_controls_prefill_switch_and_allow_manual_path(){
        // Only a private, invisible test window; no input injection into user apps.
        let name=wide("NexaBlenderDetectionControlsTest");let instance=unsafe{GetModuleHandleW(null())};
        let class=Class{style:0,proc:Some(procedure),cls:0,wnd:0,instance,icon:null_mut(),cursor:null_mut(),background:16usize as H,menu:null(),name:name.as_ptr()};
        assert_ne!(unsafe{RegisterClassW(&class)},0);
        let h=unsafe{CreateWindowExW(0,name.as_ptr(),name.as_ptr(),0,0,0,760,512,null_mut(),null_mut(),instance,null_mut())};assert!(!h.is_null());
        let mut state=Box::new(State{base:PathBuf::from("C:/"),value:None,language:Language::Ja,job:None,cancel_requested:false,blenders:vec![
            crate::blender_detection::Candidate{path:PathBuf::from("C:/Blender 4.5/blender.exe"),version:vec![4,5]},
            crate::blender_detection::Candidate{path:PathBuf::from("C:/Blender 4.2/blender.exe"),version:vec![4,2]},
        ]});
        let window=Window{hwnd:h,instance,class:name};unsafe{SetWindowLongPtrW(h,-21,&mut *state as *mut State as isize);}
        control(h,"EDIT","",10,[20,38,435,26],0x10080).unwrap();control(h,"COMBOBOX","",22,[465,38,165,220],0x10003).unwrap();
        fill_blenders(h,&state.blenders).unwrap();assert_eq!(text(h,10).unwrap(),"C:/Blender 4.5/blender.exe");
        unsafe{SendMessageW(GetDlgItem(h,22),0x14e,1,0);SendMessageW(h,0x111,(1<<16)|22,0);}
        assert_eq!(text(h,10).unwrap(),"C:/Blender 4.2/blender.exe");
        set_path(h,10,Path::new("C:/Portable/blender.exe")).unwrap();assert_eq!(text(h,10).unwrap(),"C:/Portable/blender.exe");
        unsafe{SendMessageW(GetDlgItem(h,22),0x14b,0,0);}fill_blenders(h,&[]).unwrap();
        assert_eq!(text(h,10).unwrap(),"C:/Portable/blender.exe");
        for language in [Language::Ja,Language::En]{
            state.language=language;
            for id in [10,22]{unsafe{DestroyWindow(GetDlgItem(h,id));}}
            populate(h,&state,Path::new("C:/test.blend")).unwrap();
            assert_eq!(text(h,1).unwrap(),language.text(Text::Convert));
            assert_eq!(text(h,2).unwrap(),language.text(Text::Cancel));
            assert_eq!(text(h,20).unwrap(),language.text(Text::Approximation));
            assert_eq!(text(h,21).unwrap(),language.text(Text::Overwrite));
            assert_eq!(collect(h,Path::new("C:/")).unwrap_err(),language.text(Text::SelectBlender));
            set_busy(h,true);assert_eq!(text(h,23).unwrap(),language.text(Text::Working));set_busy(h,false);
            for id in [1,2,10,11,12,13,14,15,16,17,18,19,20,21,22,23,110,111,112]{unsafe{DestroyWindow(GetDlgItem(h,id));}}
        }
        drop(window);
    }
}
