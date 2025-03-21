use std::sync::LazyLock;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct AuthCredentialsDto {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreationError,
    InvalidToken,
    WeakPassword,
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
            AuthError::WeakPassword => (StatusCode::BAD_REQUEST, "Weak password"),
        };
        let body = Json(json!({
            "error": error_message
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub company: String,
    pub exp: usize,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let token_data = decode(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

impl AuthResponse {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_owned(),
        }
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        let encoding = EncodingKey::from_secret(secret);
        let decoding = DecodingKey::from_secret(secret);
        Self { encoding, decoding }
    }
}
