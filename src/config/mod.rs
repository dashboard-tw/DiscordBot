use std::env;

pub const BASE_URL: &'static str = "http://localhost:8080";

#[derive(Clone)]
pub struct ClientConfig {
    pub id: String,
    pub secret: String,
}

#[derive(Clone)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub client_config: ClientConfig,
    pub key: Option<String>,
}

impl ClientConfig {
    pub fn new(id: String, secret: String) -> Self {
        Self {
            id: id,
            secret: secret,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let token = env::var("TOKEN").map_err(|e| format!("Failed to load TOKEN: {e}"))?;

        let client_id =
            env::var("CLIENT_ID").map_err(|e| format!("Failed to load CLIENT_ID: {e}"))?;

        let client_secret =
            env::var("CLIENT_SECRET").map_err(|e| format!("Failed to load CLIENT_SECRET: {e}"))?;

        let prefix = env::var("PREFIX").unwrap_or_else(|_| "!".to_string());

        let client_config = ClientConfig::new(client_id, client_secret);

        Ok(Self {
            token: token,
            prefix: prefix,
            client_config: client_config,
            key: None,
        })
    }
}
