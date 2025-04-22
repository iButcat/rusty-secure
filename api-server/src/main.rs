use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use std::io::Write;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;

struct AppState {
    latest_image: Mutex<Option<Vec<u8>>>,
    analysis_results: Mutex<HashMap<String, bool>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("ESP32-CAM API Server")
}

async fn post_image(
    body: web::Bytes,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let image_data = body.to_vec();

    if !image_data.is_empty() {
        println!("Received image data: {} bytes", image_data.len());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let filename = format!("uploads/esp32cam_{}.jpg", timestamp);

        if !Path::new("uploads").exists() {
            if let Err(e) = fs::create_dir("uploads") {
                eprintln!("Failed to create uploads directory: {}", e);
            }
        }

        match std::fs::File::create(&filename) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&image_data) {
                    eprintln!("Failed to write image file {}: {}", filename, e);
                } else {
                    println!("Saved image to {}", filename);
                }
            },
            Err(e) => {
                 eprintln!("Failed to create image file {}: {}", filename, e);
            }
        }

        let mut latest_image = data.latest_image.lock().unwrap();
        *latest_image = Some(image_data);
        
        let analysis_result = true;
        
        let mut results = data.analysis_results.lock().unwrap();
        results.insert(timestamp.to_string(), analysis_result);
        
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "authorized": true,
            "user_id": "90130495867",
            "user_first_name": "John",
            "user_last_name": "Doe",
            "reason": "Image received and processed"
        })));
    }
    
    Ok(HttpResponse::BadRequest().body("No image data received"))
}

async fn get_status(data: web::Data<AppState>) -> impl Responder {
    let latest_image = data.latest_image.lock().unwrap();
    let results = data.analysis_results.lock().unwrap();
    
    let has_image = latest_image.is_some();
    let latest_result = if !results.is_empty() {
        let latest_key = results.keys().max().cloned().unwrap_or_default();
        results.get(&latest_key).cloned().unwrap_or(false)
    } else {
        false
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "has_image": has_image,
        "latest_result": latest_result,
        "total_images": results.len()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        latest_image: Mutex::new(None),
        analysis_results: Mutex::new(HashMap::new()),
    });
    
    println!("Starting API server on 0.0.0.0:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::PayloadConfig::new(1 * 1024 * 1024))
            .route("/", web::get().to(index))
            .route("/analyse", web::post().to(post_image))
            .route("/status", web::get().to(get_status))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}