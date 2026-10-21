//! Standalone camera conversion pipeline; no solver or MMD memory access.
use crate::{convert::{ConvertOptions,convert_sample},range::{RangeRequest,RangeError,import_range_controlled},vmd::write_camera_vmd};
use crate::{DecodeError,i18n::{Language,Text,external}};
use std::sync::atomic::{AtomicBool,Ordering};
use std::{fs::{self,OpenOptions},io::Write,path::PathBuf,time::{SystemTime,UNIX_EPOCH}};
#[derive(Clone,Debug)]
pub struct Options {
    pub blender:PathBuf,pub source:PathBuf,pub output:PathBuf,
    pub scene:Option<String>,pub camera:Option<String>,pub start:Option<f64>,pub end:Option<f64>,
    pub units_per_meter:f64,pub width:u32,pub height:u32,pub allow_fov_approximation:bool,pub overwrite:bool,
}
fn inspect_output(options:&Options,language:Language)->Result<(),String>{
    match fs::symlink_metadata(&options.output) {
        Ok(m) if m.file_type().is_symlink() || !m.is_file()=>Err(language.text(Text::RegularOutput).into()),
        Ok(_) if !options.overwrite=>Err(language.text(Text::OutputExists).into()),
        Ok(_)=>Ok(()),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(()),Err(e)=>Err(external(language,Text::Io,e)),
    }
}
pub fn convert(options:&Options)->Result<String,String>{convert_localized(options,Language::En,None)}
fn check_cancel(language:Language,cancelled:Option<&AtomicBool>)->Result<(),String>{
    if cancelled.is_some_and(|c|c.load(Ordering::Relaxed)){Err(language.text(Text::Cancelled).into())}else{Ok(())}
}
pub fn convert_localized(options:&Options,language:Language,cancelled:Option<&AtomicBool>)->Result<String,String>{
    check_cancel(language,cancelled)?;
    validate(options,language)?;
    convert_validated(options,language,cancelled)
}
pub fn validate(options:&Options,language:Language)->Result<(),String>{
    if !options.blender.is_file() || !options.source.is_file(){return Err(language.text(Text::ExistingInputs).into());}
    if !options.source.extension().and_then(|s|s.to_str()).is_some_and(|s|s.eq_ignore_ascii_case("blend")){
        return Err(language.text(Text::BlendExtension).into());
    }
    if !options.output.extension().and_then(|s|s.to_str()).is_some_and(|s|s.eq_ignore_ascii_case("vmd")){
        return Err(language.text(Text::VmdExtension).into());
    }
    if !options.units_per_meter.is_finite() || options.units_per_meter<=0.0
        || !(1..=16384).contains(&options.width)||!(1..=16384).contains(&options.height){
        return Err(language.text(Text::UnitsDimensions).into());
    }
    inspect_output(options,language)?;
    Ok(())
}
fn convert_validated(options:&Options,language:Language,cancelled:Option<&AtomicBool>)->Result<String,String>{
    let nonce=SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|external(language,Text::Io,e))?.as_nanos();
    let folder=std::env::temp_dir().join(format!("nexa-camera-{}-{nonce}",std::process::id()));
    fs::create_dir(&folder).map_err(|e|external(language,Text::Io,e))?;
    let packet=folder.join("range.nexacam");
    let range=import_range_controlled(&options.blender,&options.source,&packet,RangeRequest {
        start:options.start,end:options.end,scene:options.scene.as_deref(),camera:options.camera.as_deref(),
    },cancelled).map_err(|e|format!("{}\n{}",range_error(language,&e),language.format(Text::Diagnostic,&[("path",&folder.display().to_string())])))?;
    let mut previous=None;let mut keys=Vec::with_capacity(range.samples.len());
    let mut max_error=[0.0f64;2];let mut approximated=0;let mut cuts=0;
    let unit_scale=range.samples[0].unit_scale;
    for (index,sample) in range.samples.iter().enumerate(){
        check_cancel(language,cancelled)?;
        if sample.unit_scale!=unit_scale{return Err(language.text(Text::AnimatedUnits).into());}
        if index>0&&sample.camera_name!=range.samples[index-1].camera_name{cuts+=1;}
        let c=convert_sample(sample,index as u32,ConvertOptions {units_per_meter:options.units_per_meter,
            viewport:[options.width,options.height],allow_fov_approximation:options.allow_fov_approximation},previous)
            .map_err(|e|language.format(Text::FrameError,&[("frame",&index.to_string()),("camera",&format!("{:?}",sample.camera_name)),("detail",language.text(e.0))]))?;
        previous=Some(c.rotation);keys.push(c.key);
        if c.approximated {approximated+=1;}
        for (i,error) in c.edge_error_px.into_iter().enumerate(){max_error[i]=max_error[i].max(error);}
    }
    // Validate/serialize before creating an output file.
    let mut data=Vec::new();write_camera_vmd(&mut data,&keys).map_err(|e|if e.kind()==std::io::ErrorKind::InvalidInput{language.text(Text::VmdKeys).into()}else{external(language,Text::Io,e)})?;
    let parent=options.output.parent().filter(|p|!p.as_os_str().is_empty()).unwrap_or(std::path::Path::new("."));
    let temporary=parent.join(format!(".nexa-camera-{}-{nonce}.tmp",std::process::id()));
    let mut file=OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(|e|external(language,Text::Io,e))?;
    file.write_all(&data).and_then(|_|file.sync_all()).map_err(|e|language.format(Text::WriteFailed,&[("path",&temporary.display().to_string()),("detail",&e.to_string())]))?;
    drop(file);
    check_cancel(language,cancelled)?;
    inspect_output(options,language)?;
    #[cfg(windows)] {
        use std::os::windows::ffi::OsStrExt;
        #[link(name="kernel32")] unsafe extern "system" {fn MoveFileExW(a:*const u16,b:*const u16,flags:u32)->i32;}
        let a:Vec<_>=temporary.as_os_str().encode_wide().chain(Some(0)).collect();
        let b:Vec<_>=options.output.as_os_str().encode_wide().chain(Some(0)).collect();
        if unsafe{MoveFileExW(a.as_ptr(),b.as_ptr(),8|if options.overwrite{1}else{0})}==0{return Err(external(language,Text::PublishFailed,std::io::Error::last_os_error()));}
    }
    #[cfg(not(windows))] {
        if !options.overwrite {fs::hard_link(&temporary,&options.output).map_err(|e|external(language,Text::Io,e))?;fs::remove_file(&temporary).map_err(|e|external(language,Text::Io,e))?;}
        else {fs::rename(&temporary,&options.output).map_err(|e|external(language,Text::Io,e))?;}
    }
    let first=&range.samples[0];let last=range.samples.last().unwrap();
    let source_duration=((last.frame-first.frame) as f64+last.subframe-first.subframe)/range.input_fps;
    let extension=(keys.len()-1) as f64/30.0-source_duration;
    Ok(language.format(Text::Report,&[
        ("output",&options.output.display().to_string()),("keys",&keys.len().to_string()),("cuts",&cuts.to_string()),
        ("width",&options.width.to_string()),("height",&options.height.to_string()),("units",&options.units_per_meter.to_string()),
        ("approximated",&approximated.to_string()),("error_x",&format!("{:.4}",max_error[0])),("error_y",&format!("{:.4}",max_error[1])),
        ("extension",&format!("{extension:.6}")),("diagnostic",&folder.display().to_string())]))
}
fn packet_error(language:Language,error:&DecodeError)->String{
    match error {
        DecodeError::TooLarge{size}=>language.format(Text::PacketLarge,&[("size",&size.to_string()),("limit",&crate::MAX_PACKET_BYTES.to_string())]),
        DecodeError::InvalidMagic=>language.text(Text::PacketMagic).into(),
        DecodeError::Truncated{offset,needed,remaining}=>language.format(Text::PacketTruncated,&[("offset",&offset.to_string()),("needed",&needed.to_string()),("remaining",&remaining.to_string())]),
        DecodeError::InvalidUtf8{offset,..}=>language.format(Text::PacketUtf8,&[("offset",&offset.to_string())]),
        DecodeError::InvalidEnum{offset,value}=>language.format(Text::PacketEnum,&[("offset",&offset.to_string()),("value",&value.to_string())]),
        DecodeError::InvalidValue{field}=>language.format(Text::PacketValue,&[("field",field)]),
        DecodeError::TrailingBytes{offset,remaining}=>language.format(Text::PacketTrailing,&[("offset",&offset.to_string()),("remaining",&remaining.to_string())]),
    }
}
fn range_error(language:Language,error:&RangeError)->String{
    match error {
        RangeError::Invalid(key)=>language.text(*key).into(),
        RangeError::Container(e)=>external(language,Text::Packet,packet_error(language,e)),
        RangeError::Packet{sample,source}=>language.format(Text::SampleError,&[("sample",&sample.to_string()),("detail",&packet_error(language,source))]),
        RangeError::Io(e)=>external(language,Text::Io,e),
        RangeError::BlenderFailed(code)=>language.format(Text::BlenderFailed,&[("code",&format!("{code:?}"))]),
        RangeError::Cancelled=>language.text(Text::Cancelled).into(),
        RangeError::TimedOut=>language.text(Text::Timeout).into(),
    }
}
