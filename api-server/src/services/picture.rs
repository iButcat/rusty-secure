use async_trait::async_trait;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::PictureService;
use super::StatusService;
use crate::errors::Error;
use crate::models::Picture;
use crate::payloads::StatusResponse;
use crate::repositories::{PictureRepository, StorageRepository};

pub struct PictureServiceImpl {
    picture_repo: Arc<dyn PictureRepository>,
    storage_repo: Arc<dyn StorageRepository>,
    status_service: Arc<dyn StatusService>,
}

impl PictureServiceImpl {
    pub fn new(
        picture_repo: Arc<dyn PictureRepository>,
        storage_repo: Arc<dyn StorageRepository>,
        status_service: Arc<dyn StatusService>,
    ) -> Self {
        Self {
            picture_repo,
            storage_repo,
            status_service,
        }
    }
}

#[async_trait]
// TODO: Better error handle, got lazy :)
impl PictureService for PictureServiceImpl {
    async fn upload_and_register_picture(
        &self,
        image_data: Vec<u8>,
    ) -> Result<StatusResponse, Error> {
        if !image_data.is_empty() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Error::Service(format!("System time error: {}", e)))?
                .as_secs();

            let object_name = format!("esp32_cam_{}.jpg", timestamp);
            let url = self
                .storage_repo
                .upload_file(&object_name, image_data)
                .await?;

            let new_picture = Picture::new(object_name, url);
            let picture_id = new_picture.id;
            self.picture_repo.insert(&new_picture).await?;

            let status_response = self
                .status_service
                .create_initial_status(picture_id)
                .await?;

            Ok(status_response)
        } else {
            Err(Error::Empty("Image data is empty".to_string()))
        }
    }
}
