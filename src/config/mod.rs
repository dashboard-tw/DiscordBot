use std::env;

pub struct Config {
    pub token: String,
    pub prefix: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let token = env::var("TOKEN")
            .map_err(|e| format!("Failed to load TOKEN: {e}"))?;

        let api_key = env::var("API_KEY")
            .map_err(|e| format!("Failed to load API_KEY: {e}"))?;

        let prefix = env::var("PREFIX").unwrap_or_else(|_| "!".to_string());

        Ok(Self {
            token,
            prefix,
            api_key,
        })
    }
}