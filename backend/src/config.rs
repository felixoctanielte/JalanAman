use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub port: u16,
    pub vapid_private_key_pem: String,
    pub vapid_public_key: String,
    pub google_maps_api_key: String,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            vapid_private_key_pem: env::var("VAPID_PRIVATE_KEY_PEM")
                .unwrap_or_default()
                .replace("\\n", "\n"),
            vapid_public_key: env::var("VAPID_PUBLIC_KEY").unwrap_or_default(),
            google_maps_api_key: env::var("GOOGLE_MAPS_API_KEY").unwrap_or_default(),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }
}
