use super::{models::AuthCredentialsDto, services::AuthService};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(auth_credentials_dto): Json<AuthCredentialsDto>,
) -> Result<String, (StatusCode, String)> {
    let AuthCredentialsDto { email, password } = auth_credentials_dto;
    if let Err(err) = AuthService::find_user(&pool, &email, &password).await {
        return Err((StatusCode::BAD_REQUEST, err.to_string()));
    }
    Ok(format!("Login successfully"))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(auth_credentials_dto): Json<AuthCredentialsDto>,
) -> Result<Json<String>, (StatusCode, String)> {
    if let Err(errors) = AuthService::validate_credentials(&auth_credentials_dto) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

    let AuthCredentialsDto { email, password } = auth_credentials_dto;

    if let Err(message) = AuthService::check_password_strength(&password) {
        return Err((StatusCode::BAD_REQUEST, message));
    }

    if AuthService::email_exists(&pool, &email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        return Err((StatusCode::CONFLICT, "Email already exists".to_string()));
    }

    if let Err(err) = AuthService::create_user(&pool, &email, &password).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    }

    Ok(Json(format!("User registered successfully")))
}
