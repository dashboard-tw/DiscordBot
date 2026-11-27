use std::time::Instant;

use poise::CreateReply;

use crate::bot::{Context, Error};

#[doc = "檢測discord的延遲"]
#[poise::command(prefix_command, slash_command, aliases("latency"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.say("Caculating...").await?;
    let rest_latency = start.elapsed().as_millis();             // use millis
    let content = format!("REST API: {}ms", rest_latency);
    let builder = CreateReply::new().content(content);
    msg.edit(ctx, builder).await?;
    Ok(())
}
