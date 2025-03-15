use monitor::monitor_dir;
use std::{path::PathBuf, sync::Arc};
use regex::Regex;

mod monitor;
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


    monitor_dir(path);

    
    }
