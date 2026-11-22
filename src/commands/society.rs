use super::datasets::send_dataset;
use crate::bot::{Context, Error};
use crate::http::datasets::Endpoint;
#[poise::command(
    slash_command,
    subcommands(
        "population_year",
        "vital_month",
        "econ_indicator",
        "income_town",
        "school",
        "childcare_center",
        "tourism_attraction",
        "tourism_hotel",
        "shelter_animal",
        "shelter_stats"
    )
)]
pub async fn society(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command)]
pub async fn population_year(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "population_year", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn vital_month(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "vital_month", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn econ_indicator(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "econ_indicator", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn income_town(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "income_town", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn school(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "school", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn childcare_center(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "childcare_center",
        Endpoint::Latest,
        county,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn tourism_attraction(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "tourism_attraction",
        Endpoint::Latest,
        county,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn tourism_hotel(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "tourism_hotel", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn shelter_animal(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "shelter_animal", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn shelter_stats(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "shelter_stats", Endpoint::Latest, county, None, page).await
}
pub fn command() -> poise::Command<crate::bot::Data, Error> {
    let mut command = society();
    command.description = Some("人口與社會資訊查詢".into());
    for subcommand in &mut command.subcommands {
        subcommand.description = Some(
            match subcommand.name.as_ref() {
                "population_year" => "查詢各縣市年底戶數與人口數",
                "vital_month" => "查詢各縣市每月出生、死亡、結婚與離婚",
                "econ_indicator" => "查詢經濟指標（消費者物價、失業率、景氣對策信號）",
                "income_town" => "查詢綜合所得稅所得總額（各鄉鎮市區）",
                "school" => "查詢各級學校名錄",
                "childcare_center" => "查詢托嬰中心",
                "tourism_attraction" => "查詢觀光景點",
                "tourism_hotel" => "查詢旅館民宿",
                "shelter_animal" => "查詢公立動物收容所開放認養動物",
                "shelter_stats" => "查詢全國公立動物收容所收容處理情形統計",
                _ => "資訊查詢",
            }
            .into(),
        );
    }
    command
}
