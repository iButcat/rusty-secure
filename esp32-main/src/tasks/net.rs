use embassy_net::Runner;
use esp_wifi::wifi::WifiDevice;
use embassy_executor::task;

#[embassy_executor::task]
pub async fn net_runner(mut runner: Runner<'static, &'static mut WifiDevice<'static>>) {
    runner.run().await
}