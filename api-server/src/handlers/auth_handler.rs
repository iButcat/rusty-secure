use actix_web::{web, HttpResponse, Responder, routes};

use crate::app_state::AppState;
use crate::errors::Error;
use crate::models::User;
use crate::payloads::{AuthResponse, OAuthCallback};

#[routes]
#[get("/api/auth/callback")]
pub async fn callback(
    query: web::Query<OAuthCallback>,
    data: web::Data<AppState>
) -> Result<impl Responder, Error> {
    let callback_data = query.into_inner();
    let state= callback_data.state;
    let code = callback_data.code;
    println!("State: {}, code: {}", state, code);
    let (user_info, token) = data.goolge_auth_service
        .exchange_code_for_token(code, state)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let existing_user = data.user_service
        .get_by_google_id(user_info.id.clone())
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let _user = match existing_user {
        Some(user) => {
            user
        },
        None => {
            let user_model = User::new(
                user_info.id.clone(),
                user_info.email.clone(),
                user_info.name.clone(),
                user_info.picture.clone()
            );
            data.user_service.register(user_model.clone())
                .await
                .map_err(|e| Error::InternalError(e.to_string()))?;
            user_model
        }
    };

    let response = AuthResponse{
        user_info,
        token,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[routes]
#[get("/api/auth/url")]
pub async fn auth_url(
    data: web::Data<AppState>
) -> Result<impl Responder, Error> {
    let auth_url = data.goolge_auth_service
        .get_authorisation_url()
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(auth_url))
}