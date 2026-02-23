use crate::app_state::AppState;
use actix_web::web::Data;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct CheckAuthToken;

impl<S, B> Transform<S, ServiceRequest> for CheckAuthToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckAuthTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthTokenMiddleware { service }))
    }
}

pub struct CheckAuthTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckAuthTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let app_state = req.app_data::<Data<AppState>>().cloned();
        let auth_header = req.headers().get("Authorization").cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let state = match app_state {
                Some(s) => s,
                None => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "App state missing",
                    ))
                }
            };

            let token_str = match auth_header {
                Some(h) => h.to_str().unwrap_or("").to_string(),
                None => return Err(actix_web::error::ErrorUnauthorized("No token")),
            };

            let (is_valid, user) = state
                .auth_service
                .verify_token(token_str)
                .await
                .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Auth failed: {}", e)))?;

            if !is_valid {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token or user"));
            }

            let user_exists = state
                .user_service
                .get_by_google_id(user.google_id)
                .await
                .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

            match user_exists {
                Some(_) => {}
                None => return Err(actix_web::error::ErrorUnauthorized("User not found")),
            }

            fut.await
        })
    }
}
