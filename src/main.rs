
use elf::file;
use fanotify::{low_level::*,  high_level::*};
use hasher::get_file_hash;
use logger::scriber;
use std::{path::PathBuf, sync::Arc};
use regex::Regex;

mod hasher;
mod logger;
fn main() {
/* ###### PERMISSION ##### binary to have similar priviliges as root: sudo setcap cap_sys_admin=eip rhunter

 */ 
let rgx: Regex = Regex::new(r"^(/([a-z0-9_-]+/)*[a-z0-9_-]+)?/$").unwrap();
let path ;
loop {
    println!("Hello, type a path to monitor: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_trim = input.trim();

    if rgx.is_match(&input_trim) {
        path = String::from(input_trim);
        println!("Path: {}", path);

        break;
    } else {
        println!("Ops! You didn't type right.");
    }
}


 
    let primary_path = PathBuf::from(path);
    let ft = Fanotify::new_nonblocking(FanotifyMode::NOTIF).expect("Error regitering fanotify listener");
    let _real_path = ft.add_path(
        FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR,
        &primary_path,
    ).unwrap();

    let mut directories:Vec<PathBuf> = vec![primary_path];
    

    loop {
        let eventos = ft.read_event();
        for events in eventos {
            
            let write = FanEvent::CloseWrite;    
            let file_path = Arc::new(PathBuf::from(&events.path));
            
            scriber(&events.events, &file_path);
            if events.events.contains(&write) && !events.path.contains("swp") && file_path.is_file()
            {

                println!("File changed: {:?}", events.path );
               
                   let hsh =  match get_file_hash(Arc::clone(&file_path))
                   {
                        Ok(hex) => println!("hash: {hex}"),
                        Err(e) => println!("Couldnt hash the file")
                    };

                   
                

            } else if file_path.is_dir() && !directories.contains(&file_path){ // Need to be a dir and not inside the vector.
                /* ################### add new dir #######
              At first, I thought of this as recursive, when the user provides a path and i need to look everything inside and them start monitor
                but I don't need that. 
              Since the events dynamically show any changes in the directories, they will automatically be added to the vector and start monitoring."
                 */  
                println!("New Dir add to monitor {:?}", file_path);
                
                directories.push(Arc::clone(&file_path).to_path_buf());
                ft.add_path(
                    FAN_ACCESS | FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR,
                    Arc::clone(&file_path).as_path(),
                ).unwrap();
                
                
            } 
            
        }

    }

    
    }
