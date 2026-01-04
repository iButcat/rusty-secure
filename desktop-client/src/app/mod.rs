pub mod config;
pub mod message;
pub mod state;

use message::Message;
use state::AppState;

use crate::app::state::Page;
use crate::models::AuthResponse;
use crate::services::RustySecureApi;

use iced::widget::{button, column, container, text, text_input};
use iced::{Element, Task};

pub fn run() -> iced::Result {
    iced::run("Rusty Secure Client", update, view)
}

// TODO: Handle the error correctly instead of having only Task::none()
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ButtonPressed => {
            println!("Button pressed");
            Task::none()
        }
        Message::TextChanged(text) => {
            println!("Text changed: {}", text);
            Task::none()
        }
        Message::CheckboxToggled(checked) => {
            println!("Checkbox toggled: {}", checked);
            Task::none()
        }
        Message::NavigateTo(page) => {
            state.current_page = page;
            Task::none()
        }
        Message::SetLoading(loading) => {
            state.loading = loading;
            Task::none()
        }
        Message::SetError(error) => {
            state.error_message = Some(error);
            Task::none()
        }
        Message::ClearError => {
            state.error_message = None;
            Task::none()
        }
        Message::Logout => {
            println!("Logout");
            Task::none()
        }
        Message::FetchData => {
            println!("Fetching data...");
            Task::none()
        }
        Message::DataLoaded(result) => match result {
            Ok(data) => {
                println!("Data loaded: {:?}", data);
                Task::none()
            }
            Err(e) => {
                println!("Error loading data: {}", e);
                Task::none()
            }
        },
        Message::LoginWithGoogle => {
            let api = state.api_service.clone();

            Task::perform(
                async move { api.get_auth_url().await.map_err(|e| e.to_string()) },
                Message::AuthUrlReceived,
            )
        }
        Message::AuthUrlReceived(result) => {
            match result {
                Ok((url, _auth_state)) => {
                    println!("Go to URL: {}", url);
                    webbrowser::open(&url).ok();
                }
                Err(e) => {
                    println!("Auth Error: {}", e);
                    state.error_message = Some(e);
                }
            }
            Task::none()
        }
        Message::TokenInputChanged(input) => {
            println!("Token input changed: {}", input);
            state.token_input = input;
            Task::none()
        }
        Message::SubmitToken => {
            let parsed: Result<AuthResponse, _> = serde_json::from_str(&state.token_input);

            match parsed {
                Ok(auth_data) => {
                    println!(
                        "Token valid. Fetching profile for Google ID: {}",
                        auth_data.user_info.id,
                    );
                    state.set_loading(true);

                    let api = state.api_service.clone();
                    let google_id = auth_data.user_info.id;

                    Task::perform(
                        async move {
                            api.get_user_by_google_id(google_id)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::UserFetched,
                    )
                }
                Err(e) => {
                    println!("Parsing Failed: {}", e);
                    state.error_message = Some(format!("Invalid Token JSON: {}", e));
                    Task::none()
                }
            }
        }
        Message::UserFetched(result) => {
            state.set_loading(false);

            match result {
                Ok(Some(user)) => {
                    println!("User fetched: {:?}", user);
                    state.user = Some(user);
                    state.user_logged_in = true;
                    state.current_page = Page::Home;
                }
                Ok(None) => {
                    state.error_message = Some("User not found in database.".to_string());
                }
                Err(e) => {
                    println!("Failed to fetch profile: {}", e);
                    state.error_message = Some(format!("Failed to fetch profile: {}", e));
                }
            }
            Task::none()
        }
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
        text("Login to Rusty Secure").size(30),
        container(
            column![
                text("Step 1: Open Browser").size(18),
                button("Login with Google").on_press(Message::LoginWithGoogle),
            ]
            .spacing(10)
        )
        .padding(10)
        .style(iced::widget::container::bordered_box),
        container(
            column![
                text("Step 2: Paste code").size(18),
                text("Copy the code from the browser and paste it here:"),
                text_input("Paste JSON Here...", &state.token_input)
                    .on_input(Message::TokenInputChanged)
                    .on_submit(Message::SubmitToken),
                button("Complete Login").on_press(Message::SubmitToken)
            ]
            .spacing(10)
        )
    ]
    .spacing(10)
    .padding(20)
    .into()
}
