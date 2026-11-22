use super::datasets::send_dataset;
use crate::bot::{Context, Error};
use crate::http::datasets::Endpoint;
#[poise::command(
    slash_command,
    subcommands(
        "typhoon_track",
        "typhoon_warning",
        "weather_warning",
        "rain",
        "observation",
        "forecast",
        "earthquake",
        "earthquake_local",
        "disaster_alert",
        "debris_alert",
        "flood_sensor",
        "work_closure",
        "shelter",
        "shelter_report",
        "radar_image",
        "satellite_image",
        "radar_layer",
        "cloud_layer",
        "wind_layer",
        "fire_incident"
    )
)]
pub async fn weather(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command)]
pub async fn typhoon_track(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "typhoon_track", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn typhoon_warning(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "typhoon_warning", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn weather_warning(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "weather_warning", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn rain(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "rain_obs",
        if stations.unwrap_or(false) {
            Endpoint::Sites
        } else {
            Endpoint::Latest
        },
        county,
        site,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn observation(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "weather_obs",
        if stations.unwrap_or(false) {
            Endpoint::Sites
        } else {
            Endpoint::Latest
        },
        county,
        site,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn forecast(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "county_forecast", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn earthquake(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "earthquake", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn earthquake_local(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "earthquake_local", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn disaster_alert(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "disaster_alert", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn debris_alert(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "debris_alert", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn flood_sensor(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "flood_sensor",
        if stations.unwrap_or(false) {
            Endpoint::Sites
        } else {
            Endpoint::Latest
        },
        county,
        site,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn work_closure(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "work_closure", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn shelter(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "shelter", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn shelter_report(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "shelter_report", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn radar_image(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "radar_image", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn satellite_image(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "satellite_image", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn radar_layer(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "radar_layer", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn cloud_layer(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "cloud_layer", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn wind_layer(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "wind_layer", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn fire_incident(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "fire_incident", Endpoint::Latest, county, None, page).await
}
pub fn command() -> poise::Command<crate::bot::Data, Error> {
    let mut command = weather();
    command.description = Some("天氣與防災資訊查詢".into());
    for subcommand in &mut command.subcommands {
        subcommand.description = Some(
            match subcommand.name.as_ref() {
                "typhoon_track" => "查詢颱風路徑（熱帶氣旋）",
                "typhoon_warning" => "查詢颱風警報",
                "weather_warning" => "查詢天氣警特報（各縣市）",
                "rain" => "查詢雨量（每 10 分鐘）",
                "observation" => "查詢天氣觀測（氣溫、濕度、風）",
                "forecast" => "查詢縣市 36 小時天氣預報",
                "earthquake" => "查詢顯著有感地震報告",
                "earthquake_local" => "查詢小區域有感地震報告",
                "disaster_alert" => "查詢防災示警（土石流、淹水、道路封閉、水庫放流）",
                "debris_alert" => "查詢土石流及大規模崩塌警戒",
                "flood_sensor" => "查詢淹水感測器即時淹水深度",
                "work_closure" => "查詢天然災害停止上班及上課情形",
                "shelter" => "查詢避難收容處所與開設狀態",
                "shelter_report" => "查詢收容所開設通報（災害期間）",
                "radar_image" => "查詢雷達合成回波圖（臺灣）",
                "satellite_image" => "查詢衛星雲圖（東亞紅外線彩色）",
                "radar_layer" => "查詢雷達回波地圖圖層",
                "cloud_layer" => "查詢動態雲圖（向日葵衛星紅外線）",
                "wind_layer" => "查詢風場（數值預報 10 公尺風）",
                "fire_incident" => "查詢火災事件",
                _ => "資訊查詢",
            }
            .into(),
        );
    }
    command
}
