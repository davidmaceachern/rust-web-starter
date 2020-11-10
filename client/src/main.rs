use anyhow::anyhow;
use futures_util::StreamExt;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{CreateBucketRequest, DeleteBucketRequest, PutObjectRequest, S3Client, S3};
use serde_json::json;
use std::io::{self, Read, Write};
use tempfile::NamedTempFile;
use uuid::Uuid;

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
        let result = self.s3.put_object(req).await.expect("Couldn't PUT object");
        println!("{:#?}", result);
    }
}

fn key(folder: String, uuid: String, ext: &str) -> String {
    folder + &uuid.to_owned() + ext
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
    // bucket.create().await; # TODO: move this

    let mut tmpfile = NamedTempFile::new()?;
    let mut open_tmpfile = tmpfile.reopen()?;

    let uri = "https://testnet-api.phemex.com/v1/md/ticker/24hr/all";
    let res = reqwest::get(uri).await?;
    let body: String = res.text().await?;
    tmpfile.write_all(body.as_bytes())?;

    let folder = String::from("collector/");
    let mut uuid = Uuid::new_v4().to_string();
    let ext = ".json";
    let object_key: String = key(folder, uuid, ext);

    let mut contents: Vec<u8> = Vec::new();
    match open_tmpfile.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            bucket.put_object(object_key, contents).await;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_key() {
        let folder = String::from("collector/");
        let mut uuid = String::from("c7ca4c34-178b-4bc1-880b-c81690fcdfda");
        let ext = ".json";
        assert_eq!(
            key(folder, uuid, ext),
            "collector/c7ca4c34-178b-4bc1-880b-c81690fcdfda.json"
        );
    }
    // TODO: e2e test for this function
}
