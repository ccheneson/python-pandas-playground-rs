use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    repositories::ApiToCode,
    sandbox::{docker_sandbox::DockerSandbox, Sandbox},
};

use super::AppState;

pub async fn create_api(
    State(repository): State<AppState>,
    Path(api): Path<String>,
    payload: String,
) -> impl IntoResponse {
    println!("{}", payload);
    let mut data = repository.repository.lock().expect("mutex was poisoned");
    data.add_code(api, payload);
    (StatusCode::CREATED, "Python code added to repository")
}

pub async fn execute_api(
    State(repository_state): State<AppState>,
    State(docker_image_state): State<AppState>,
    Path(api): Path<String>,
) -> impl IntoResponse {
    let data = repository_state
        .repository
        .lock()
        .expect("mutex was poisoned");

    match data.get_code(api) {
        None => {
            std::mem::drop(data);
            (StatusCode::NOT_FOUND, "Code not found".to_owned())
        }
        Some(py_code) => {
            let sandbox = DockerSandbox::new(py_code.to_owned(), docker_image_state.docker_image);
            let result = sandbox.execute_in_sandbox();
            match result {
                Ok(output) => (StatusCode::OK, output),
                Err(err) => {
                    std::mem::drop(data);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            }
        }
    }
}
