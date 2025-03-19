use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};
use sqlx::PgPool;

use super::models::{CreateTaskDto, Task, TaskError, TaskStatus, UpdateTaskStatusDto};

pub async fn get_all_tasks(State(pool): State<PgPool>) -> Result<Json<Vec<Task>>, TaskError> {
    let tasks = sqlx::query_as(
        r#"
        SELECT id, task_name, task_status, created_at
        FROM tasks
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| TaskError::InternalServerError)?;

    Ok(Json(tasks))
}

pub async fn get_task_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Task>, TaskError> {
    let task = sqlx::query_as(
        r#"
        SELECT id, task_name, task_status, created_at
        FROM tasks
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| TaskError::NotFound)?;

    Ok(Json(task))
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
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update_task_status_dto): Json<UpdateTaskStatusDto>,
) -> Result<Json<Task>, TaskError> {
    let UpdateTaskStatusDto { task_status } = update_task_status_dto;
    let task = sqlx::query_as(
        r#"
        UPDATE tasks
        SET task_status = $1
        WHERE id = $2
        RETURNING id, task_name, task_status, created_at
        "#,
    )
    .bind(task_status)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| TaskError::InternalServerError)?;

    Ok(Json(task))
}
