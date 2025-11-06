use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::{net::UdpSocket, sync::Arc};

mod models;

mod payloads;

mod app_state;
use app_state::AppState;

mod handlers;
use handlers::{get_status, patch_authorised, post_picture};

mod repositories;
use repositories::{
    GcsRepository, MongoRepository, PictureRepository, StatusRepository, StorageRepository,
};

mod mongo_client;
use mongo_client::init_mongo_client;

mod google_storage_client;
use google_storage_client::init_google_storage_client;

mod config;
use config::Config;
use services::{PictureServiceImpl, StatusServiceImpl};

use crate::{
    handlers::{auth_url, callback, get_by_google_id},
    repositories::UserRepository,
    services::{GoogleAuthServiceImpl, UserServiceImpl},
};

mod errors;

mod services;

fn get_local_ip() -> Result<String, Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_address = socket.local_addr()?;
    Ok(local_address.ip().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = get_local_ip().unwrap();
    println!("local ip: {}", local_ip);

    dotenv().ok();

    let config = Config::new();

    let database_name = config.database_name;
    let database_name_copy = database_name.clone();

    let mongo_client = match init_mongo_client(config.mongodb_url, database_name).await {
        Ok(client) => client,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let storage_client = init_google_storage_client(config.credentials_path)
        .await
        .unwrap();

    let mongo_repo = Arc::new(MongoRepository::new(mongo_client, database_name_copy));
    let gcp_repo = Arc::new(GcsRepository::new(storage_client, config.bucket_name));

    let status_repository: Arc<dyn StatusRepository> = mongo_repo.clone();
    let picture_repository: Arc<dyn PictureRepository> = mongo_repo.clone();
    let user_repository: Arc<dyn UserRepository> = mongo_repo.clone();
    let storage_repository: Arc<dyn StorageRepository> = gcp_repo;

    let status_service = Arc::new(StatusServiceImpl::new(
        status_repository.clone(),
        picture_repository.clone(),
        config.http_server_address,
    ));
    let picture_service = Arc::new(PictureServiceImpl::new(
        picture_repository,
        storage_repository,
        status_service.clone(),
    ));
    let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
    let goolge_auth_service = Arc::new(GoogleAuthServiceImpl::new(
        config.google_auth_client_id,
        config.google_auth_client_secret,
        config.google_auth_redirect_url,
        config.google_auth_scope,
    ));

    println!("Starting API server on 0.0.0.0:8080");

    HttpServer::new(move || {
        let app_state = AppState {
            status_service: status_service.clone(),
            picture_service: picture_service.clone(),
            user_service: user_service.clone(),
            goolge_auth_service: goolge_auth_service.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(post_picture)
            .service(get_status)
            .service(patch_authorised)
            .service(auth_url)
            .service(callback)
            .service(get_by_google_id)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
