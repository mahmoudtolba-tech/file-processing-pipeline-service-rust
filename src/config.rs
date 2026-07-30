use crate::errors::ConfigError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let server = ServerConfig {
            bind_addr: env::var("SERVER_BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        };

        let db = DbConfig {
            url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".into()))?,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
        };

        Ok(Config { server, db })
    }
}