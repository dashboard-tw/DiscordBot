use crate::{
    bot::Error,
    cache::{self, TokenAnswer},
    config::{self, Config},
    http::{HttpClient, webhook::WebhookPayload},
};

pub async fn access_token() -> Result<String, Error> {
    static LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    let _guard = LOCK.lock().await;
    {
        let shared = cache::get();
        let cached = shared.read().await;
        if let (Some(token), Some(updated)) = (&cached.token, cached.token_updated_at) {
            if updated.elapsed().as_secs() < token.expires_in.saturating_sub(60) {
                return Ok(token.access_token.clone());
            }
        }
    }
    Ok(refresh_token(Config::get()).await?.access_token)
}

pub async fn fetch_token(config: &Config) -> Result<TokenAnswer, Error> {
    fetch_token_from(config, config::BASE_URL).await
}

async fn fetch_token_from(config: &Config, base_url: &str) -> Result<TokenAnswer, Error> {
    let response = HttpClient::get()
        .client
        .post(format!("{}/auth/token", base_url.trim_end_matches('/')))
        .timeout(std::time::Duration::from_secs(30))
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", config.client_config.id.as_str()),
            ("client_secret", config.client_config.secret.as_str()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get token: HTTP {}", response.status()).into());
    }

    let token = response.json::<TokenAnswer>().await?;
    if token.access_token.trim().is_empty() || token.expires_in == 0 {
        return Err("Token response contains an empty token or zero lifetime".into());
    }
    Ok(token)
}

pub async fn refresh_token(config: &Config) -> Result<TokenAnswer, Error> {
    refresh_token_from(config, config::BASE_URL).await
}

async fn refresh_token_from(config: &Config, base_url: &str) -> Result<TokenAnswer, Error> {
    let token = fetch_token_from(config, base_url).await?;
    cache::update_token(&token.access_token, token.expires_in).await?;
    let message = refresh_notification(&token);
    println!("{message}");
    if let Some(url) = config.webhook_url.as_deref() {
        if let Err(error) = WebhookPayload::new().content(format!("```text\n{message}\n```")).send(url).await {
            eprintln!("Token refresh webhook notification failed: {error}");
        }
    }
    Ok(token)
}

fn mask_token(token: &str) -> String {
    let length = token.chars().count();
    let visible = if length > 4 { 4 } else { 0 };
    token.chars().take(visible).collect::<String>() + &"*".repeat(length - visible)
}

fn refresh_notification(token: &TokenAnswer) -> String {
    format!("Token refreshed: {} | expires in {} seconds", mask_token(&token.access_token), token.expires_in)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_mask_only_exposes_prefix_and_hides_short_tokens() {
        assert_eq!(mask_token("abcdefghij"), "abcd******");
        assert_eq!(mask_token("abcde"), "abcd*");
        assert_eq!(mask_token("abcd"), "****");
        assert_eq!(mask_token("a"), "*");
        assert_eq!(mask_token(""), "");
        assert_eq!(mask_token("甲乙丙丁戊己"), "甲乙丙丁**");
    }

    #[test]
    fn refresh_log_and_webhook_message_never_include_full_token() {
        let token: TokenAnswer = serde_json::from_value(serde_json::json!({
            "access_token": "abcd-secret-token", "expires_in": 3600, "token_type": "Bearer",
        })).unwrap();
        let message = refresh_notification(&token);
        assert!(message.contains("abcd*************"));
        assert!(message.contains("3600"));
        assert!(!message.contains(&token.access_token));
        assert!(!message.contains("secret"));
    }
}
