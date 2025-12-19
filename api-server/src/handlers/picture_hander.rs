use actix_web::{routes, web, HttpResponse, Responder};

use crate::app_state::AppState;
use crate::errors::Error;

#[routes]
#[post("/picture")]
pub async fn post_picture(
    body: web::Bytes,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let image_data = body.to_vec();

    let status_response = data
        .picture_service
        .upload_and_register_picture(image_data)
        .await
        .map_err(|_| Error::Internal("Failed to register or upload picture".to_string()))?;

    // This is for testing, this request should be used when someone review if
    // the person on the picture is recognised to then authorised and sent it
    let _ok = data
        .status_service
        .send_status(status_response.id)
        .await
        .map_err(|_| "Failed in send to esp 32".to_string());

    Ok(HttpResponse::Ok().json(status_response))
}
