use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static CONFIG: OnceLock<Result<Config, String>> = OnceLock::new();

pub const BASE_URL: &'static str = "https://dashboard.tw/api/v1";
pub const SERVER_URL: &'static str = "https://discord.com/invite/gU5gndueZH";
pub const GITHUB_URL: &'static str = "https://github.com/dashboard-tw";
pub const PROJECT_NAME: &'static str = "DiscordBot";

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientConfig {
    pub id: String,
    pub secret: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub bot: BotConfig,
    #[serde(default)]
    pub features: FeaturesConfig,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotConfig {
    pub token: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    pub client_config: ClientConfig,
    #[serde(default)]
    pub admins: Vec<String>,
    #[serde(skip)]
    pub key: Option<String>,
    pub webhook_url: Option<String>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FeaturesConfig {
    pub alert: BasicFeatureConfig,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BasicFeatureConfig {
    pub enable: bool,
    pub channels: Vec<String>,
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
            .get_or_init(|| Self::from_file("config.yml"))
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
        self.bot.admins.iter().any(|admin| admin == user_id)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
        Self::from_yaml(&contents)
            .map_err(|e| format!("Invalid configuration in {}: {e}", path.display()))
    }

    fn from_yaml(contents: &str) -> Result<Self, String> {
        let mut config: Self = serde_yaml_ng::from_str(contents).map_err(|e| e.to_string())?;
        for (name, value) in [
            ("bot.token", config.bot.token.as_str()),
            ("bot.client_config.id", config.bot.client_config.id.as_str()),
            (
                "bot.client_config.secret",
                config.bot.client_config.secret.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                return Err(format!("{name} must not be empty"));
            }
        }
        config.bot.webhook_url = config
            .bot
            .webhook_url
            .map(|url| url.trim().to_string())
            .filter(|url| !url.is_empty());
        Ok(config)
    }
}

fn default_prefix() -> String {
    "!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQUIRED: &str =
        "bot:\n  token: test-token\n  client_config:\n    id: test-id\n    secret: test-secret\n";

    #[test]
    fn optional_settings_use_defaults() {
        let config = Config::from_yaml(REQUIRED).unwrap();
        assert_eq!(config.bot.prefix, "!");
        assert!(config.bot.admins.is_empty());
        assert!(config.bot.webhook_url.is_none());
        assert!(config.bot.key.is_none());
        assert!(!config.features.alert.enable);
        assert!(config.features.alert.channels.is_empty());
    }

    #[test]
    fn parses_settings_and_trims_webhook() {
        let config = Config::from_yaml(&format!(
            "{REQUIRED}  prefix: '?'\n  admins: ['123456789012345678', '987654321098765432']\n  webhook_url: ' https://example.com/webhook '\n"
        )).unwrap();
        assert_eq!(config.bot.token, "test-token");
        assert_eq!(config.bot.client_config.id, "test-id");
        assert_eq!(config.bot.client_config.secret, "test-secret");
        assert_eq!(config.bot.prefix, "?");
        assert!(config.is_admin("123456789012345678"));
        assert!(!config.is_admin("unknown"));
        assert_eq!(
            config.bot.webhook_url.as_deref(),
            Some("https://example.com/webhook")
        );
    }

    #[test]
    fn empty_or_null_webhook_is_disabled() {
        for value in ["''", "'   '", "null"] {
            let config = Config::from_yaml(&format!("{REQUIRED}  webhook_url: {value}\n")).unwrap();
            assert!(config.bot.webhook_url.is_none());
        }
    }

    #[test]
    fn rejects_invalid_or_incomplete_settings() {
        for contents in [
            "token: [",
            "token: test-token",
            "token: ''\nclient_config: {id: test-id, secret: test-secret}",
            "token: test-token\nclient_config: {id: ' ', secret: test-secret}",
            "token: test-token\nclient_config: {id: test-id, secret: ''}",
            "bot: {token: '', client_config: {id: test-id, secret: test-secret}}",
            "bot: {token: test-token, client_config: {id: ' ', secret: test-secret}}",
            "bot: {token: test-token, client_config: {id: test-id, secret: ''}}",
            "bot: {token: test-token}",
        ] {
            assert!(Config::from_yaml(contents).is_err());
        }
        assert!(Config::from_yaml(&format!("{REQUIRED}  admins: not-a-list")).is_err());
        assert!(Config::from_yaml(&format!("{REQUIRED}unknown_setting: true")).is_err());
        assert!(Config::from_yaml(&format!("{REQUIRED}  unknown_setting: true")).is_err());
    }

    #[test]
    fn parses_alert_settings() {
        let config = Config::from_yaml(&format!(
            "{REQUIRED}features:\n  alert:\n    enable: true\n    channels: ['123456789012345678', '987654321098765432']\n"
        )).unwrap();
        assert!(config.features.alert.enable);
        assert_eq!(
            config.features.alert.channels,
            ["123456789012345678", "987654321098765432"]
        );
    }

    #[test]
    fn partial_feature_settings_use_defaults() {
        for features in [
            "",
            "features: {}",
            "features: {alert: {}}",
            "features: {alert: {enable: false}}",
        ] {
            let config = Config::from_yaml(&format!("{REQUIRED}{features}")).unwrap();
            assert!(!config.features.alert.enable);
            assert!(config.features.alert.channels.is_empty());
        }
        let config =
            Config::from_yaml(&format!("{REQUIRED}features: {{alert: {{enable: true}}}}")).unwrap();
        assert!(config.features.alert.enable);
        assert!(config.features.alert.channels.is_empty());
    }

    #[test]
    fn rejects_invalid_feature_settings() {
        for features in [
            "features: {unknown: {}}",
            "features: {alert: {enabled: true}}",
            "features: {alert: {enable: not-a-bool}}",
            "features: {alert: {channels: not-a-list}}",
            "features: {alert: {channels: [{}]}}",
        ] {
            assert!(Config::from_yaml(&format!("{REQUIRED}{features}")).is_err());
        }
    }

    #[test]
    fn empty_required_field_reports_nested_path() {
        let error = Config::from_yaml(
            "bot: {token: '', client_config: {id: test-id, secret: test-secret}}",
        )
        .err()
        .unwrap();
        assert_eq!(error, "bot.token must not be empty");
    }

    #[test]
    fn example_file_loads() {
        let config =
            Config::from_file(Path::new(env!("CARGO_MANIFEST_DIR")).join("config.example.yml"))
                .unwrap();
        assert_eq!(config.bot.prefix, "!");
        assert!(config.features.alert.enable);
        assert!(config.features.alert.channels.is_empty());
    }

    #[test]
    fn missing_file_reports_path() {
        let error = Config::from_file("missing-config-for-test.yml")
            .err()
            .unwrap();
        assert!(error.contains("missing-config-for-test.yml"));
    }
}
