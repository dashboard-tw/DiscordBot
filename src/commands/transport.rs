use super::datasets::send_dataset;
use crate::bot::{Context, Error};
use crate::http::datasets::Endpoint;
#[poise::command(
    slash_command,
    subcommands(
        "freeway_traffic",
        "fuel_price",
        "fuel_station",
        "rail_alert",
        "train",
        "bike",
        "charging",
        "flight",
        "traffic_accident"
    )
)]
pub async fn transport(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command)]
pub async fn freeway_traffic(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "freeway_traffic",
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
pub async fn fuel_price(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "fuel_price", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn fuel_station(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "fuel_station", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn rail_alert(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "rail_alert", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn train(
    ctx: Context<'_>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "tra_live",
        if stations.unwrap_or(false) {
            Endpoint::Sites
        } else {
            Endpoint::Latest
        },
        None,
        site,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn bike(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "bike_availability",
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
pub async fn charging(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "ev_live",
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
pub async fn flight(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "flight",
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
pub async fn traffic_accident(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "traffic_accident",
        Endpoint::Latest,
        county,
        None,
        page,
    )
    .await
}
pub fn command() -> poise::Command<crate::bot::Data, Error> {
    let mut command = transport();
    command.description = Some("交通資訊查詢".into());
    for subcommand in &mut command.subcommands {
        subcommand.description = Some(
            match subcommand.name.as_ref() {
                "freeway_traffic" => "查詢高速公路即時路況（路段旅行速率）",
                "fuel_price" => "查詢中油汽柴油參考牌價（每週）",
                "fuel_station" => "查詢台灣中油加油站",
                "rail_alert" => "查詢臺鐵與高鐵營運通阻",
                "train" => "查詢臺鐵列車即時位置與誤點",
                "bike" => "查詢公共自行車即時車位（YouBike）",
                "charging" => "查詢電動車充電站即時狀態",
                "flight" => "查詢機場航班動態（全國各機場）",
                "traffic_accident" => "查詢A1 類道路交通事故",
                _ => "資訊查詢",
            }
            .into(),
        );
    }
    command
}
