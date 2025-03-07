use fanotify::high_level::*;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;
use chrono::prelude::*;

pub fn scriber(event:  &Vec<FanEvent>, event_path: &PathBuf) -> Result<(), std::io::Error>{

    
    let local: DateTime<Local> = Local::now();
    let mut file = OpenOptions::new().append(true).create(true).open("log.txt").unwrap();
    
    writeln!(file, "Timestamp: {:?} Path: {:?}, Actions: {:?}", local, event_path, event).unwrap();
    Ok(())

}