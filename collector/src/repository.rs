use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::{CreateBucketRequest, DeleteBucketRequest, PutObjectRequest, S3Client, S3};

// struct ObjectKey(String);
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

    pub async fn create(&self) {
        let req = CreateBucketRequest {
            bucket: self.name.clone(),
            ..Default::default()
        };
        self.s3
            .create_bucket(req)
            .await
            .expect("Failed to create S3 bucket");
    }

    pub async fn delete(&self) {
        let req = DeleteBucketRequest {
            bucket: self.name.clone(),
            ..Default::default()
        };
        self.s3
            .delete_bucket(req)
            .await
            .expect("Failed to delete S3 bucket");
    }

    pub async fn put_object(&self, object_key: String, payload: Vec<u8>) {
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
