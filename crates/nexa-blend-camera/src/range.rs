//! Bounded, one-process camera-range evaluation.
use super::{CameraSample, DecodeError, MAX_PACKET_BYTES, Reader, decode};
use crate::i18n::{Language,Text};
use std::{sync::atomic::{AtomicBool,Ordering},time::{Duration,Instant}};
use std::{fmt, fs::{self, File}, io::Read, path::Path, process::{Child,Command,ExitStatus, Stdio}};

pub const MAX_RANGE_SAMPLES: usize = 108_001;
pub const MAX_RANGE_BYTES: usize = 128 * 1024 * 1024;
const COMMON: &str = include_str!("../../../tools/blender_camera_export.py");
const RANGE: &str = include_str!("../../../tools/blender_camera_range_export.py");

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RangeRequest<'a> {
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub scene: Option<&'a str>,
    pub camera: Option<&'a str>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CameraRange { pub samples: Vec<CameraSample>, pub input_fps: f64 }
#[derive(Debug)]
pub enum RangeError {
    Invalid(Text),
    Container(DecodeError),
    Packet { sample: usize, source: DecodeError },
    Io(std::io::Error),
    BlenderFailed(Option<i32>),
    Cancelled,
    TimedOut,
}
impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(s) => f.write_str(Language::En.text(*s)),
            Self::Container(e) => write!(f, "range container: {e}"),
            Self::Packet {sample,source} => write!(f, "camera sample {sample}: {source}"),
            Self::Io(e) => write!(f,"camera range I/O: {e}"),
            Self::Cancelled => f.write_str(Language::En.text(Text::Cancelled)),
            Self::TimedOut => f.write_str(Language::En.text(Text::Timeout)),
            Self::BlenderFailed(code) => write!(f,"Blender range evaluation failed: {code:?}"),
        }
    }
}
impl std::error::Error for RangeError {}
impl From<std::io::Error> for RangeError { fn from(e:std::io::Error)->Self {Self::Io(e)} }
struct OwnedChild(Child);
impl Drop for OwnedChild {fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}

fn wait_controlled(child:&mut Child,cancelled:&AtomicBool,timeout:Duration)->Result<ExitStatus,RangeError>{
    let started=Instant::now();
    loop {
        let stopped=if cancelled.load(Ordering::Relaxed) {Some(RangeError::Cancelled)}
            else if started.elapsed()>=timeout {Some(RangeError::TimedOut)} else {None};
        if let Some(error)=stopped {
            // Own only this direct child, and reap it before returning.
            child.kill()?; child.wait()?; return Err(error);
        }
        if let Some(status)=child.try_wait()? {return Ok(status);}
        std::thread::park_timeout(Duration::from_millis(50));
    }
}

