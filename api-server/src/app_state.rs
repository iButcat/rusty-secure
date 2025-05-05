use std::sync::Arc;

use crate::services::{StatusService, PictureService};

pub struct AppState {
    pub status_service: Arc<dyn StatusService>,
    pub picture_service: Arc<dyn PictureService>
}