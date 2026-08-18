use config::{ConfigError, Config as Cfg, Environment, File};
use serde::Deserialize;

/// Application configuration loaded from environment variables.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
}

impl Config {
    /// Load configuration from `.env`, `config.toml` (optional) and environment variables.
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Cfg::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default().separator("__"))
            .build()?;
        cfg.try_deserialize()
    }
}