pub fn decode_range(data: &[u8]) -> Result<CameraRange, RangeError> {
    if data.len() > MAX_RANGE_BYTES { return Err(RangeError::Invalid(Text::RangeTooLarge)); }
    let mut r = Reader { data, offset: 0 };
    if r.take(8).map_err(RangeError::Container)? != b"NEXARAN1" {
        return Err(RangeError::Invalid(Text::RangeMagic));
    }
    let count = r.u32().map_err(RangeError::Container)? as usize;
    if !(1..=MAX_RANGE_SAMPLES).contains(&count) { return Err(RangeError::Invalid(Text::SampleCount)); }
    // No allocation from a count that cannot fit even packet-size prefixes.
    if count > data.len().saturating_sub(r.offset) / 4 {
        return Err(RangeError::Invalid(Text::RangeTruncated));
    }
    let mut samples: Vec<CameraSample> = Vec::with_capacity(count);
    for sample in 0..count {
        let size = r.u32().map_err(RangeError::Container)? as usize;
        if !(1..=MAX_PACKET_BYTES).contains(&size) {return Err(RangeError::Invalid(Text::SampleSize));}
        let packet = r.take(size).map_err(RangeError::Container)?;
        let next = decode(packet).map_err(|source| RangeError::Packet {sample,source})?;
        if let Some(first) = samples.first() {
            if first.scene_name != next.scene_name || first.view_layer_name != next.view_layer_name
                || first.fps != next.fps || first.fps_base != next.fps_base {
                return Err(RangeError::Invalid(Text::SceneChanged));
            }
        }
        samples.push(next);
    }
    if r.offset != data.len() {return Err(RangeError::Invalid(Text::RangeTrailing));}
    let input_fps = samples[0].fps as f64 / samples[0].fps_base;
    if !input_fps.is_finite() || input_fps <= 0.0 {return Err(RangeError::Invalid(Text::InputFps));}
    Ok(CameraRange {samples,input_fps})
}
pub fn read_range(path: &Path) -> Result<CameraRange, RangeError> {
    let mut bytes=Vec::new();
    File::open(path)?.take(MAX_RANGE_BYTES as u64 + 1).read_to_end(&mut bytes)?;
    decode_range(&bytes)
}
fn literal(text:&str) -> String {
    let mut out=String::from("'");
    for c in text.chars() {
        match c { '\\' => out.push_str("\\\\"), '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"), '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('\''); out
}
pub fn import_range(blender:&Path, source:&Path, output:&Path, request:RangeRequest<'_>)
    -> Result<CameraRange,RangeError>
{
    import_range_controlled(blender,source,output,request,None)
}
pub fn import_range_controlled(blender:&Path, source:&Path, output:&Path, request:RangeRequest<'_>, control:Option<&AtomicBool>)
    -> Result<CameraRange,RangeError>
{
    for frame in [request.start,request.end].into_iter().flatten() {
        if !frame.is_finite() || !(-1_048_574.0..=1_048_574.0).contains(&frame) {
            return Err(RangeError::Invalid(Text::FrameLimits));
        }
    }
    if let (Some(start),Some(end))=(request.start,request.end) {
        if start>end {return Err(RangeError::Invalid(Text::StartEnd));}
    }
    match fs::symlink_metadata(output) {
        Ok(_) => return Err(RangeError::Invalid(Text::RangeExists)),
        Err(e) if e.kind()==std::io::ErrorKind::NotFound => {},
        Err(e) => return Err(e.into()),
    }
    let expression=format!("n={{'__name__':'nexa_camera_common'}};exec({},n);n['__name__']='__main__';exec({},n)",literal(COMMON),literal(RANGE));
    let mut command=Command::new(blender);
    command.args(["--background","--factory-startup","--disable-autoexec","--python-exit-code","1","--python-expr"])
        .arg(expression).args(["--","--input"]).arg(source).arg("--output").arg(output);
    if let Some(start)=request.start {command.arg("--start").arg(start.to_string());}
    if let Some(end)=request.end {command.arg("--end").arg(end.to_string());}
    if let Some(scene)=request.scene {command.arg("--scene").arg(scene);}
    if let Some(camera)=request.camera {command.arg("--camera").arg(camera);}
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::inherit());
    let status=if let Some(cancelled)=control {
        let mut child=OwnedChild(command.spawn()?);
        wait_controlled(&mut child.0,cancelled,Duration::from_secs(600))?
    } else {command.status()?};
    if !status.success() {return Err(RangeError::BlenderFailed(status.code()));}
    read_range(output)
}

#[cfg(test)]
mod process_tests {
    use super::*;
    use std::io::{BufRead,BufReader,Write};
    use std::sync::mpsc;

    #[test]
    #[ignore = "only launched as a subprocess by the process contract tests"]
    fn controlled_child_wait_target(){
        assert_eq!(std::env::var("NEXA_RANGE_PROCESS_TEST").as_deref(),Ok("1"));
        println!("nexa-range-child-ready");
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs(30));
        panic!("process contract test did not stop its child");
    }

    fn running_child()->OwnedChild{
        let mut command=Command::new(std::env::current_exe().unwrap());
        command.args(["--ignored","--exact","range::process_tests::controlled_child_wait_target","--nocapture"])
            .env("NEXA_RANGE_PROCESS_TEST","1")
            .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
        let mut child=OwnedChild(command.spawn().unwrap());
        let output=BufReader::new(child.0.stdout.take().unwrap());
        let (ready_sender,ready_receiver)=mpsc::channel();
        let reader=std::thread::spawn(move ||{
            for line in output.lines(){
                if line.unwrap().ends_with("nexa-range-child-ready"){
                    let _=ready_sender.send(());break;
                }
            }
        });
        ready_receiver.recv_timeout(Duration::from_secs(5)).expect("child did not become ready");
        reader.join().unwrap();
        assert!(child.0.try_wait().unwrap().is_none(),"test child already exited");
        child
    }

    fn assert_stopped(child:&mut OwnedChild){
        // Check before OwnedChild drops so cleanup cannot hide a still-running child.
        let status=child.0.try_wait().unwrap().expect("controlled child is still running");
        assert!(!status.success(),"controlled child unexpectedly succeeded");
        assert_eq!(child.0.wait().unwrap(),status);
    }

    #[test]
    fn cancelled_child_is_killed_and_reaped(){
        let mut child=running_child();
        let cancelled=AtomicBool::new(false);
        cancelled.store(true,Ordering::Relaxed);
        let started=Instant::now();
        assert!(matches!(wait_controlled(&mut child.0,&cancelled,Duration::from_secs(600)),Err(RangeError::Cancelled)));
        assert!(started.elapsed()<Duration::from_secs(5),"cancellation did not promptly stop the child");
        assert_stopped(&mut child);
    }

    #[test]
    fn timed_out_child_is_killed_and_reaped(){
        let mut child=running_child();
        let cancelled=AtomicBool::new(false);
        let timeout=Duration::from_millis(100);
        let started=Instant::now();
        assert!(matches!(wait_controlled(&mut child.0,&cancelled,timeout),Err(RangeError::TimedOut)));
        assert!(started.elapsed()>=timeout,"timeout elapsed too early");
        assert!(started.elapsed()<Duration::from_secs(5),"timeout did not promptly stop the child");
        assert_stopped(&mut child);
    }
}
