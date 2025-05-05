use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod models;

mod payloads;

mod app_state;
use app_state::AppState;

mod handlers;
use handlers::{get_status, post_picture, patch_authorised};

mod repositories;
use repositories::{
    GcsRepository, 
    MongoRepository, 
    PictureRepository, 
    StatusRepository, 
    StorageRepository
};

mod mongo_client;
use mongo_client::init_mongo_client;

mod google_storage_client;
use google_storage_client::init_google_storage_client;

mod config;
use config::Config;
use services::{PictureServiceImpl, StatusServiceImpl};

mod errors;

mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::new();

    let database_name = config.database_name;
    let database_name_copy = database_name.clone();

    let mongo_client = match init_mongo_client(
        config.mongodb_url,
        database_name
    ).await {
        Ok(client) => client,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let storage_client = init_google_storage_client(config.credentials_path).await.unwrap();

    let mongo_repo = Arc::new(MongoRepository::new(
        mongo_client, 
        database_name_copy
    ));
    let gcp_repo = Arc::new(GcsRepository::new(
        storage_client, 
        config.bucket_name
    ));

    let status_repository: Arc<dyn StatusRepository> = mongo_repo.clone();
    let picture_repository: Arc<dyn PictureRepository> = mongo_repo.clone();
    let storage_repository: Arc<dyn StorageRepository> = gcp_repo;

    let status_service = Arc::new(StatusServiceImpl::new(
        status_repository.clone(), 
        picture_repository.clone()
    ));
    let picture_service = Arc::new(PictureServiceImpl::new(
        picture_repository, 
        storage_repository, 
        status_service.clone()
    ));
    
    println!("Starting API server on 0.0.0.0:8080");
    
    HttpServer::new(move || {
        let app_state = AppState {
            status_service: status_service.clone(),
            picture_service: picture_service.clone()
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(post_picture)
            .service(get_status)
            .service(patch_authorised)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}