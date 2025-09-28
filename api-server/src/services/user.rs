use std::sync::Arc;
use async_trait::async_trait;

use super::UserService;
use crate::repositories::UserRepository;
use crate::models::User;
use crate::errors::Error;

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>
}

impl UserServiceImpl {
    pub fn new(
        user_repo: Arc<dyn UserRepository>
    ) -> Self {
        Self {
            user_repo
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_by_google_id(&self, google_id: String) -> Result<Option<User>, Error> {
        let user = self.user_repo
            .get_by_google_id(google_id)
            .await?;

        Ok(user)
    }

    // NOTE: This could be also better by using the returned user from db
    async fn register(&self, user: User) -> Result<Option<User>, Error> {
        self.user_repo.insert(&user).await?;

        Ok(Some(user))
    }
}