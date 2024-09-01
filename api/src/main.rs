use axum::{routing::post, Router};
use python_pandas_playground::{
    http::{
        handlers::{create_api, execute_api},
        AppState,
    },
    repositories::map_repository::MapRepository,
    sandbox::docker_sandbox::DockerSandbox,
};
use std::{
    collections::HashMap,
    env,
    io::Error,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

const BINDING_ADRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let inner_repo: HashMap<String, String> = HashMap::new();
    let repository = MapRepository::new(inner_repo);

    let docker_image = env::var("DOCKER_IMAGE").unwrap_or("amancevice/pandas".to_owned());
    let sandbox = DockerSandbox::new(docker_image);

    let state = AppState {
        repository: Arc::new(Mutex::new(repository)),
        docker_cli: Arc::new(Mutex::new(sandbox)),
    };

    let app = Router::new()
        .route("/code/:api", post(create_api))
        .route("/execute/:api", post(execute_api))
        .nest_service("/", ServeDir::new("./dist/"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(BINDING_ADRESS).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
