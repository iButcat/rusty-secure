mod status;
pub use status::StatusServiceImpl;

mod picture;
pub use picture::PictureServiceImpl;

mod auth;
pub use auth::AuthServiceImpl;

mod user;
pub use user::UserServiceImpl;

use async_trait::async_trait;
use bson::Uuid;

use crate::errors::Error;
use crate::models::{Device, Picture, Token, User};
use crate::payloads::{StatusResponse, UserInfo};

// NOTE: Service should return a model then the API layer convert to payload..
#[async_trait]
pub trait StatusService: Send + Sync {
    async fn get_status_details(&self, status_id: Uuid) -> Result<StatusResponse, Error>;
    async fn update_authorisation(
        &self,
        status_id: Uuid,
        authorised: bool,
    ) -> Result<StatusResponse, Error>;
    async fn create_initial_status(&self, picture_id: Uuid) -> Result<StatusResponse, Error>;
    async fn send_status(&self, status_id: Uuid) -> Result<bool, Error>;
}

#[async_trait]
pub trait PictureService: Send + Sync {
    async fn upload_and_register_picture(
        &self,
        user_id: Uuid,
        image_data: Vec<u8>,
    ) -> Result<StatusResponse, Error>;
    async fn get_all(&self, user_id: Uuid) -> Result<Vec<Picture>, Error>;
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn get_authorisation_url(
        &self,
        response_type: Option<String>,
    ) -> Result<(String, String), Error>;
    async fn exchange_code_for_token(
        &self,
        code: String,
        state: String,
    ) -> Result<(UserInfo, Token), Error>;
    async fn verify_token(&self, token: String) -> Result<(bool, User), Error>;
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_by_google_id(&self, google_id: String) -> Result<Option<User>, Error>;
    async fn register(&self, user: User) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait DeviceService: Send + Sync {
    async fn register(&self, device: Device) -> Result<Option<Device>, Error>;
}
