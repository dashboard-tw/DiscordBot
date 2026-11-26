use poise::CreateReply;
use std::time::{Duration, Instant};

use crate::bot::{Context, Error};
use crate::cache::TokenAnswer;
use crate::{config::Config, token, logger};

#[poise::command(prefix_command, slash_command)]
pub async fn refresh(ctx: Context<'_>) -> Result<(), Error> {
    let config = Config::get();
    if !config.is_admin(&ctx.author().id.to_string()) {
        ctx.send(
            CreateReply::new()
                .content("你沒有權限使用此指令")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    ctx.defer_ephemeral().await?;
    let start = Instant::now();
    let result = token::refresh_token(config).await;
    let content = refresh_message(result, start.elapsed());
    ctx.send(CreateReply::new().content(content).ephemeral(true))
        .await?;
    Ok(())
}

fn refresh_message(result: Result<TokenAnswer, Error>, elapsed: Duration) -> String {
    match result {
        Ok(token) => format!(
            "Token已更新並存入快取 有效時間：{} 秒\n-# 耗時：{:.2}ms",
            token.expires_in,
            elapsed.as_secs_f64() * 1000.0
        ),
        Err(err) => {
            logger::error!("Admin token refresh failed: {err}");
            "Token 更新失敗原有快取未變更，請查看伺服器記錄".to_string()
        }
    }
}
