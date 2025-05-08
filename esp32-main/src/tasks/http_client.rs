use embassy_time::{Duration, Timer};
use log::{info, error};
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::dns::DnsSocket;
use static_cell::StaticCell;

use crate::http::CamStatusResponse;
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
                    Ok(status_response) => {
                        info!("Received auth response: {:?}", status_response);
                        sender.send(HttpMessage::StatusResult(status_response)).await;
                    },
                    Err(e) => {
                        error!("Request attempt failed: {:?}", e);
                        sender.send(HttpMessage::RequestFailed(e.clone())).await;
                    }
                }
            },
            HttpMessage::RequestFailed(client_error) => {
                info!("RequestFailed({:?}) received in http_camera_task - debugging", client_error);
            },
            HttpMessage::StatusResult(result) => {
                 info!("StatusResult({:?}) received in http_camera_task - debugging", result);
             }
        }
    }
}