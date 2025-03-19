use sqlx::PgPool;
use validator::{Validate, ValidationErrors};

use super::models::AuthCredentialsDto;

pub struct AuthService;

impl AuthService {
    pub async fn login() -> Result<(), ()> {
        Ok(())
    }

    pub async fn create_user(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let created_at = chrono::Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO users (email, password, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, email, password, created_at
            "#,
            email,
            password,
            created_at
        )
        .fetch_one(pool)
        .await
        .map_err(|_| sqlx::Error::RowNotFound)?;
        Ok(())
    }

    pub fn validate_credentials(
        auth_credentials_dto: &AuthCredentialsDto,
    ) -> Result<(), ValidationErrors> {
        auth_credentials_dto.validate()
    }

    pub fn check_password_strength(password: &str) -> Result<(), String> {
        let estimate = zxcvbn::zxcvbn(&password, &[]);
        if estimate.score() < zxcvbn::Score::Three {
            return Err("Password is too weak".to_string());
        }
        Ok(())
    }

    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT email
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| sqlx::Error::RowNotFound)?;
        Ok(result.is_some())
    }
}
