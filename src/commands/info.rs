
use poise::{CreateReply, serenity_prelude::{Color, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter}};

use crate::{bot::{Context, Error}, config::{BASE_URL, GITHUB_URL}};

#[doc = "查看機器人的基本資訊"]
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let dash_url = format!("{}", BASE_URL.strip_suffix("api/v1").unwrap_or_default());
    let embed = CreateEmbed::new().title("機器人資訊")
        .description("用政府開放資料，看全臺各縣市的即時狀況。\n臺灣儀表板是雲際科技工作室的公益專案。我們把各機關的開放資料整理成圖表與地圖，涵蓋天氣、空品、水情、電力、交通與防災，並長期保存歷史資料。")
        .color(Color::new(0x1B96DE))
        .url(&dash_url)
        .thumbnail(format!("{}/logo.png", &dash_url))
        .footer(CreateEmbedFooter::new("臺灣儀表板｜dashboard.tw"));

    let components = vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new_link(dash_url)
                .label("dashboard.tw"),
            CreateButton::new_link(GITHUB_URL)
                .label("GitHub"),
        ])
    ];
    let builder = CreateReply::new().embed(embed).components(components);
    ctx.send(builder).await?;
    Ok(())
}
