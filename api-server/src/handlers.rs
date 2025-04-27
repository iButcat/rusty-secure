use actix_web::{web, HttpResponse, Error};
use google_cloud_storage::http::buckets::list::ListBucketsRequest;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use actix_web::routes;

use crate::AppState;

#[routes]
#[post("/picture")]
pub async fn post_picture(
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

        let buckets = data.storage_client.list_buckets(&ListBucketsRequest{
            project: "munimentum".into(),
            max_results: None,
            prefix: None,
            page_token: None,
            projection: None,
            match_glob: None
        }).await.unwrap();

        // TODO: delete this after testing
        println!("bucket worked with name: {}", buckets.items[0].name);

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