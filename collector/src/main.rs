use rusoto_core::Region;
use std::io::{Read, Write};
use tempfile::NamedTempFile;
use uuid::Uuid;

mod repository;

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
    let bucket = repository::Bucket::new(region, access_key, secret_key, "phemex");
    bucket.create().await; // TODO: move this

    let mut tmpfile = NamedTempFile::new()?;
    let mut open_tmpfile = tmpfile.reopen()?;

    let uri = "https://testnet-api.phemex.com/v1/md/ticker/24hr/all";
    let res = reqwest::get(uri).await?;
    let body: String = res.text().await?;
    tmpfile.write_all(body.as_bytes())?;

    let folder = String::from("collector/");
    let uuid = Uuid::new_v4().to_string();
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
        let uuid = String::from("c7ca4c34-178b-4bc1-880b-c81690fcdfda");
        let ext = ".json";
        assert_eq!(
            key(folder, uuid, ext),
            "collector/c7ca4c34-178b-4bc1-880b-c81690fcdfda.json"
        );
    }
    // TODO: e2e test for this function
}
