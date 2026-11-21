use std::time::Duration;
use tokio::time;
use serde_json::json;

use crate::{cache, config::Config, http::HttpClient};

pub async fn background_refresh_task(config: &Config) {
    let mut interval = time::interval(Duration::from_secs(23 * 60 * 60));

    loop {
        interval.tick().await;

        let response = HttpClient::get()
            .client
            .post("")
            .json(&json!({
                "id": config.client_config.id,
                "secret": config.client_config.secret,
            }))
            .send()
            .await;

        match response {
            Ok(response) => {
                let status = response.status();

                if status.as_u16() != 200 {
                    eprintln!("HTTP Error: {}", status);
                    continue;
                }

                let body: serde_json::Value = match response.json().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("Failed to read response: {}", error);
                        continue;
                    }
                };

                let Some(key) = body.get("key").and_then(|v| v.as_str()) else {
                    eprintln!("Response missing key");
                    continue;
                };

                let value = cache::CacheValue::Text(key.to_string());

                let _ = cache::push_value("key", &value).await;

                println!("Key refreshed");
            }
            Err(error) => {
                eprintln!("HTTP Request Failed: {}", error);
            }
        }
    }
}