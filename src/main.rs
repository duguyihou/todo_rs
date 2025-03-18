mod config;
mod features;
mod utils;
use axum::Router;
use features::{auth, tasks};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::create_db_pool::create_db_pool;

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

    let pool = create_db_pool().await;

    let app = Router::new()
        .merge(tasks::routes::task_routes(pool.clone()))
        .merge(auth::routes::auth_routes(pool));

    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
