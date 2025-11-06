mod app;
use app::run;

mod services;
mod errors;
mod models;

pub fn main() -> iced::Result {
    run()
}
