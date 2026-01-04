use crate::app::Page;
use crate::models::User;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),

    SetLoading(bool),
    SetError(String),
    ClearError,

    Logout,

    FetchData,
    DataLoaded(Result<Vec<String>, String>),

    ButtonPressed,
    TextChanged(String),
    CheckboxToggled(bool),

    LoginWithGoogle,
    AuthUrlReceived(Result<(String, String), String>),
    TokenInputChanged(String),
    SubmitToken,
    UserFetched(Result<Option<User>, String>),
}
