use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct SupabaseConfig {
    pub pub_url: String,
}

impl SupabaseConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            pub_url: env::var("DATABASE_URL")?,
        })
    }
}
