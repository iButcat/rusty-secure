use actix_web::HttpResponse;
use bson::Uuid;
use async_trait::async_trait;
use chrono::DateTime;
use futures_util::TryStreamExt;
use std::sync::Arc;

use super::StatusService;
use crate::repositories::{StatusRepository, PictureRepository};
use crate::models::{Status, Picture};
use crate::payloads::StatusResponse;
use crate::errors::Error;

pub struct StatusServiceImpl {
    status_repo: Arc<dyn StatusRepository>,
    picture_repo: Arc<dyn PictureRepository>,

    http_server_address: String
}

impl StatusServiceImpl {
    pub fn new(
        status_repo: Arc<dyn StatusRepository>, 
        picture_repo: Arc<dyn PictureRepository>,
        http_server_address: String
) -> Self {
        Self {
            status_repo,
            picture_repo,
            http_server_address
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
        updated_status.updated_at = Some(chrono::Local::now());

        let picture = self.find_picture_by_id(updated_status.picture_id).await?;

        Ok(StatusResponse::new(updated_status, picture))
    }

    async fn create_initial_status(&self, picture_id: Uuid) -> Result<StatusResponse, Error> {
        let picture = self.find_picture_by_id(picture_id).await?;

        let status_model = Status::new(picture_id);
        let _initial_status = self.status_repo.insert(&status_model).await?;

        Ok(StatusResponse::new(status_model, picture))
    }

    async fn send_status(&self, status_id: Uuid) -> Result<bool, Error> {
        let status = self.status_repo.find_by_id(status_id)
            .await?
            .ok_or_else(|| Error::NotFound(
                format!("Status not found for ID: {}", status_id)
            ))?;
        let picture = self.find_picture_by_id(status.picture_id).await?;
        
        let status_payload = StatusResponse::new(status, picture);
        
        let client = reqwest::Client::new();
        println!("Sending status to: {}", self.http_server_address);
        
        match client.post("http://192.168.1.40/24:80/authorised")
            .header("Content-Type", "application/json")
            .json(&status_payload)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Successfully sent status to HTTP Server");
                        Ok(true)
                    } else {
                        let status = response.status();
                        let error_msg = format!("HTTP Server responded with error status: {}", status);
                        println!("{}", error_msg);
                        Err(Error::ServiceError(error_msg))
                    }
                },
                Err(err) => {
                    let error_msg = format!("Failed to send status to HTTP Server: {}", err);
                    println!("{}", error_msg);
                    Err(Error::ServiceError(error_msg))
                }
        }
    }
}