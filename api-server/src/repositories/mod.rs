mod mongo_repository;
pub use mongo_repository::MongoRepository;

mod gcs_repository;
pub use gcs_repository::GcsRepository;

use bson::Uuid;
use async_trait::async_trait;

use crate::models::{Picture, Status};
use crate::errors::Error;

#[async_trait]
pub trait StatusRepository: Send + Sync {
    async fn find_by_id(&self, id:Uuid) -> Result<Option<Status>, Error>;
    async fn insert(&self, status: &Status) -> Result <(), Error>;
    async fn find_and_update_authorised(&self, id: Uuid,
        authorised: bool) -> Result<Option<Status>, Error>;
}

#[async_trait]
pub trait PictureRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Picture>, Error>;
    async fn insert(&self, picture: &Picture) -> Result<(), Error>;
}

#[async_trait]
pub trait StorageRepository: Send + Sync {
    // Returns an Url, I should make a type for it
    async fn upload_file(&self, name: &str, 
        data: Vec<u8>) -> Result <String, Error>;
}