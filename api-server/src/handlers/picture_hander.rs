use actix_web::{web, HttpResponse, Responder, routes};

use crate::app_state::AppState;
use crate::errors::Error;

#[routes]
#[post("/picture")]
pub async fn post_picture(
    body: web::Bytes,
    data: web::Data<AppState>
) -> Result<impl Responder, Error> {
    let image_data = body.to_vec();

    let status_response = data.picture_service
    .upload_and_register_picture(image_data)
    .await
    .map_err(|_| Error::InternalError(
        "Failed to register or upload picture".to_string()
    ))?;

    Ok(HttpResponse::Ok().json(status_response))
}