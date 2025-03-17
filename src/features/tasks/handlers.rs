use axum::{extract::Path, response::Html, Json};

use super::models::{CreateTaskDto, Task, TaskError, TaskStatus};

pub async fn get_all_tasks() -> Result<Json<Vec<Task>>, TaskError> {
    Ok(Json(vec![
        Task {
            id: 1,
            title: "Learn Axum".to_string(),
            completed: TaskStatus::InProgress,
        },
        Task {
            id: 2,
            title: "Write blog post".to_string(),
            completed: TaskStatus::Open,
        },
    ]))
}

pub async fn get_task_by_id(Path(id): Path<String>) -> Html<Json<String>> {
    Html(Json(format!("Get task with id: {}", id)))
}

pub async fn create_task(Json(create_task_dto): Json<CreateTaskDto>) -> Html<&'static str> {
    Html("Create a new task")
}

pub async fn delete_task(Path(id): Path<String>) -> Html<Json<String>> {
    Html(Json(format!("Delete task with id: {}", id)))
}

pub async fn update_task_status(
    Path(id): Path<String>,
    Json(status): Json<TaskStatus>,
) -> Html<Json<String>> {
    Html(Json(format!("Update task status with id: {}", id)))
}
