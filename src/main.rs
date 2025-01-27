use std::env;

use axum::{response::IntoResponse, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env = env::var("ENV").unwrap_or("dev".to_string()).to_uppercase();
    let host = env::var(format!("{}_HOST", env)).unwrap();
    let port = env::var(format!("{}_PORT", env)).unwrap();
    let addr = host + ":" + port.as_str();

    let app = Router::new().route("/todo", get(get_todo_handler));
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_todo_handler() -> Result<String, HTTPError> {
    Ok("get todo".to_string())
}

enum HTTPError {
    Internal,
}

impl IntoResponse for HTTPError {
    fn into_response(self) -> axum::response::Response {
        match self {
            HTTPError::Internal => "Internal error".into_response(),
        }
    }
}
