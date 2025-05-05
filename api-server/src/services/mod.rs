mod status;
pub use status::StatusServiceImpl;

mod picture;
pub use picture::PictureServiceImpl;

use async_trait::async_trait;
use bson::Uuid;

use crate::errors::Error;
use crate::payloads::StatusResponse;

#[async_trait]
pub trait StatusService: Send + Sync {
    async fn get_status_details(&self, status_id: Uuid) -> Result<StatusResponse, Error>;
    async fn update_authorisation(&self, status_id: Uuid, 
        authorised: bool) -> Result<StatusResponse, Error>;
    async fn create_initial_status(&self, 
        picture_id: Uuid) -> Result<StatusResponse, Error>;
}

#[async_trait]
pub trait PictureService: Send + Sync {
    async fn upload_and_register_picture(&self, 
        image_data: Vec<u8>) -> Result<StatusResponse, Error>;
}