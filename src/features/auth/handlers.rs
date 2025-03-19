use super::models::{AuthCredentialsDto, AuthError};
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn login() -> &'static str {
    "Login successful"
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(auth_credentials_dto): Json<AuthCredentialsDto>,
) -> Result<Json<String>, AuthError> {
    let AuthCredentialsDto { email, password } = auth_credentials_dto;
    let created_at = chrono::Utc::now();

    sqlx::query_as(
        r#"
        INSERT INTO users (email, password, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, email, password, created_at
        "#,
    )
    .bind(email)
    .bind(password)
    .bind(created_at)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        println!("🐵 ------ err {}", err);
        AuthError::WrongCredentials
    })?;

    Ok(Json(format!("User registered successfully")))
}
