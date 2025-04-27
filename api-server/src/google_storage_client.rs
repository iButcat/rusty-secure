use std::{fs, path::Path};
use google_cloud_storage::client::{ClientConfig, Client};

pub async fn init_google_storage_client() -> Result<Client, ()> {
    let credentials_path = "service-account.json";

    if !Path::new(credentials_path).exists() {
        // TODO: handle error correctly
        println!("Service account not found at: {}", credentials_path);
        return Err(())
    };

    let credentials_json = fs::read_to_string(credentials_path).unwrap();

    let config = ClientConfig::default().with_credentials(
            // TODO: handle error, we can't use "?"
            serde_json::from_str(&credentials_json).unwrap()
    ).await.unwrap();
        
    Ok(Client::new(config))
}