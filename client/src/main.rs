use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{CreateBucketRequest, DeleteBucketRequest, PutObjectRequest, S3Client, S3};

// use serde::{Deserialize, Serialize};
use serde_json::json;

use futures_util::StreamExt;
// use std::collections::HashMap;
use std::io::{self, Write, Read};
use anyhow::anyhow;
use uuid::Uuid;
use tempfile::NamedTempFile;

struct Bucket {
    s3: S3Client,
    name: String,
}

// struct ObjectKey(String);

impl Bucket {
    fn new(region: Region, access_key: String, secret_key: String, name: &str) -> Self {
        Bucket {
            s3: S3Client::new_with(
                HttpClient::new().expect("Failed to create request dispatcher"),
                StaticProvider::new_minimal(access_key, secret_key),
                region,
            ),
            name: name.into(),
        }
    }

    async fn create(&self) {
        let req = CreateBucketRequest {
            bucket: self.name.clone(),
            ..Default::default()
        };
        self.s3
            .create_bucket(req)
            .await
            .expect("Failed to create S3 bucket");
    }

    async fn delete(&self) {
        let req = DeleteBucketRequest {
            bucket: self.name.clone(),
            ..Default::default()
        };
        self.s3
            .delete_bucket(req)
            .await
            .expect("Failed to delete S3 bucket");
    }

    async fn put_object(&self, object_key: String, payload: Vec<u8>) {
        let req = PutObjectRequest {
            bucket: self.name.clone(),
            key: object_key.to_owned(),
            body: Some(payload.into()),
            ..Default::default() 
        };
        let result = self.s3
            .put_object(req)
            .await
            .expect("Couldn't PUT object");
        println!("{:#?}", result);
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:9000".to_owned(),
    };
    let access_key = String::from("AKIAIOSFODNN7EXAMPLE");
    let secret_key = String::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
    let bucket = Bucket::new(region, access_key, secret_key, "phemex");
    bucket.create().await;

    let mut tmpfile = NamedTempFile::new()?;
    let open_tmpfile = tmpfile.reopen()?;

    let uri = "https://testnet-api.phemex.com/v1/md/ticker/24hr/all";
    let res = reqwest::get(uri).await?; 
    let body: String = res.text().await?;
    tmpfile.write_all(body.as_bytes())?;

    fn key() -> String {
        let folder = String::from("/collector/");
        let mut uuid = Uuid::new_v4().to_string();
        let ext = ".json"; 
        folder + &uuid.to_owned() + ext
    }

    let object_key: String = key();

    let full_name = "David";
    let age_last_year: i32 = 99;

    let payload = json!({
        "name": full_name,
        "age": age_last_year + 1
    });

    // let mut f = File::open(local_filename).unwrap();
    //let mut f = open_tmpfile.into_file(); // TODO already a file? https://docs.rs/tempfile/3.1.0/tempfile/struct.NamedTempFile.html#method.reopen
    let mut contents: Vec<u8> = Vec::new();
    match open_tmpfile.read_to_end(&mut contents) {
        //Err(why) => anyhow!("{}", why),
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            bucket.put_object(object_key, open_tmpfile.into_bytes()).await; // need to convert to vec<u8>
        }
    }
    Ok(())
}
