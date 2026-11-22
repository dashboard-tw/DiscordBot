#[cfg(test)]
use serde_json::{Value, json};
use poise::serenity_prelude as serenity;

use crate::{config::Config, http::webhook::WebhookPayload};

#[derive(Clone, Debug)]
struct Notification<'a> {
    ready: &'a serenity::Ready,
}

impl<'a> Notification<'a> {
    fn from_event(ready: &'a serenity::Ready) -> Self {
        Self {
            ready: ready
        }
    }
    fn payload(&self) -> WebhookPayload {
        let guild_count = self.ready.guilds.len();
        let mut embed = serenity::CreateEmbed::new().title("機器人上線").colour(0x1B96DE)
            .field("機器人資料", format!("{}(`{}`)", self.ready.user.name, self.ready.user.id), false)
            .field("伺服器數量", guild_count.to_string(), false);
        if let Some(url) = &self.ready.user.avatar_url() {
            embed = embed.thumbnail(url);
        }
        WebhookPayload::new().embed(embed)
    }
}


pub async fn handle(ready: &serenity::Ready) {
    let notification = Notification::from_event(ready);
    println!("Logged in as {}", ready.user.name);
    let Some(url) = Config::get().webhook_url.as_deref() else {
        return;
    };
    if let Err(error) = notification.payload().send(url).await {
        eprintln!("Ready webhook notification failed: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_value<'a>(payload: &'a Value, name: &str) -> &'a Value {
        &payload["embeds"][0]["fields"].as_array().expect("Embed fields must be an array")
            .iter().find(|field| field["name"] == name)
            .unwrap_or_else(|| panic!("Missing Embed field: {name}"))["value"]
    }

    #[test]
    fn ready_payload_contains_guild_count_and_optional_avatar() {
        let mut ready: serenity::Ready = serde_json::from_value(json!({
            "v": 10,
            "user": { "id": "123", "username": "test-bot", "discriminator": "0", "avatar": null, "bot": true },
            "guilds": [{ "id": "456", "unavailable": true }],
            "session_id": "test-session",
            "resume_gateway_url": "wss://gateway.discord.gg",
            "application": { "id": "123", "flags": 0 },
        })).unwrap();
        let payload = serde_json::to_value(Notification::from_event(&ready).payload()).unwrap();
        assert_eq!(payload["embeds"][0]["title"], "機器人上線");
        assert_eq!(field_value(&payload, "機器人資料"), "test-bot(`123`)");
        assert_eq!(field_value(&payload, "伺服器數量"), "1");
        assert_eq!(payload["allowed_mentions"]["parse"], json!([]));
        assert!(payload["embeds"][0].get("thumbnail").is_none());

        ready.user.avatar = Some("0123456789abcdef0123456789abcdef".parse().unwrap());
        let payload = serde_json::to_value(Notification::from_event(&ready).payload()).unwrap();
        assert_eq!(payload["embeds"][0]["thumbnail"]["url"], ready.user.avatar_url().unwrap());
        ready.guilds.clear();
        assert_eq!(field_value(&serde_json::to_value(Notification::from_event(&ready).payload()).unwrap(), "伺服器數量"), "0");
    }
}