extern crate alloc;

use alloc::format;
use embassy_net::tcp::TcpSocket;
use embassy_net::{IpListenEndpoint, Stack};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Sender;
use embassy_time::{Duration, Timer};
use embedded_io_async::Read as _;
use embedded_io_async::Write as _;
use log::{error, info, warn};
use serde_json_core::from_slice;

use crate::display::DisplayMessage;
use crate::http::AuthUpdatePayload;

const HTTP_PORT: u16 = 80;

#[embassy_executor::task]
pub async fn http_server_task(
    stack: &'static Stack<'static>,
    display_sender: Sender<'static, CriticalSectionRawMutex, DisplayMessage, 2>,
) {
    let mut rx_buffer_socket = [0u8; 1024];
    let mut tx_buffer_socket = [0u8; 1024];

    loop {
        if !stack.is_link_up() {
            warn!("HTTP Server: Link is down, waiting...");
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }

        if stack.config_v4().is_none() {
            warn!("HTTP Server: IP address not configured...");
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }

        let mut socket = TcpSocket::new(*stack, &mut rx_buffer_socket, &mut tx_buffer_socket);
        socket.set_timeout(Some(Duration::from_secs(10)));

        let local_endpoint = IpListenEndpoint {
            addr: None,
            port: HTTP_PORT,
        };
        info!("HTTP Server: Listening on port {}...", HTTP_PORT);

        if let Err(e) = socket.accept(local_endpoint).await {
            error!("HTTP Server: Failed to accept connection {:?}", e);
            socket.abort();
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        let remote_ep = socket.remote_endpoint().unwrap();
        info!("HTTP Server: Accepted connection from: {:?}", remote_ep);

        let mut request_buffer = [0u8; 2048];
        let mut read_len = 0;

        let read_result = embassy_time::with_timeout(Duration::from_secs(10), async {
            match socket.read(&mut request_buffer).await {
                Ok(0) => {
                    info!("HTTP Server: Client disconnected before sending data");
                    Err("ClientDisconnected")
                }
                Ok(n) => {
                    read_len = n;
                    Ok(())
                }
                Err(e) => {
                    error!("HTTP Server: Read error: {:?}", e);
                    Err("ReadError")
                }
            }
        })
        .await;

        if read_result.is_err() {
            warn!(
                "HTTP Server: Failed to read request or timed out. Error: {:?}",
                read_result.unwrap_err()
            );
            socket.abort();
            continue;
        }

        let request_str = match core::str::from_utf8(&request_buffer[..read_len]) {
            Ok(s) => s,
            Err(_) => {
                warn!("HTTP Server: Received non-UTF8 request.");
                let response = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\n";
                if socket.write_all(response.as_bytes()).await.is_err() {
                    warn!("HTTP Server: Failed to send 400 HTTP response.");
                }
                socket.close();
                socket.abort();
                continue;
            }
        };

        info!(
            "HTTP Server: Received request (first line):\n{}",
            request_str.lines().next().unwrap_or("")
        );

        let mut auth_payload: Option<AuthUpdatePayload> = None;
        let mut response_status_code = "400 Bad Request";
        let mut response_body_content = "Bad Request";

        if let Some(body_start_index) = request_str.find("\r\n\r\n") {
            let body_str = &request_str[(body_start_index + 4)..];
            let first_line = request_str.lines().next().unwrap_or("");

            if first_line.starts_with("POST /authorised") {
                if !body_str.is_empty() {
                    match from_slice::<AuthUpdatePayload>(body_str.as_bytes()) {
                        Ok((payload, _consumed)) => {
                            info!(
                                "HTTP Server: Parsed AuthUpdatePayload: ID={}, Authorised={}",
                                payload.id.as_str(),
                                payload.authorised
                            );
                            display_sender
                                .send(DisplayMessage::AuthStatus(payload.authorised))
                                .await;
                            auth_payload = Some(payload);
                            response_status_code = "200 OK";
                            response_body_content = if auth_payload.as_ref().unwrap().authorised {
                                "Authorization status updated to true"
                            } else {
                                "Authorization status updated to false"
                            };
                        }
                        Err(e) => {
                            error!("HTTP Server: JSON parse error: {:?}", e);
                            if body_str.len() > 100 {
                                error!(
                                    "HTTP Server: Failed to parse body: {}...",
                                    &body_str[..100]
                                );
                            } else {
                                error!("HTTP Server: Failed to parse body: {}", body_str);
                            }
                            response_status_code = "400 Bad Request";
                            response_body_content = "Invalid JSON payload";
                        }
                    }
                } else {
                    warn!(
                        "HTTP Server: /authorised endpoint called (POST) but no JSON body found."
                    );
                    response_status_code = "400 Bad Request";
                    response_body_content = "Missing JSON body for /authorised";
                }
            } else {
                warn!(
                    "HTTP Server: Unrecognized request path or method: {}",
                    first_line
                );
                response_status_code = "404 Not Found";
                response_body_content = "Endpoint not found or method not allowed";
            }
        } else {
            warn!("HTTP Server: Malformed request (no CRLFCRLF found).");
        }

        let http_response = format!(
            "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            response_status_code,
            response_body_content.len(),
            response_body_content
        );

        if let Err(e) = socket.write_all(http_response.as_bytes()).await {
            error!("HTTP Server: Failed to send response: {:?}", e);
        }

        socket.close();
        socket.abort();
        info!("HTTP Server: Connection closed.");
        Timer::after(Duration::from_millis(100)).await;
    }
}
