use crate::{
    bot::Error,
    cache::{self, TokenAnswer},
    config::{self, Config},
    http::HttpClient,
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
    Ok(token)
}
