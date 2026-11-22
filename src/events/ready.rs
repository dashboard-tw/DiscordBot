use serde_json::{Value, json};
use poise::serenity_prelude as serenity;

use crate::{config::Config, http::webhook};

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
    fn payload(&self) -> Value {
        let guild_count = self.ready.guilds.len();
        let mut payload = json!({
            "allowed_mentions": { "parse": [] },
            "embeds": [{
                "title": "機器人上線",
                "color": 0x1B96DE,
                "fields": [
                    { "name": "機器人資料", "value": format!("{}(`{}`)", self.ready.user.name, self.ready.user.id) },
                    { "name": "伺服器數量", "value": guild_count.to_string() },
                ],
            }],
        });
        if let Some(url) = &self.ready.user.avatar_url() {
            payload["embeds"][0]["thumbnail"] = json!({ "url": url });
        }
        payload
    }
}


pub async fn handle(ready: &serenity::Ready) {
    let notification = Notification::from_event(ready);
    println!("Logged in as {}", ready.user.name);
    let Some(url) = Config::get().webhook_url.as_deref() else {
        return;
    };
    if let Err(error) = webhook::send(url, &notification.payload()).await {
        eprintln!("Ready webhook notification failed: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let payload = Notification::from_event(&ready).payload();
        assert_eq!(payload["embeds"][0]["title"], "機器人上線");
        assert_eq!(payload["embeds"][0]["fields"][0]["value"], "1");
        assert_eq!(payload["allowed_mentions"]["parse"], json!([]));
        assert!(payload["embeds"][0].get("thumbnail").is_none());

        ready.user.avatar = Some("0123456789abcdef0123456789abcdef".parse().unwrap());
        let payload = Notification::from_event(&ready).payload();
        assert_eq!(payload["embeds"][0]["thumbnail"]["url"], ready.user.avatar_url().unwrap());
        ready.guilds.clear();
        assert_eq!(Notification::from_event(&ready).payload()["embeds"][0]["fields"][0]["value"], "0");
    }
}