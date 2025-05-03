use actix_web::{web, HttpResponse, Error};
use google_cloud_storage::http::objects::upload::{
    Media, 
    UploadObjectRequest, 
    UploadType
};
use bson::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::routes;
use mongodb::bson::doc;

use crate::payloads::StatusResponse;
use crate::{payloads, AppState};
use crate::models::{Picture, Status};

static GOOGLE_STORAGE_BASE_URL: &str = "https://storage.cloud.google.com";
static GOOGLE_STORAGE_BASE_PATH: &str = "/uploads/";

#[routes]
#[post("/picture")]
pub async fn post_picture(
    body: web::Bytes,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let image_data = body.to_vec();

    if !image_data.is_empty() {

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let object_name = format!("esp32_cam{}.jpg", timestamp);
        let object_name_copy = object_name.clone();

        let mut media = Media::new(format!("{}/{}", "uploads".to_string(), object_name));
        media.content_type = "image/jpeg".into();
        media.content_length = Some(image_data.len() as u64);

        let object = match data.storage_client.upload_object(
            &UploadObjectRequest{
                bucket: data.config.bucket_name.to_string(),
                ..Default::default()
            },
            image_data,
            &UploadType::Simple(media)
        ).await {
            Ok(obj) => {
                println!("Successfully uploaded to GCS: {}", obj.name);
                obj
            },
            Err(e) => {
                eprintln!("Failed to upload to Storage: {}", e);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Failed to upload image to Storage: {}", e)
                })));
            }
        };

        let url = format!(
            "{}{}{}", 
            GOOGLE_STORAGE_BASE_URL.to_string(),
            GOOGLE_STORAGE_BASE_PATH,
            object_name
        );
        let new_picture = Picture::new(object_name_copy, url);
        let picture_id = new_picture.id;
        let new_picture_clone = new_picture.clone();

        let _ = match data.mongo_client
            .database(&data.config.database_name)
            .collection::<Picture>("pictures")
            .insert_one(new_picture).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error while saving picture: {}", e)
                }
            };

        let new_status = Status::new(picture_id);
        let new_status_clone = new_status.clone();

        let _ = match data.mongo_client
            .database(&data.config.database_name)
            .collection::<Status>("statuses").insert_one(new_status).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error while saving picture: {}", e)
                }
            };
        
        let status_response = payloads::StatusResponse::new(
            new_status_clone, new_picture_clone);

        return Ok(HttpResponse::Ok().json(serde_json::json!(status_response)));
    }
    
    Ok(HttpResponse::BadRequest().body("No image data received"))
}

#[routes]
#[get("/status/{id}")]
pub async fn status(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let status_id = path.into_inner();
    let status_uuid = match Uuid::parse_str(&status_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid status ID format. Expected UUID"
            })));
        }
    };

    let status_result = data.mongo_client
        .database(&data.config.database_name)
        .collection::<Status>("statuses")
        .find_one(doc! {"_id": status_uuid}).await;
    
    match status_result {
        Ok(Some(status_doc)) => {
            let picture_result = data.mongo_client
                .database(&data.config.database_name)
                .collection::<Picture>("pictures")
                .find_one(doc! {"_id": status_doc.picture_id}).await;
            
            match picture_result {
                Ok(Some(picture_doc)) => {
                    let status_response = StatusResponse::new(status_doc, picture_doc);
                    Ok(HttpResponse::Ok().json(status_response))
                },
                Ok(None) => {
                    Ok(HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Associated picture not found"
                    })))
                },
                Err(e) => {
                    eprintln!("Database error fetching picture: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Error retrieving associated picture"
                    })))
                }
            }
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Status not found"
            })))
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error occurred"
            })))
        }
    }
}
