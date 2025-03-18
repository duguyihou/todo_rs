use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};
use sqlx::PgPool;

use super::models::{CreateTaskDto, Task, TaskError, TaskStatus};

pub async fn get_all_tasks() -> Result<Json<Vec<Task>>, TaskError> {
    Ok(Json(vec![
        Task {
            id: 1,
            task_name: "Learn Axum".to_string(),
            task_status: TaskStatus::InProgress,
            created_at: chrono::Utc::now(),
        },
        Task {
            id: 2,
            task_name: "Write blog post".to_string(),
            task_status: TaskStatus::Open,
            created_at: chrono::Utc::now(),
        },
    ]))
}

pub async fn get_task_by_id(Path(id): Path<String>) -> Html<Json<String>> {
    Html(Json(format!("Get task with id: {}", id)))
}

pub async fn create_task(
    State(pool): State<PgPool>,
    Json(create_task_dto): Json<CreateTaskDto>,
) -> Result<Json<Task>, TaskError> {
    let CreateTaskDto {
        task_name,
        task_status,
    } = create_task_dto;
    let created_at = chrono::Utc::now();

    let task = sqlx::query_as(
        r#"
        INSERT INTO tasks (task_name, task_status, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, task_name, task_status, created_at
        "#,
    )
    .bind(task_name)
    .bind(task_status)
    .bind(created_at)
    .fetch_one(&pool)
    .await
    .map_err(|_| TaskError::InternalServerError)?;

    Ok(Json(task))
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
