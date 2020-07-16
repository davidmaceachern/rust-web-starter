#![allow(dead_code, unused_macros, unused_variables, unused_mut)]

use async_std::task;
use dotenv;
use kv_log_macro as log;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::path::Path;
// use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use chrono::Utc;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    femme::with_level(femme::LevelFilter::Trace);
    log::info!("Running Application");
    let mut id = get_id();
    let mut secret = get_secret();
    fetch_data(id, secret);
    // write_json();
    update_last_modified();
    Ok(())        
}

fn write_json() {
    todo!()
}

fn fetch_data(id: String, secret: String) -> Result<(), surf::Exception>  {
    println!("id is: {} and secret is  {}", id, secret);
    task::block_on(async {
        let uri = "https://testnet-api.phemex.com/v1/md/ticker/24hr/all";
        let string: String = surf::get(uri).recv_string().await?;
        println!("{}", string);
        Ok::<(), surf::Exception>(())
    })
}

fn get_id() -> String {
    let id = env::var("API_ID").is_err();
    let name = "API_ID";
    let mut val = match env::var(name) {
        Err(why) => panic!("couldn't retrieve: {}", why),
        Ok(val) => val
    };
    val
}

fn get_secret() -> String {
    let secret = env::var("API_SECRET").is_err();
    let name = "API_SECRET";
    let mut val = match env::var(name) {
        Err(why) => panic!("couldn't retrieve: {}", why),
        Ok(val) => val    
    };
    val
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