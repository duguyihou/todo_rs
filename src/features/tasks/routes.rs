use axum::{
    middleware,
    routing::{get, patch},
    Router,
};
use sqlx::PgPool;

use crate::features::auth::middlewares::jwt_middleware;

use super::handlers;

pub fn task_routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/tasks",
            get(handlers::get_all_tasks).post(handlers::create_task),
        )
        .route(
            "/tasks/{id}",
            get(handlers::get_task_by_id).delete(handlers::delete_task),
        )
        .route("/tasks/{id}/status", patch(handlers::update_task_status))
        .layer(middleware::from_fn(jwt_middleware))
        .with_state(pool)
}
