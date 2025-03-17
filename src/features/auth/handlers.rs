use super::models::{LoginRequest, RegisterRequest};
use axum::{extract::State, Json};

pub async fn login(Json(payload): Json<LoginRequest>) -> &'static str {
    "Login successful"
}

pub async fn register(Json(payload): Json<RegisterRequest>) -> &'static str {
    // Handle registration logic
    "Registration successful"
}
