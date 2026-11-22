use poise::serenity_prelude as serenity;
use serde_json::{Value, json};

use crate::{config::Config, http::webhook};

struct Notification {
    joined: bool,
    name: String,
    id: serenity::GuildId,
    icon_url: Option<String>,
    member_count: Option<u64>,
    owner_id: Option<serenity::UserId>,
    owner_name: Option<String>,
}

impl Notification {
    fn from_guild(joined: bool, guild: &serenity::Guild) -> Self {
        Self {
            joined,
            name: guild.name.clone(),
            id: guild.id,
            icon_url: guild.icon_url(),
            member_count: Some(guild.member_count),
            owner_id: Some(guild.owner_id),
            owner_name: guild.members.get(&guild.owner_id).map(|member| member.user.name.clone()),
        }
    }

    fn from_event(event: &serenity::FullEvent) -> Option<Self> {
        match event {
            serenity::FullEvent::GuildCreate { guild, is_new: Some(true) } => Some(Self::from_guild(true, guild)),
            serenity::FullEvent::GuildDelete { incomplete, full } if !incomplete.unavailable => {
                if let Some(guild) = full {
                    return Some(Self::from_guild(false, guild));
                }
                Some(Self {
                    joined: false,
                    name: full.as_ref().map(|guild| guild.name.clone()).unwrap_or_else(|| "未知伺服器".into()),
                    id: incomplete.id,
                    icon_url: None,
                    member_count: None,
                    owner_id: None,
                    owner_name: None,
                })
            }
            _ => None,
        }
    }

    fn payload(&self) -> Value {
        let owner = match self.owner_id {
            Some(id) => match &self.owner_name {
                Some(name) => format!("{name}\n<@{id}>（ID：{id}）"),
                None => format!("<@{id}>（ID：{id}）"),
            },
            None => "未知".into(),
        };
        let mut payload = json!({
            "allowed_mentions": { "parse": [] },
            "embeds": [{
                "title": if self.joined { "機器人加入伺服器" } else { "機器人退出伺服器" },
                "color": if self.joined { 0x57F287 } else { 0xED4245 },
                "fields": [
                    { "name": "伺服器名稱", "value": if self.name.is_empty() { "未知伺服器" } else { &self.name } },
                    { "name": "伺服器 ID", "value": self.id.to_string() },
                    { "name": "成員人數", "value": self.member_count.map(|count| count.to_string()).unwrap_or_else(|| "未知".into()) },
                    { "name": "擁有者", "value": owner },
                ],
            }],
        });
        if let Some(url) = &self.icon_url {
            payload["embeds"][0]["thumbnail"] = json!({ "url": url });
        }
        payload
    }
}

pub async fn handle(event: &serenity::FullEvent) {
    let Some(notification) = Notification::from_event(event) else {
        return;
    };
    println!("{} guild: {} ({})", if notification.joined { "Joined" } else { "Left" }, notification.name, notification.id);

    let Some(url) = Config::get().webhook_url.as_deref() else {
        return;
    };
    if let Err(error) = webhook::send(url, &notification.payload()).await {
        eprintln!("Guild webhook notification failed: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn removal(unavailable: bool) -> serenity::FullEvent {
        serenity::FullEvent::GuildDelete {
            incomplete: serde_json::from_value(json!({
                "id": "123", "unavailable": unavailable,
            })).unwrap(),
            full: None,
        }
    }

    #[test]
    fn guild_removal_notifies_without_cached_guild() {
        let notification = Notification::from_event(&removal(false)).unwrap();
        let payload = notification.payload();
        assert_eq!(payload["embeds"][0]["title"], "機器人退出伺服器");
        assert_eq!(payload["embeds"][0]["fields"][0]["value"], "未知伺服器");
        assert_eq!(payload["embeds"][0]["fields"][1]["value"], "123");
        assert_eq!(payload["allowed_mentions"]["parse"], json!([]));
        assert_eq!(payload["embeds"][0]["fields"][2]["value"], "未知");
        assert_eq!(payload["embeds"][0]["fields"][3]["value"], "未知");
        assert!(payload["embeds"][0].get("thumbnail").is_none());
    }

    #[test]
    fn unavailable_guild_is_not_notified_as_removal() {
        assert!(Notification::from_event(&removal(true)).is_none());
    }

    #[test]
    fn join_payload_contains_guild_details() {
        let notification = Notification {
            joined: true,
            name: "測試伺服器".into(),
            id: serenity::GuildId::new(456),
            icon_url: Some("https://cdn.discordapp.com/icons/456/icon.png".into()),
            member_count: Some(42),
            owner_id: Some(serenity::UserId::new(789)),
            owner_name: Some("測試擁有者".into()),
        };
        let payload = notification.payload();
        assert_eq!(payload["embeds"][0]["title"], "機器人加入伺服器");
        assert_eq!(payload["embeds"][0]["fields"][0]["value"], "測試伺服器");
        assert_eq!(payload["embeds"][0]["fields"][1]["value"], "456");
        assert_eq!(payload["embeds"][0]["fields"][2]["value"], "42");
        assert_eq!(payload["embeds"][0]["fields"][3]["value"], "測試擁有者\n<@789>（ID：789）");
        assert_eq!(payload["embeds"][0]["thumbnail"]["url"], "https://cdn.discordapp.com/icons/456/icon.png");
    }

    #[test]
    fn cached_guild_details_are_used_for_join_and_removal() {
        let mut guild = serenity::Guild::default();
        guild.id = serenity::GuildId::new(456);
        guild.name = "快取伺服器".into();
        guild.member_count = 42;
        guild.owner_id = serenity::UserId::new(789);
        guild.icon = Some("0123456789abcdef0123456789abcdef".parse().unwrap());
        let joined = serenity::FullEvent::GuildCreate { guild: guild.clone(), is_new: Some(true) };
        let mut removed = removal(false);
        if let serenity::FullEvent::GuildDelete { full, .. } = &mut removed {
            *full = Some(guild.clone());
        }
        for event in [joined, removed] {
            let notification = Notification::from_event(&event).unwrap();
            assert_eq!(notification.member_count, Some(42));
            assert_eq!(notification.owner_id, Some(guild.owner_id));
            assert_eq!(notification.icon_url, guild.icon_url());
            assert_eq!(notification.payload()["embeds"][0]["fields"][3]["value"], "<@789>（ID：789）");
        }
        let existing = serenity::FullEvent::GuildCreate { guild, is_new: Some(false) };
        assert!(Notification::from_event(&existing).is_none());
    }
}