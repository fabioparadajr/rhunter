use crate::hasher;
use crate::logger;
use fanotify::{high_level::*, low_level::*};
use std::{path::PathBuf, sync::Arc};

pub fn monitor_dir(path: String) -> String {
    let primary_path = PathBuf::from(path);
    let ft =
        Fanotify::new_nonblocking(FanotifyMode::NOTIF).expect("Error regitering fanotify listener");
    /*let _real_path = ft.add_path(
        FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR | FAN_CREATE,
        &primary_path,
    ).unwrap();
     */

    let real_path = ft.add_mountpoint(
        FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR,
        &primary_path,
    );
    //let mut directories:Vec<PathBuf> = vec![primary_path];

    let path = primary_path.to_str().unwrap();

    loop {
        let eventos = ft.read_event();
        for events in eventos {
            let write = FanEvent::CloseWrite;
            let file_path = Arc::new(PathBuf::from(&events.path));

            if events.path.contains(path) {
                logger::scriber(&events.events, &file_path);
                if events.events.contains(&write)
                    && !events.path.contains("swp")
                    && file_path.is_file()
                {
                    println!("File changed: {:?}", events.path);

                    let hsh = match hasher::get_file_hash(Arc::clone(&file_path)) {
                        Ok(hex) => println!("hash: {hex}"),
                        Err(e) => println!("Couldnt hash the file"),
                    };
                }
            }
        }
    }
}
