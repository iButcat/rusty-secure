extern crate alloc;

use log::{info, error};
use reqwless::{
    client::HttpClient as ReqwlessHttpClient,
    request::Method,
    response::StatusCode,
};
use embedded_nal_async::{TcpConnect, Dns};
use crate::http::CamAuthResponse;
use serde_json_core::from_slice;

#[derive(Debug)]
pub enum ClientError {
    RequestCreationFailed,
    SendFailed,
    StatusError(StatusCode),
    BodyReadFailed,
    JsonParseFailed,
}

pub struct HttpClient<'a, T: TcpConnect, D: Dns> {
    client: ReqwlessHttpClient<'a, T, D>,
    cam_capture_url: &'static str,
}

impl<'a, T: TcpConnect, D: Dns> HttpClient<'a, T, D> {
    pub fn new(tcp: &'a T, dns: &'a D, cam_capture_url: &'static str) -> Self {
        Self {
            client: ReqwlessHttpClient::new(tcp, dns),
            cam_capture_url,
        }
    }

    pub async fn request_camera_capture(&mut self) -> Result<CamAuthResponse, ClientError> {
        info!("Requesting camera capture");
        let url = self.cam_capture_url; 

        let mut request = self.client
            .request(Method::GET, &url)
            .await
            .map_err(|e| {
                error!("Failed to create request: {:?}", e);
                ClientError::RequestCreationFailed
            })?;

        let mut rx_buf = [0u8; 1024];
        let response = request.send(&mut rx_buf).await.map_err(|e| {
            error!("Failed to send request: {:?}", e);
            ClientError::SendFailed
        })?;

        if !response.status.is_successful() {
            error!("Request failed with status: {:?}", response.status);
            return Err(ClientError::StatusError(response.status));
        }

        let body = match response.body().read_to_end().await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to read response body: {:?}", e);
                return Err(ClientError::BodyReadFailed);
            }
        };

        match from_slice::<CamAuthResponse>(body) {
            Ok((auth_response, _)) => {
                info!("Received auth response: {:?}", auth_response);
                Ok(auth_response)
            }
            Err(e) => {
                error!("Failed to parse JSON response: {:?}", e);
                if let Ok(body_str) = core::str::from_utf8(body) {
                    error!("Raw response body: {}", body_str);
                } else {
                    error!("Raw response body (not UTF-8): {:?}", body);
                }
                Err(ClientError::JsonParseFailed)
            }
        }
    }
}