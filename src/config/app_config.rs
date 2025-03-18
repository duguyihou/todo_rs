use std::env;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    host: String,
    port: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(AppConfig {
            port: env::var("PORT")?.parse().unwrap(),
            host: env::var("HOST").unwrap(),
        })
    }

    pub fn get_addr(&self) -> String {
        let host = self.host.to_owned();
        let port = self.port.to_string();
        let addr = host + ":" + port.as_str();
        addr
    }
}
