use axum::{routing::post, Router};
use python_pandas_playground::{
    controllers::{
        handlers::{create_api, execute_api},
        AppState,
    },
    repositories::map_repository::MapRepository,
};
use std::{
    collections::HashMap,
    env,
    io::Error,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let inner_repo: HashMap<String, String> = HashMap::new();
    let sandbox_docker_image = env::var("DOCKER_IMAGE").unwrap_or("amancevice/pandas".to_owned());

    let state = AppState {
        repository: Arc::new(Mutex::new(MapRepository::new(inner_repo))),
        docker_image: sandbox_docker_image,
    };

    let app = Router::new()
        .route("/code/:api", post(create_api))
        .route("/execute/:api", post(execute_api))
        .nest_service("/", ServeDir::new("www/dist/"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
