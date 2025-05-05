use std::{fs, path::Path};
use google_cloud_storage::client::{ClientConfig, Client};

use crate::errors::Error;

pub async fn init_google_storage_client(path: String) -> Result<Client,  Error> {
    if !Path::new(&path).exists() {
        println!("Service account not found at: {}", path);
        return Err(Error::WithText("Service account not found".into()))
    };

    let credentials_json = fs::read_to_string(path).unwrap();

    let _ = match ClientConfig::default().with_credentials(
            serde_json::from_str(&credentials_json).unwrap()
    ).await {
        Ok(config) => return Ok(Client::new(config)),
        Err(e) => return Err(Error::WithText(e.to_string()))
    };
}