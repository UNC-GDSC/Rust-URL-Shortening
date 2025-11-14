use std::env;

pub struct Config {
    pub database_url: String,
    pub base_url: String,
}

impl Config {
    /// Loads configuration from environment variables.
    /// Panics if DATABASE_URL is not set.
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        Config { database_url, base_url }
    }
}
