use crate::{repositories::ApiToCode, sandbox::Sandbox};
use std::sync::{Arc, Mutex};

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<Mutex<dyn ApiToCode + Send + Sync>>,
    pub docker_cli: Arc<Mutex<dyn Sandbox + Send + Sync>>,
}
