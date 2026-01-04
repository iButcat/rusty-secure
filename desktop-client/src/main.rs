mod app;
use app::run;

mod errors;
mod models;
mod services;

#[tokio::main]
async fn main() -> iced::Result {
    run()
}
