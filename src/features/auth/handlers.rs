use super::{
    models::{AuthCredentialsDto, AuthResponse},
    services::AuthService,
};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(auth_credentials_dto): Json<AuthCredentialsDto>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let AuthCredentialsDto { email, password } = auth_credentials_dto;
    match AuthService::find_user(&pool, &email, &password).await {
        Ok(user) => {
            let auth_response = AuthService::create_auth_response(&user).unwrap();
            Ok(Json(auth_response))
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())),
    }
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

pub async fn verify_email(
    State(pool): State<PgPool>,
    token: String,
) -> Result<Json<String>, (StatusCode, String)> {
    if let Err(err) = AuthService::verify(&pool, &token).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    }
    Ok(Json("Email verified successfully".to_string()))
}
