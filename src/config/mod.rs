use std::env;
use std::sync::OnceLock;

static CONFIG: OnceLock<Result<Config, String>> = OnceLock::new();

pub const BASE_URL: &'static str = "https://dashboard.tw/api/v1";

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
    pub admins: Vec<String>,
    pub key: Option<String>,
    pub webhook_url: Option<String>,
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
    pub fn init() -> Result<&'static Self, String> {
        CONFIG
            .get_or_init(Self::from_env)
            .as_ref()
            .map_err(Clone::clone)
    }

    pub fn get() -> &'static Self {
        CONFIG
            .get()
            .and_then(|result| result.as_ref().ok())
            .expect("Config::init() must succeed before Config::get() is called")
    }

    pub fn is_admin(&self, user_id: &str) -> bool {
        self.admins.iter().any(|admin| admin == user_id)
    }

    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let token = env::var("TOKEN").map_err(|e| format!("Failed to load TOKEN: {e}"))?;

        let client_id =
            env::var("CLIENT_ID").map_err(|e| format!("Failed to load CLIENT_ID: {e}"))?;

        let client_secret =
            env::var("CLIENT_SECRET").map_err(|e| format!("Failed to load CLIENT_SECRET: {e}"))?;

        let prefix = env::var("PREFIX").unwrap_or_else(|_| "!".to_string());

        let raw_admins = env::var("ADMINS").unwrap_or_else(|_| "[]".to_string());
        let admins = parse_admins(&raw_admins)?;

        let client_config = ClientConfig::new(client_id, client_secret);

        Ok(Self {
            token: token,
            prefix: prefix,
            client_config: client_config,
            key: None,
            admins: admins,
            webhook_url: env::var("WEBHOOK_URL").ok()
                .map(|url| url.trim().to_string()).filter(|url| !url.is_empty()),
        })
    }
}

fn parse_admins(raw: &str) -> Result<Vec<String>, String> {
    serde_json::from_str(raw).map_err(|_| {
        "Invalid ADMINS: expected a JSON string array, e.g. ADMINS='[\"123456789\"]'".to_string()
    })
}