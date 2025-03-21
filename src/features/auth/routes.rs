use super::handlers;
use axum::{routing::post, Router};
use sqlx::PgPool;

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
        .route("/verify", post(handlers::verify_email))
        .with_state(pool)
}
