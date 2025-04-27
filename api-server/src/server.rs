use actix_web::{web, App, HttpServer};
use google_cloud_storage::client::Client as StorageClient;
use mongodb::Client as MongoClient;
use std::sync::Mutex;
use std::collections::HashMap;

mod handlers;
use handlers::post_picture;

mod models;
use models::{Picture, Authorisation};

mod mongo_client;
use mongo_client::init_mongo_client;

mod google_storage_client;
use google_storage_client::init_google_storage_client;

struct AppState {
    mongo_client: MongoClient,
    storage_client: StorageClient,

    // TODO: check if those are indeed necessary
    latest_image: Mutex<Option<Vec<u8>>>,
    analysis_results: Mutex<HashMap<String, bool>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = match init_mongo_client().await {
        Ok(client) => client,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let storage_client = init_google_storage_client().await.unwrap();

    let app_state = web::Data::new(AppState {
        mongo_client: mongo_client,
        storage_client: storage_client,
        latest_image: Mutex::new(None),
        analysis_results: Mutex::new(HashMap::new()),
    });
    
    println!("Starting API server on 0.0.0.0:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::PayloadConfig::new(1 * 1024 * 1024))
            .service(post_picture)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}