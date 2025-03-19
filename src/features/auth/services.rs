use super::models::{AuthCredentialsDto, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::Json;
use sqlx::PgPool;
use validator::{Validate, ValidationErrors};
pub struct AuthService;

impl AuthService {
    pub async fn find_user(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<Json<User>, String> {
        let user = Self::find_user_by_email(pool, email)
            .await
            .map_err(|e| e.to_string())?;
        Self::verify_password(password, &user.password).map_err(|e| e.to_string())?;
        Ok(user)
    }

    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Json<User>, String> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(Json(user))
    }

    pub async fn create_user(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let created_at = chrono::Utc::now();

        let password_hash = Self::hash_password(password).map_err(|e| sqlx::Error::Protocol(e))?;

        sqlx::query!(
            r#"
                INSERT INTO users (email, password, created_at)
                VALUES ($1, $2, $3)
                RETURNING id, email, password, created_at
                "#,
            email,
            password_hash,
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

    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<(), String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| e.to_string())
    }
}
