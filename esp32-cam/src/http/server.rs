use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use embassy_sync::blocking_mutex::Mutex as BlockingMutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use esp_idf_hal::gpio::{PinDriver, Gpio4, Output};
use esp_idf_hal::delay::FreeRtos;
use embedded_svc::http::client::Client as HttpClientTrait;
use esp_idf_svc::http::client::{EspHttpConnection, Configuration as HttpConfig};
use esp_idf_svc::http::server::{EspHttpServer, Configuration};
use esp_idf_svc::http::Method;
use embedded_svc::io::Write;
use log::{info, error};
use anyhow::{Result, Context};
use serde_json;

use crate::cam::camera_controller::CameraController;
use crate::http::client::CameraHttpClient;
use crate::http::StatusResponse;

type SharedFlashPin<'a> = Arc<StdMutex<PinDriver<'a, Gpio4, Output>>>;
type SharedCamera<'a> = Arc<BlockingMutex<CriticalSectionRawMutex, CameraController<'a>>>;

pub struct CameraHttpServer<'a> {
    server: EspHttpServer<'a>,
}

impl<'a> CameraHttpServer<'a> {
    pub fn new(
        camera: SharedCamera<'static>,
        flash_led: SharedFlashPin<'static>,
        api_url: &str
    ) -> Result<Self> {
        let server_configuration = Configuration {
            stack_size: 10240,
            ..Default::default()
        };

        let mut server = EspHttpServer::new(&server_configuration)?;
        
        let api_url_owned = api_url.to_string();

        server.fn_handler::<anyhow::Error, _>("/capture", Method::Get, move |req| {
            info!("Received capture request");

            let flash_on_result = match flash_led.lock() {
                Ok(mut guard) => guard.set_high(),
                Err(poisoned) => {
                    error!("Flash mutex poisoned on lock for ON: {}", poisoned);
                    Err(esp_idf_hal::sys::EspError::from_infallible::<-1>())
                }
            };
            if let Err(e) = flash_on_result {
                error!("Failed to turn flash ON: {}", e);
            } else {    
                info!("Flash LED turned ON");
                FreeRtos::delay_ms(500);
                // TODO remove useless logs
                info!("Flash LED turned OFF");
            }
            
            let image_data: Option<Vec<u8>> = camera.lock(|cam_controller| {
                cam_controller.capture()
            });

            let flash_off_result = match flash_led.lock() {
                Ok(mut guard) => guard.set_low(),
                Err(poisoned) => {
                     error!("Flash mutex poisoned on lock for OFF: {}", poisoned);
                     Err(esp_idf_hal::sys::EspError::from_infallible::<-1>())
                }
            };
            if let Err(e) = flash_off_result {
                error!("Failed to turn flash OFF: {}", e);
            } else {
                info!("Flash LED turned OFF");
            }
            
            match image_data {
                Some(data) => {
                    info!("Image captured, size: {} bytes", data.len());

                    let status_result: Result<StatusResponse, anyhow::Error> = {
                        let http_config = HttpConfig::default();
                        let connection = EspHttpConnection::new(&http_config)
                            .context("Handler: Failed create HTTP connection")?;
                        let http_client = HttpClientTrait::wrap(connection);

                        let mut camera_client = CameraHttpClient::new(
                            http_client, api_url_owned.clone())
                            .context("Handler: Failed create CameraHttpClient")?;

                        info!("Calling post_picture...");
                        camera_client.post_picture(&data)
                    };


                    let (_status_code, response_body_str) = match status_result {
                        Ok(status_response) => {
                            let response_body_str = serde_json::to_string(&status_response).unwrap();
                            (200, response_body_str)
                        }
                        Err(e) => {
                            error!("Image analysis failed: {:?}", e);
                            (500, "Image analysis failed".to_string())
                        }
                    };


                    let mut resp = match req.into_response(
                        200, Some("OK"), &[("Content-Type", "application/json")]) {
                        Ok(resp) => resp,
                        Err(e) => { error!("Resp ERR: {}", e); return Err(e.into()); }
                    };
                    
                    if let Err(e) = resp.write_all(response_body_str.as_bytes()) {
                        error!("Write ERR: {}", e); return Err(e.into());
                    }
                    
                    Ok(())
                }
                None => {
                    error!("Failed to capture image");
                    let mut resp = req.into_status_response(500)?;
                    resp.write(b"Failed to capture image")?;
                    
                    Ok(())
                }
            }
        })?;
        info!("HTTP server initialized");
        Ok(Self { server })
    }
    
    pub fn add_rotate_endpoint(&mut self) -> Result<()> {
        self.server.fn_handler::<anyhow::Error, _>("/rotate", Method::Get, move |req| {
            info!("Received rotate request");
            let mut resp = req.into_ok_response()?;
            resp.write(b"Camera rotated")?;
            
            Ok(())
        })?;
        
        Ok(())
    }
}
