#![allow(dead_code, unused_macros, unused_variables, unused_mut)]
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use chrono::Utc;

fn main() -> std::io::Result<()> {
    update_last_modified();
    Ok(())        
}

fn update_last_modified() {
    let path = Path::new("../tmp/last_modified.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    
    match file.write_all(Utc::now().to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn check_last_modified() {
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => s
    // }
    // let dt = DateTime::parse_from_str(
    //     s, "%Y-%m-%d %H:%M:%S%z");
    todo!();
}