use actix_web::{routes, web, HttpResponse, Responder};

use crate::app_state::AppState;
use crate::errors::Error;
use crate::payloads::UserResponse;

#[routes]
#[get("/api/user/{id}")]
pub async fn get_by_google_id(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let google_id: String = path.into_inner();
    let user = data
        .user_service
        .get_by_google_id(google_id)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if !user.is_none() {
        let u = user.unwrap();

        let user_response = UserResponse::new(u);

        Ok(HttpResponse::Ok().json(user_response))
    } else {
        Ok(HttpResponse::NotFound().json("Not found"))
    }
}
