use poise::serenity_prelude as serenity;

use super::{Data, Error};
use crate::{commands, config::Config, events, tasks};

pub async fn run(config: Config) -> Result<(), Error> {
    let task_config = config.clone();

    let intents =
        serenity::GatewayIntents::non_privileged()
            | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let commands =
                    poise::builtins::create_application_commands(
                        &framework.options().commands
                    );

                let result =
                    serenity::Command::set_global_commands(&ctx.http, commands).await?;

                println!("Registered {} slash commands!", result.len());

                Ok(Data::default())
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |framework, event| {
                Box::pin(events::event_handler(framework, event))
            },
            commands: commands::all(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.prefix.into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let mut client = serenity::ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await?;

    tokio::spawn(async move {
        tasks::refresh::background_refresh_task(&task_config).await;
    });

    client.start().await?;

    Ok(())
}