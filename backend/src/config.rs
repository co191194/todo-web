//! Application configuration

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expires_in: i64,  // minutes
    pub jwt_refresh_expires_in: i64, // days
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-key".to_string()),
            jwt_access_expires_in: env::var("JWT_ACCESS_EXPIRES_IN")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(15),
            jwt_refresh_expires_in: env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
        })
    }
}
