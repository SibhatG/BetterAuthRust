use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: u64,  // In seconds
    pub refresh_token_expiry: u64, // In seconds
}

#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub duration: u64, // In seconds
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub email: EmailConfig,
    pub rate_limit: RateLimitConfig,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server: ServerConfig {
                host: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "5000".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a number"),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                    // Default PostgreSQL connection string for Replit
                    "postgres://postgres:postgres@localhost/postgres".to_string()
                }),
                pool_size: env::var("DATABASE_POOL_SIZE")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .expect("DATABASE_POOL_SIZE must be a number"),
            },
            jwt: JwtConfig {
                secret: env::var("SECRET_KEY").unwrap_or_else(|_| {
                    // Default secret key for development only
                    "development_secret_key_please_change_in_production".to_string()
                }),
                access_token_expiry: env::var("ACCESS_TOKEN_EXPIRY")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .expect("ACCESS_TOKEN_EXPIRY must be a number"),
                refresh_token_expiry: env::var("REFRESH_TOKEN_EXPIRY")
                    .unwrap_or_else(|_| "604800".to_string())
                    .parse()
                    .expect("REFRESH_TOKEN_EXPIRY must be a number"),
            },
            email: EmailConfig {
                smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
                smtp_port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "25".to_string())
                    .parse()
                    .expect("SMTP_PORT must be a number"),
                smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
                smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
                from_email: env::var("EMAIL_FROM").unwrap_or_else(|_| "no-reply@example.com".to_string()),
            },
            rate_limit: RateLimitConfig {
                requests: env::var("RATE_LIMIT_REQUESTS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .expect("RATE_LIMIT_REQUESTS must be a number"),
                duration: env::var("RATE_LIMIT_DURATION")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .expect("RATE_LIMIT_DURATION must be a number"),
            },
        }
    }
}
