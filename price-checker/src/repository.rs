use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{ListObjectsV2Request, S3Client, S3};

pub struct Bucket {
    s3: S3Client,
    name: String,
}

impl Bucket {
    pub fn new(region: Region, access_key: String, secret_key: String, name: &str) -> Self {
        Bucket {
            s3: S3Client::new_with(
                HttpClient::new().expect("Failed to create request dispatcher"),
                StaticProvider::new_minimal(access_key, secret_key),
                region,
            ),
            name: name.into(),
        }
    }

    pub async fn list_objects(&self) {
        let req = ListObjectsV2Request {
            bucket: self.name.clone(),
            ..ListObjectsV2Request::default()
        };
        let result = self
            .s3
            .list_objects_v2(req)
            .await
            .expect("Couldn't list objects");
        if result.key_count > Some(0) {
            // TODO: We can add a metric about attempting to process objects
            println!(
                "There are {:#?} objects in the bucket, we can do some analysis",
                Some(result.key_count)
            )
        } else {
            println!("No objects in the bucket, nothing to do here.")
        }
        println!("{:#?}", result);
    }
}
