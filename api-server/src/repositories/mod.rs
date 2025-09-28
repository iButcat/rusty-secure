mod mongo_repository;
pub use mongo_repository::MongoRepository;

mod gcs_repository;
pub use gcs_repository::GcsRepository;

use bson::Uuid;
use async_trait::async_trait;

use crate::models::{Picture, Status, User};
use crate::errors::Error;

#[async_trait]
pub trait StatusRepository: Send + Sync {
    async fn find_by_id(&self, id:Uuid) -> Result<Option<Status>, Error>;
    // NOTE: We could be using the result of insert operation and return the model.
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
    async fn upload_file(&self, name: &str, 
        data: Vec<u8>) -> Result <String, Error>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: &User) -> Result<(), Error>;
    async fn get_by_google_id(&self, google_id: String) -> Result<Option<User>, Error>;
}