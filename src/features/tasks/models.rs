use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Task {
    pub id: i32,
    pub task_name: String,
    pub task_status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskDto {
    pub task_name: String,
    pub task_status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFilterDto {
    pub task_status: Option<TaskStatus>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskStatusDto {
    pub task_status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    InProgress,
    Completed,
}

pub enum TaskError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for TaskError {
    fn into_response(self) -> Response<Body> {
        match self {
            TaskError::NotFound => StatusCode::NOT_FOUND.into_response(),
            TaskError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
