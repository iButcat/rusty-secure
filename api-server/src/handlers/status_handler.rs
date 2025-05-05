use actix_web::{patch, routes, web, HttpResponse, Responder};
use bson::Uuid;

use crate::app_state::AppState;
use crate::payloads::{StatusResponse, AuthorisedPatchRequest};
use crate::errors::Error;

#[routes]
#[get("/status/{id}")]
pub async fn get_status(
    path: web::Path::<String>,
    data: web::Data<AppState>
) -> Result<impl Responder, Error> {
    let status_id = path.into_inner();
    let status_uuid = Uuid::parse_str(&status_id)
        .map_err(|_| Error::UuidFormatError("Invalid status ID format".to_string()))?;

    let status_response = data.status_service
    .get_status_details(status_uuid)
    .await.map_err(|_| Error::NotFound(
        format!("Status not found for ID: {}", status_uuid.to_string())
    ))?;

    Ok(HttpResponse::Ok().json(status_response))
}

#[routes]
#[patch("/status/{id}")]
pub async fn patch_authorised(
    body: web::Json<AuthorisedPatchRequest>,
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let status_id = path.into_inner();
    let status_uuid = Uuid::parse_str(&status_id)
        .map_err(|_| Error::UuidFormatError(
            "Invalid status ID format".to_string()
    ))?;

    let authorised = body.authorised;

    let status_response = data.status_service.update_authorisation(status_uuid, authorised)
    .await
    .map_err(|_| Error::InternalError(
        "Something went wrong trying to update status authorisation".to_string()
    ))?;

    Ok(HttpResponse::Ok().json(status_response))
}