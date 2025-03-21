mod config;
mod features;
mod utils;
use axum::Router;
use config::app_config::AppConfig;
use features::{auth, tasks, users};
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

    let app_config = AppConfig::from_env().expect("Failed to load app config");
    let addr = app_config.get_addr();

    let pool = create_db_pool().await;

    let app = Router::new()
        .merge(tasks::routes::task_routes(pool.clone()))
        .merge(users::routes::user_routes(pool.clone()))
        .merge(auth::routes::auth_routes(pool));

    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
