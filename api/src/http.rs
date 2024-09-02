use crate::{repositories::ApiToCode, sandbox::Sandbox};
use std::sync::{Arc, Mutex};

pub mod errors;
pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<Mutex<dyn ApiToCode + Send + Sync>>,
    pub sandbox: Arc<Mutex<dyn Sandbox + Send + Sync>>,
}
