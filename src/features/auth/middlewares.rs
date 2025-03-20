use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::services::AuthService;

pub async fn jwt_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match AuthService::validate_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
