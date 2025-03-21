use axum::{middleware, routing::get, Router};
use sqlx::PgPool;

use crate::features::auth::middlewares::jwt_middleware;

use super::handlers;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(handlers::get_user))
        .layer(middleware::from_fn(jwt_middleware))
        .with_state(pool)
}
