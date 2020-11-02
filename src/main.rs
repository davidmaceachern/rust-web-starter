#![allow(dead_code, unused_macros, unused_variables, unused_mut)]

use async_std::task;
use dotenv;
use kv_log_macro as log;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
// use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use chrono::Utc;

#[derive(Serialize, Deserialize)]
struct PhemexResponse {
    askEp: i32,
    bidEp: i32,
    fundingRateEr: i32,
    highEp: i32,
    indexEp: i32,
    lastEp: i32,
    lowEp: i32,
    markEp: i32,
    openEp: i32,
    openInterest: i32,
    predFundingRateEr: i32,
    symbol: String,
    timestamp: i32,
    turnoverEv: i32,
    volume: i32,
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    femme::with_level(femme::LevelFilter::Trace);
    log::info!("Running Application");
    let mut id = get_id();
    let mut secret = get_secret();
    let data = fetch_data(id, secret);
    write_json(&data);
    update_last_modified();
    Ok(())
}

fn write_json(data: &str) -> bool {
    let content = serde_json::to_string(data).unwrap();
    let directory = String::from("../tmp/");
    let dt = Utc::now();
    let file_name = format!("{:?}", dt) + ".json";
    let location = directory + &file_name;
    let path = Path::new(&location);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {
            log::info!("successfully wrote to {}", display);
            true
        }
    }
}

fn fetch_data(id: String, secret: String) -> String {
    println!("id is: {} and secret is  {}", id, secret);
    let mut string = String::new();
    task::block_on(async {
        let uri = "https://testnet-api.phemex.com/v1/md/ticker/24hr/all";
        string = surf::get(uri).recv_string().await?;
        println!("{}", string);
        Ok::<(), surf::Exception>(())
    });
    string
}

fn get_id() -> String {
    let id = env::var("API_ID").is_err();
    let name = "API_ID";
    let mut val = match env::var(name) {
        Err(why) => panic!("couldn't retrieve: {}", why),
        Ok(val) => val,
    };
    val
}

fn get_secret() -> String {
    let secret = env::var("API_SECRET").is_err();
    let name = "API_SECRET";
    let mut val = match env::var(name) {
        Err(why) => panic!("couldn't retrieve: {}", why),
        Ok(val) => val,
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

fn check_last_modified() -> bool {
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
    true
}
#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_json() {
        let data: &str = r#"
            { "error": null,
              "id": 0,
              "result": [
        {
            "askEp": 426800,
            "bidEp": 426300,
            "fundingRateEr": 10000,
            "highEp": 429200,
            "indexEp": 426585,
            "lastEp": 426300,
            "lowEp": 417200,
            "markEp": 426605,
            "openEp": 420500,
            "openInterest": 10973,
            "predFundingRateEr": 10000,
            "symbol": "LTCUSD",
            "timestamp": 1595103169787216117,
            "turnoverEv": 13159209790,
            "volume": 1555142
            },
            {
            "askEp": 18121000,
            "bidEp": 18065000,
            "fundingRateEr": 10000,
            "highEp": 18121000,
            "indexEp": 18101870,
            "lastEp": 18121000,
            "lowEp": 18065000,
            "markEp": 18102726,
            "openEp": 18099000,
            "openInterest": 182039,
            "predFundingRateEr": 10000,
            "symbol": "GOLDUSD",
            "timestamp": 1595103169787268293,
            "turnoverEv": 152464051,
            "volume": 8426
            }
        ]
        }"#;
        assert_eq!(write_json(data), true);
    }
    #[test]
    fn test_check_last_modified() {
        assert_eq!(check_last_modified(), true);
    }
}

// let r: PhemexResponse = serde_json::from_str(data)?;
// let writer = &File::create("/tmp/phemex-response.json");
//let w = serde_json::to_writer(&File::create("/tmp/phemex-response.json"), data);
//fs::write("/tmp/phemex-response.json", r).expect("Unable to write file");
// serde_json::ser::to_writer(&mut writer, &data)?;
