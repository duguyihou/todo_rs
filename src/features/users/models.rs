use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response<Body> {
        match self {
            UserError::NotFound => StatusCode::NOT_FOUND.into_response(),
            UserError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
