use std::time::Duration;
use tokio::time;

use crate::{config::Config, token, logger};

pub async fn background_refresh_task(config: &Config) {
    let mut interval = time::interval(Duration::from_secs(23 * 60 * 60));

    loop {
        interval.tick().await;

        match token::refresh_token(config).await {
            Ok(_) => {}, // logger::info!("Token refreshed; expires in {} seconds", token.expires_in)
            Err(err) => logger::error!("Token refresh failed: {err}"),
        }
    }
}
