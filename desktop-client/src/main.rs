use iced::widget::{button, column, text};
use iced::Element;

pub fn main() -> iced::Result {
    iced::run("Simple Counter - Iced", update, view)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn update(value: &mut i32, message: Message) {
    match message {
        Message::IncrementPressed => {
            *value += 1;
        }
        Message::DecrementPressed => {
            *value -= 1;
        }
    }
}

fn view(value: &i32) -> Element<Message> {
    column![
        button("+").on_press(Message::IncrementPressed),

        text(*value).size(50),

        button("-").on_press(Message::DecrementPressed)
    ]
    .padding(20)
    .into()
}