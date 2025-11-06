use std::sync::Arc;

use crate::services::{GoogleAuthService, PictureService, StatusService, UserService};

pub struct AppState {
    pub status_service: Arc<dyn StatusService>,
    pub picture_service: Arc<dyn PictureService>,
    pub user_service: Arc<dyn UserService>,
    pub goolge_auth_service: Arc<dyn GoogleAuthService>,
}
