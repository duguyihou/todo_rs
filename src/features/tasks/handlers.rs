use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sqlx::PgPool;

use crate::features::auth::models::Claims;

use super::models::{CreateTaskDto, Task, TaskError, UpdateTaskStatusDto};

pub async fn get_all_tasks(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Task>>, TaskError> {
    let user_id = claims.sub;

    let tasks = sqlx::query_as(
        r#"
        SELECT id, task_name, task_status, created_at, user_id
        FROM tasks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
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
    Extension(claims): Extension<Claims>,
    Json(create_task_dto): Json<CreateTaskDto>,
) -> Result<Json<Task>, TaskError> {
    let CreateTaskDto {
        task_name,
        task_status,
    } = create_task_dto;
    let created_at = chrono::Utc::now();
    let user_id = claims.sub;

    let task = sqlx::query_as(
        r#"
        INSERT INTO tasks (task_name, task_status, created_at, user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, task_name, task_status, created_at, user_id
        "#,
    )
    .bind(task_name)
    .bind(task_status)
    .bind(created_at)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| TaskError::InternalServerError)?;

    Ok(Json(task))
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<String>, TaskError> {
    sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|_| TaskError::NotFound)?;

    Ok(Json(format!("Task with id {} has been deleted", id)))
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
