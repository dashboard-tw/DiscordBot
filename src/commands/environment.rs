use super::datasets::send_dataset;
use crate::bot::{Context, Error};
use crate::http::datasets::Endpoint;
#[poise::command(
    slash_command,
    subcommands(
        "air_quality",
        "uv",
        "aqi_forecast",
        "reservoir",
        "river_level",
        "river_quality",
        "renewable_month",
        "waste",
        "power_sales"
    )
)]
pub async fn environment(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command)]
pub async fn air_quality(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "aqi_obs",
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
pub async fn uv(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "uv_obs",
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
pub async fn aqi_forecast(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "aqi_forecast", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn reservoir(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "reservoir_obs",
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
pub async fn river_level(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "river_level",
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
pub async fn river_quality(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "river_quality",
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
pub async fn renewable_month(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "renewable_month", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn waste(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "waste", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn power_sales(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "power_sales", Endpoint::Latest, county, None, page).await
}
pub fn command() -> poise::Command<crate::bot::Data, Error> {
    let mut command = environment();
    command.description = Some("環境資訊查詢".into());
    for subcommand in &mut command.subcommands {
        subcommand.description = Some(
            match subcommand.name.as_ref() {
                "air_quality" => "查詢空氣品質指標（每小時）",
                "uv" => "查詢紫外線指數（每小時）",
                "aqi_forecast" => "查詢空氣品質預報（每天三次）",
                "reservoir" => "查詢水庫水情（每小時）",
                "river_level" => "查詢河川即時水位（每 10 分鐘）",
                "river_quality" => "查詢河川水質（河川污染指數）",
                "renewable_month" => "查詢再生能源每月發電量",
                "waste" => "查詢各縣市垃圾與資源回收",
                "power_sales" => "查詢各縣市各部門售電量",
                _ => "資訊查詢",
            }
            .into(),
        );
    }
    command
}
