
use bson::Uuid;
use async_trait::async_trait;
use std::sync::Arc;

use super::StatusService;
use crate::repositories::{StatusRepository, PictureRepository};
use crate::models::{Status, Picture};
use crate::payloads::StatusResponse;
use crate::errors::Error;

pub struct StatusServiceImpl {
    status_repo: Arc<dyn StatusRepository>,
    picture_repo: Arc<dyn PictureRepository>
}

impl StatusServiceImpl {
    pub fn new(status_repo: Arc<dyn StatusRepository>, 
        picture_repo: Arc<dyn PictureRepository>) -> Self {
        Self {
            status_repo,
            picture_repo
        }
    }

    async fn find_picture_by_id(&self, id: Uuid) -> Result<Picture, Error> {
        self.picture_repo.find_by_id(id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Status not found for ID: {}", id)))
    }
}

#[async_trait]
impl StatusService for StatusServiceImpl {
    async fn get_status_details(&self, status_id: Uuid) -> Result<StatusResponse, Error> {
        let status = self.status_repo.find_by_id(status_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Status not found for ID: {}", status_id)))?;

        let picture = self.find_picture_by_id(status.picture_id).await?;
        Ok(StatusResponse::new(status, picture))
    }

    async fn update_authorisation(&self, status_id: Uuid, authorised: bool) -> Result<StatusResponse, Error> {
        // The error handling might be weaker here since it could be another error than not found
        let mut updated_status = self.status_repo
        .find_and_update_authorised(status_id, authorised)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Status not found for ID: {}", status_id)))?;

        updated_status.authorised = authorised;

        let picture = self.find_picture_by_id(updated_status.picture_id).await?;

        Ok(StatusResponse::new(updated_status, picture))
    }

    async fn create_initial_status(&self, picture_id: Uuid) -> Result<StatusResponse, Error> {
        let picture = self.find_picture_by_id(picture_id).await?;

        let status_model = Status::new(picture_id);
        let _initial_status = self.status_repo.insert(&status_model).await?;

        Ok(StatusResponse::new(status_model, picture))
    }
}