use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

use crate::features::auth::models::Claims;

use super::{
    models::{User, UserError},
    services::UserSerivce,
};

pub async fn get_user(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<User>, UserError> {
    let user_id = claims.sub;

    tracing::info!("User ID: {}", user_id);

    let user = UserSerivce::get_user_by_id(&pool, user_id)
        .await
        .map_err(|_| UserError::InternalServerError)?;
    Ok(Json(user))
}
