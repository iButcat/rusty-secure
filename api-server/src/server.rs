use actix_web::{web, App, HttpServer};
use google_cloud_storage::client::Client as StorageClient;
use mongodb::Client as MongoClient;
use dotenv::dotenv;

mod handlers;
use handlers::{post_picture, status};

mod models;

mod payloads;

mod mongo_client;
use mongo_client::init_mongo_client;

mod google_storage_client;
use google_storage_client::init_google_storage_client;

mod config;
use config::Config;

mod error;

struct AppState {
    mongo_client: MongoClient,
    storage_client: StorageClient,

    config: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::new();
    let config_clone = config.clone();

    let mongo_client = match init_mongo_client(
        config.mongodb_url,
        config.database_name
    ).await {
        Ok(client) => client,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let storage_client = init_google_storage_client(config.credentials_path).await.unwrap();

    let app_state = web::Data::new(AppState {
        mongo_client: mongo_client,
        storage_client: storage_client,
        config: config_clone,
    });
    
    println!("Starting API server on 0.0.0.0:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::PayloadConfig::new(1 * 1024 * 1024))
            .service(post_picture)
            .service(status)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}