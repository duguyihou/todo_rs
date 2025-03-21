use sqlx::PgPool;

use crate::config::supabase_config::SupabaseConfig;

pub async fn create_db_pool() -> PgPool {
    let supabase_config = SupabaseConfig::from_env().expect("Failed to load Supabase config");

    PgPool::connect(&supabase_config.pub_url)
        .await
        .expect("Failed to connect to database")
}
