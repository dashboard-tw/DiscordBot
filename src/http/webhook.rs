use std::time::Duration;
use serde::Serialize;
use poise::serenity_prelude::{CreateAllowedMentions, CreateEmbed};

use super::HttpClient;

#[derive(Clone, Debug, Serialize)]
pub struct WebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    embeds: Vec<CreateEmbed>,
    allowed_mentions: CreateAllowedMentions,
}

impl Default for WebhookPayload {
    fn default() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            embeds: Vec::new(),
            allowed_mentions: CreateAllowedMentions::new().everyone(false).all_users(false).all_roles(false),
        }
    }
}

impl WebhookPayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn avatar_url(mut self, url: impl Into<String>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub async fn send(&self, url: &str) -> Result<(), reqwest::Error> {
        send(url, self).await
    }
}

pub async fn send<T: Serialize + ?Sized>(url: &str, payload: &T) -> Result<(), reqwest::Error> {
    HttpClient::get().client.post(url)
        .timeout(Duration::from_secs(10))
        .json(payload)
        .send().await
        .and_then(|response| response.error_for_status())
        .map(|_| ())
        .map_err(|error| error.without_url())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_builder_serializes_content_and_embeds() {
        let payload = WebhookPayload::new().content("hello").username("bot")
            .avatar_url("https://example.com/avatar.png")
            .embed(CreateEmbed::new().title("first"))
            .embed(CreateEmbed::new().title("second"));
        let value = serde_json::to_value(payload).unwrap();
        assert_eq!(value["content"], "hello");
        assert_eq!(value["username"], "bot");
        assert_eq!(value["avatar_url"], "https://example.com/avatar.png");
        assert_eq!(value["embeds"].as_array().unwrap().len(), 2);
        assert_eq!(value["allowed_mentions"]["parse"], serde_json::json!([]));
        let empty = serde_json::to_value(WebhookPayload::new()).unwrap();
        assert!(empty.get("content").is_none());
    }

    #[tokio::test]
    async fn invalid_url_returns_error_without_exposing_credentials() {
        let error = WebhookPayload::new().content("test")
            .send("invalid://example.test/secret-token").await.unwrap_err();
        assert!(error.url().is_none());
        assert!(!error.to_string().contains("secret-token"));
    }
}