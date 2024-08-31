pub mod handlers;

use crate::repositories::map_repository::MapRepository;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<Mutex<MapRepository>>,
    pub docker_image: String,
}
