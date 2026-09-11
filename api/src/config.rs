use std::env;

pub struct Config {
    pub api_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            api_url: env::var("API_URL").unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap_or("supersecrettoken".to_string()),
        }
    }
}
