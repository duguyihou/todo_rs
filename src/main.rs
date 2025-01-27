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

    let app = Router::new().route("/todo", get(get_todo_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
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
