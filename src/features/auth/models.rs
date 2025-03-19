use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthCredentialsDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreationError,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing crdentials"),
            AuthError::TokenCreationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error")
            }
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}
