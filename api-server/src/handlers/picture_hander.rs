use actix_web::{routes, web, HttpResponse, Responder};
use bson::Uuid;

use crate::app_state::AppState;
use crate::errors::Error;

#[routes]
#[post("/picture/{user_id}")]
pub async fn post_picture(
    path: web::Path<String>,
    body: web::Bytes,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let image_data = body.to_vec();
    let user_id = path.into_inner();
    let user_uuid = Uuid::parse_str(user_id).map_err(|e| Error::Internal(e.to_string()))?;

    let status_response = data
        .picture_service
        .upload_and_register_picture(user_uuid, image_data)
        .await
        .map_err(|_| Error::Internal("Failed to register or upload picture".to_string()))?;

    // NOTE: This is for testing, this request should be used when someone review
    // if the person on the picture is recognised to then authorised and sent it
    let _ok = data
        .status_service
        .send_status(status_response.id)
        .await
        .map_err(|_| "Failed in send to esp 32".to_string());

    Ok(HttpResponse::Ok().json(status_response))
}

#[routes]
#[get("/api/picture/{user_id}")]
pub async fn get_all_picture(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let user_id = path.into_inner();
    let user_uuid = Uuid::parse_str(user_id).map_err(|e| Error::Internal(e.to_string()))?;

    let pictures = data
        .picture_service
        .get_all(user_uuid)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(pictures))
}
