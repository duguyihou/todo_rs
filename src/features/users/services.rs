use sqlx::PgPool;

use super::models::{User, UserError};

pub struct UserSerivce;

impl UserSerivce {
    pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, UserError> {
        let user = sqlx::query_as(
            r#"
            SELECT id, email, created_at, password
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|_| UserError::NotFound)?;
        Ok(user)
    }
}
