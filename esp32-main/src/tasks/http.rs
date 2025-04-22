use embassy_time::{Duration, Timer};
use log::{info, error};
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::dns::DnsSocket;
use static_cell::StaticCell;

use crate::http::CamAuthResponse;
use crate::http::HttpMessage;
use crate::http::client::{HttpClient, ClientError};

static TCP_CLIENT_STATE: StaticCell<TcpClientState<2, 1024, 1024>> = StaticCell::new();

#[embassy_executor::task(pool_size = 8)]
pub async fn http_camera_task(
    stack: &'static embassy_net::Stack<'static>,
    receiver: Receiver<'static, CriticalSectionRawMutex, HttpMessage, 1>,
    sender: Sender<'static, CriticalSectionRawMutex, HttpMessage, 1>,
    config: &'static crate::config::Config
) {
    Timer::after(Duration::from_secs(2)).await;

    info!("Initializing HTTP camera task");

    info!("HTTP camera task waiting for network stack...");
    loop {
        if stack.is_config_up() {
            info!("Network stack is up!");
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
    
    let state: &mut TcpClientState<2, 1024, 1024> = 
        TCP_CLIENT_STATE.init(TcpClientState::new());
    let tcp_client = TcpClient::new(*stack, state);
    let dns = DnsSocket::new(*stack);
    let mut client = 
        HttpClient::new(&tcp_client, &dns, config.cam_capture_url);

    info!("HTTP camera task initialized and network is up");

    loop {
        info!("Waiting for request message");
        match receiver.receive().await {
            HttpMessage::RequestCapture => {
                info!("Received RequestCapture message");
                match client.request_camera_capture().await {
                    Ok(auth_response) => {
                        info!("Received auth response: {:?}", auth_response);
                        sender.send(HttpMessage::AuthResult(auth_response)).await;
                    },
                    Err(e) => {
                        error!("Request attempt failed: {:?}", e);
                        let failure_reason = match e {
                                ClientError::RequestCreationFailed => "Request Failed",
                                ClientError::SendFailed => "Send Failed",
                                ClientError::StatusError(_) => "Status Error",
                                ClientError::BodyReadFailed => "Body Read Error",
                                ClientError::JsonParseFailed => "JSON Parse Error",
                            };
                            
                            let user_id = heapless::String::<64>::new();
                            let user_first_name = heapless::String::<64>::new();
                            let user_last_name = heapless::String::<64>::new();
                            let mut reason = heapless::String::<128>::new();
                            
                            let _ = reason.push_str(failure_reason);
                            
                            sender.send(HttpMessage::AuthResult(CamAuthResponse {
                                authorized: false,
                                user_id,
                                user_first_name,
                                user_last_name,
                                reason,
                            })).await;
                    }
                }
            },
            // Handle other messages if needed, otherwise ignore
            // Example: Handle the AuthResult if this task needs to react to it too
            HttpMessage::AuthResult(result) => {
                 info!("AuthResult({:?}) received in http_camera_task - likely for debugging or further processing", result);
             }
        }
    }
}