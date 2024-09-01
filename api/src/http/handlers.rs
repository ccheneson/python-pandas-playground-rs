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
    State(docker_cli_state): State<AppState>,
    Path(api): Path<String>,
) -> impl IntoResponse {
    let repository = repository_state
        .repository
        .lock()
        .expect("[repository]mutex was poisoned");

    let docker_cli = &mut docker_cli_state
        .docker_cli
        .lock()
        .expect("[docker-cli]mutex was poisoned");

    match repository.get_code(api) {
        None => {
            std::mem::drop(repository);
            (StatusCode::NOT_FOUND, "Code not found".to_owned())
        }
        Some(py_code) => {
            let result = docker_cli.execute_in_sandbox(py_code);
            match result {
                Ok(output) => (StatusCode::OK, output),
                Err(err) => {
                    std::mem::drop(repository);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            }
        }
    }
}
