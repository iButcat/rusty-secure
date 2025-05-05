use google_cloud_storage::http::objects::upload::{
    Media, 
    UploadObjectRequest, 
    UploadType
};
use google_cloud_storage::client::Client;
use async_trait::async_trait;

use super::StorageRepository;
use crate::errors::Error;

static GOOGLE_STORAGE_BASE_URL: &str = "https://storage.cloud.google.com";
static GOOGLE_STORAGE_BASE_PATH: &str = "uploads";
static CONTENT_TYPE: &str = "image/jpeg";

pub struct GcsRepository {
    client: Client,
    bucket_name: String
}

impl GcsRepository {
    pub fn new(client: Client, bucket_name: String) -> Self {
        Self {
            client,
            bucket_name
        }
    }
}

#[async_trait]
impl StorageRepository for GcsRepository {
    async fn upload_file(&self, name: &str, data: Vec<u8>) -> Result<String, Error> {
        let object_path = format!("{}{}", GOOGLE_STORAGE_BASE_PATH.trim_start_matches('/'), name);

        let mut media = Media::new(object_path.clone());
        media.content_type = CONTENT_TYPE.into();
        media.content_length = Some(data.len() as u64);

        let object =  self.client.upload_object(
            &UploadObjectRequest{
                bucket: self.bucket_name.to_string(),
                ..Default::default()
            },
            data,
            &UploadType::Simple(media)
        ).await.map_err(|e| Error::StorageError(e.to_string()))?;

        // NOTE: remove logs and keep only for errors
        println!("Successfully uploaded to GSC: {}", object.name);

        Ok(format!(
            "{}/{}/{}/{}", 
            GOOGLE_STORAGE_BASE_URL,
            self.bucket_name,
            GOOGLE_STORAGE_BASE_PATH,
            object.name
        ))
    }
}