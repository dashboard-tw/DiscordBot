use std::time::Duration;
use serde::Serialize;

use super::HttpClient;

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

    #[tokio::test]
    async fn invalid_url_returns_error_without_exposing_credentials() {
        let error = send("invalid://example.test/secret-token", &serde_json::json!({
            "content": "test",
        })).await.unwrap_err();
        assert!(error.url().is_none());
        assert!(!error.to_string().contains("secret-token"));
    }
}