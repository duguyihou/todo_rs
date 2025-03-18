mod config;
mod features;
use axum::Router;
use config::supabase_config::SupabaseConfig;
use features::{auth, tasks};
use sqlx::PgPool;
use std::env;
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

    let supabase_config = SupabaseConfig::from_env().expect("Failed to load Supabase config");

    let pool = PgPool::connect(&supabase_config.pub_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .merge(tasks::routes::task_routes(pool.clone()))
        .merge(auth::routes::auth_routes(pool));

    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
