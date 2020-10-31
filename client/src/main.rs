use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{CreateBucketRequest, DeleteBucketRequest, S3Client, S3};

use anyhow::anyhow;

struct Bucket {
    s3: S3Client,
    name: String,
}

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
    bucket.delete().await;
    Ok(())
}
