use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", get(handlers::login))
        .route("/register", get(handlers::register))
}
