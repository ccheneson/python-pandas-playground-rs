use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
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

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use axum::{body, extract::Path, http::StatusCode, response::IntoResponse};

    use crate::{
        http::{
            handlers::{create_api, execute_api},
            AppState,
        },
        repositories::map_repository::MapRepository,
        sandbox::{errors::PythonCodeExecutionError, Sandbox},
    };

    struct SandboxTestOk();
    impl Sandbox for SandboxTestOk {
        fn execute_in_sandbox(&self, py_code: &str) -> Result<String, Box<dyn std::error::Error>> {
            Ok(py_code.to_owned())
        }
    }

    struct SandboxTestKo();
    impl Sandbox for SandboxTestKo {
        fn execute_in_sandbox(&self, _: &str) -> Result<String, Box<dyn std::error::Error>> {
            Err(Box::new(PythonCodeExecutionError(
                "Error in execution of the python code".to_owned(),
            )))
        }
    }

    #[tokio::test]
    async fn test_ok() {
        let repository: MapRepository = MapRepository::new(HashMap::new());
        let sandbox_ok = SandboxTestOk();

        let state = axum::extract::State(AppState {
            repository: Arc::new(Mutex::new(repository)),
            docker_cli: Arc::new(Mutex::new(sandbox_ok)),
        });

        let payload = "mypythoncode".to_owned();

        // Create api => code
        let resp_create = create_api(
            state.clone(),
            Path("apiname".to_owned()),
            payload.to_string(),
        )
        .await
        .into_response();

        //Execute api
        let resp_execute = execute_api(state.clone(), state.clone(), Path("apiname".to_owned()))
            .await
            .into_response();

        let status = resp_execute.status();
        let body_string: String = body::to_bytes(resp_execute.into_body(), usize::MAX)
            .await
            .into_iter()
            .flat_map(|bytes| String::from_utf8(bytes.to_vec()))
            .collect();

        assert_eq!(resp_create.status(), StatusCode::CREATED);
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body_string, payload);
    }

    #[tokio::test]
    async fn test_execute_ko() {
        let repository: MapRepository = MapRepository::new(HashMap::new());
        let sandbox_ko = SandboxTestKo();

        let state = axum::extract::State(AppState {
            repository: Arc::new(Mutex::new(repository)),
            docker_cli: Arc::new(Mutex::new(sandbox_ko)),
        });

        let payload = "mypythoncode".to_owned();

        // Create api => code
        let _ = create_api(
            state.clone(),
            Path("apiname".to_owned()),
            payload.to_string(),
        )
        .await
        .into_response();

        //Execute api
        let resp_execute = execute_api(state.clone(), state.clone(), Path("apiname".to_owned()))
            .await
            .into_response();

        let status = resp_execute.status();
        let body_string: String = body::to_bytes(resp_execute.into_body(), usize::MAX)
            .await
            .into_iter()
            .flat_map(|bytes| String::from_utf8(bytes.to_vec()))
            .collect();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body_string,
            "PythonCodeExecutionError: Error in execution of the python code".to_owned()
        );
    }
}
