use poise::serenity_prelude as serenity;

use super::{Data, Error};
use crate::{commands, config::Config, events, tasks, logger};

pub async fn run(config: &'static Config) -> Result<(), Error> {
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let commands =
                    poise::builtins::create_application_commands(&framework.options().commands);

                let result = serenity::Command::set_global_commands(&ctx.http, commands).await?;

                logger::info!("Registered {} slash commands!", result.len());

                Ok(Data::default())
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |framework, event| Box::pin(events::event_handler(framework, event)),
            commands: commands::all(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.bot.prefix.as_str().into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let mut client = serenity::ClientBuilder::new(config.bot.token.as_str(), intents)
        .framework(framework)
        .await?;

    tokio::spawn(async move {
        logger::info!("Refresh task start");
        tasks::refresh::background_refresh_task(config).await;
    });

    client.start().await?;

    Ok(())
}
