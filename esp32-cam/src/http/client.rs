use embedded_svc::{
    http::client::Client as HttpClientTrait,
    io::Write,
};
use esp_idf_svc::http::client::EspHttpConnection;
use anyhow::{Result, Context, anyhow};
use log::{info, error};

use crate::http::AnalysisResponse;

pub struct CameraHttpClient {
    client: HttpClientTrait<EspHttpConnection>,
    api_url: &'static str,
}

impl CameraHttpClient {
    pub fn new(wrapped_client: HttpClientTrait<EspHttpConnection>, api_url: &str) -> Result<Self> {
        Ok(Self { client: wrapped_client, api_url })
    }

    pub fn analyse_image(&mut self, image_data: &[u8]) -> Result<AnalysisResponse, anyhow::Error> {
        let headers = [
            ("accept", "application/json"),
            ("Content-Type", "image/jpeg")
        ];

        info!("Sending image ({} bytes) to {}", image_data.len(), self.api_url);

        let mut request = self.client.post(self.api_url, &headers)
            .context("Client: Failed to create POST request")?;

        request.write_all(image_data)
            .context("Client: Failed to write image data to request")?;

        info!("Client: Submitting request...");
        let mut response = request.submit()
            .context("Client: Failed to submit request")?;

        let status = response.status();
        info!("Client: Response status: {}", status);

        let mut body_bytes = Vec::new();
        let mut buf = [0u8; 1024];
        loop {
            match response.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => body_bytes.extend_from_slice(&buf[..n]),
                Err(e) => return Err(anyhow!("Client: Failed to read response body: {}", e)),
            }
        }

        info!("Client: Read {} bytes from response body.", body_bytes.len());

        if body_bytes.is_empty() {
            return Err(anyhow!("Client: Empty response body"));
        }

        let analysis_response: AnalysisResponse = serde_json::from_slice(&body_bytes)
            .context("Client: Failed to parse response body as AnalysisResponse")?;

        info!("Client: Analysis response: {:?}", analysis_response);
        
        Ok(analysis_response)
    }
}