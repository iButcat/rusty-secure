pub mod config;
pub mod message;
pub mod state;

use message::Message;
use state::AppState;

use crate::app::state::Page;

use iced::Element;
use iced::widget::{button, column, container, text, text_input};

pub fn run() -> iced::Result {
    iced::run("Rusty Secure Client", update, view)
}

fn update(state: &mut AppState, message: Message) {
    match message {
        Message::ButtonPressed => {
            println!("Button pressed");
        }
        Message::TextChanged(text) => {
            println!("Text changed: {}", text);
        }
        Message::UsernameChanged(username) => {
            println!("Username changed: {}", username);
            state.username = username;
        }
        Message::PasswordChanged(password) => {
            println!("Password changed: {}", password);
            state.password = password;
        }
        Message::CheckboxToggled(checked) => {
            println!("Checkbox toggled: {}", checked);
        }
        Message::NavigateTo(page) => {
            state.current_page = page;
        }
        Message::SetLoading(loading) => {
            state.loading = loading;
        }
        Message::SetError(error) => {
            state.error_message = Some(error);
        }
        Message::ClearError => {
            state.error_message = None;
        }
        Message::Login(username, password) => {
            println!("Login attempt: {} / {}", username, password);
            println!(
                "Got stored username: {}, and password: {}",
                state.username, state.password
            );
        }
        Message::Logout => {
            println!("Logout");
        }
        Message::FetchData => {
            println!("Fetching data...");
        }
        Message::DataLoaded(result) => match result {
            Ok(data) => println!("Data loaded: {:?}", data),
            Err(e) => println!("Error loading data: {}", e),
        },
    }
}

fn view(state: &AppState) -> Element<Message> {
    let content = match state.current_page {
        Page::Home => home_view(),
        Page::Settings => settings_view(),
        Page::Login => login_view(state),
    };

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x(1000)
        .center_y(1000)
        .into()
}

fn home_view() -> Element<'static, Message> {
    column![
        text("Welcome to Rusty Secure").size(24),
        button("Go to Settings").on_press(Message::NavigateTo(Page::Settings)),
        button("Logout").on_press(Message::Logout),
    ]
    .spacing(10)
    .padding(20)
    .into()
}

fn settings_view() -> Element<'static, Message> {
    column![
        text("Settings").size(24),
        button("Back to Home").on_press(Message::NavigateTo(Page::Home)),
    ]
    .spacing(10)
    .padding(20)
    .into()
}

fn login_view(state: &AppState) -> Element<'static, Message> {
    column![
        text("Login").size(24),
        text_input("Username", &state.username).on_input(Message::UsernameChanged),
        text_input("Password", &state.password).on_input(Message::PasswordChanged),
        button("Login").on_press(Message::Login("user".to_string(), "pass".to_string())),
    ]
    .spacing(10)
    .padding(20)
    .into()
}
