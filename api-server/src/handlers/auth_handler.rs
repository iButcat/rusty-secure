use actix_web::{routes, web, HttpResponse, Responder};

use crate::app_state::AppState;
use crate::errors::Error;
use crate::models::User;
use crate::payloads::{AuthResponse, OAuthCallback};

use serde::Deserialize;

#[routes]
#[get("/api/auth/callback")]
pub async fn callback(
    query: web::Query<OAuthCallback>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let callback_data = query.into_inner();
    let state = callback_data.state.clone();
    let code = callback_data.code;
    println!("State: {}, code: {}", state, code);
    let (user_info, token) = data
        .goolge_auth_service
        .exchange_code_for_token(code, state)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let existing_user = data
        .user_service
        .get_by_google_id(user_info.id.clone())
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let _user = match existing_user {
        Some(user) => user,
        None => {
            let user_model = User::new(
                user_info.id.clone(),
                user_info.email.clone(),
                user_info.name.clone(),
                user_info.picture.clone(),
            );
            data.user_service
                .register(user_model.clone())
                .await
                .map_err(|e| Error::Internal(e.to_string()))?;
            user_model
        }
    };

    let response = AuthResponse { user_info, token };
    let response_type = if callback_data.state.contains(':') {
        callback_data.state.split(':').nth(1).unwrap_or("json")
    } else {
        "json"
    };

    // We only take care of html or always return json payload
    if response_type == "html" {
        let html_body = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head><title>Login Successful</title></head>
        <body>
            <h1>Login Successful!</h1>
            <textarea id="json">{}</textarea>
            <button onclick="copy()">Copy Token</button>
            <script>function copy() {{ document.getElementById('json').select(); document.execCommand('copy'); }}</script>
        </body>
        </html>
        "#,
            serde_json::to_string(&response).unwrap()
        );
        Ok(HttpResponse::Ok().content_type("text/html").body(html_body))
    } else {
        Ok(HttpResponse::Ok().json(response))
    }
}

#[derive(Deserialize)]
pub struct AuthUrlQuery {
    pub response_type: Option<String>,
}

#[routes]
#[get("/api/auth/url")]
pub async fn auth_url(
    query: web::Query<AuthUrlQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let (url, state) = data
        .goolge_auth_service
        .get_authorisation_url(query.response_type.clone())
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json((url, state)))
}
