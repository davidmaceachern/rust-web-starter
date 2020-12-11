use rusoto_core::Region;

mod repository;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:9000".to_owned(),
    };
    let access_key = String::from("AKIAIOSFODNN7EXAMPLE");
    let secret_key = String::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
    let bucket = repository::Bucket::new(region, access_key, secret_key, "phemex");
    // TODO: assume bucket already created by collector
    bucket.list_objects().await; // TODO: list objects in bucket before attempting to process
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
