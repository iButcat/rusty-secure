use crate::app::Page;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),

    SetLoading(bool),
    SetError(String),
    ClearError,

    Login(String, String),
    Logout,

    FetchData, 
    DataLoaded(Result<Vec<String>, String>), 

    ButtonPressed,
    TextChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    CheckboxToggled(bool),
}