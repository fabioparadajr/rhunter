use chrono::prelude::*;
use fanotify::high_level::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;


pub fn scriber(event: &Vec<FanEvent>, event_path: &Path) -> Result<(), std::io::Error> {
    let local: DateTime<Local> = Local::now();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    writeln!(
        file,
        "Timestamp: {:?} Path: {:?}, Actions: {:?}",
        local, event_path, event
    )
    .unwrap();
    Ok(())
}
