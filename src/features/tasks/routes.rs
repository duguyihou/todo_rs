use axum::{
    routing::{get, patch},
    Router,
};

use super::handlers;

pub fn task_routes() -> Router {
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
}
