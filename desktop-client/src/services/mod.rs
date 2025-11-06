pub mod api;
pub use api::RustySecureApiImpl;

use crate::errors::Error;
use crate::models::User;

pub trait RustySecureApi {
    async fn get_auth_url(&self) -> Result<(String, String), Error>;
    async fn get_user_by_google_id(&self, id: String) -> Result<Option<User>, Error>;
}