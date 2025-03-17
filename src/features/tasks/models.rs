use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskDto {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
