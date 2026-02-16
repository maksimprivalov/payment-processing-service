use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL not set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET not set"),
        }
    }
}