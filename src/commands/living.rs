use super::datasets::send_dataset;
use crate::bot::{Context, Error};
use crate::http::datasets::Endpoint;
#[poise::command(
    slash_command,
    subcommands(
        "water_outage",
        "water_outage_taipei",
        "power_outage",
        "power_gen",
        "power_region",
        "power_load",
        "power_peak_daily",
        "power_supply_daily",
        "power_supply_forecast",
        "medical_facility",
        "ltc_facility",
        "farm_price",
        "poultry_price",
        "hog_trade",
        "food_rejection",
        "real_estate",
        "dengue_daily",
        "nhi_visit"
    )
)]
pub async fn living(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command)]
pub async fn water_outage(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "water_outage", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn water_outage_taipei(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "water_outage_taipei",
        Endpoint::Latest,
        county,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn power_outage(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "power_outage", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn power_gen(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "測站或觀測點代碼"] site: Option<String>,
    #[description = "顯示測站清單以查找代碼"] stations: Option<bool>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "power_gen",
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
pub async fn power_region(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "power_region", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn power_load(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_dataset(ctx, "power_load", Endpoint::Latest, None, None, None).await
}
#[poise::command(slash_command)]
pub async fn power_peak_daily(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "power_peak_daily", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn power_supply_daily(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "power_supply_daily",
        Endpoint::Latest,
        None,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn power_supply_forecast(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "power_supply_forecast",
        Endpoint::Latest,
        None,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn medical_facility(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(
        ctx,
        "medical_facility",
        Endpoint::Latest,
        county,
        None,
        page,
    )
    .await
}
#[poise::command(slash_command)]
pub async fn ltc_facility(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "ltc_facility", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn farm_price(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "farm_price", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn poultry_price(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "poultry_price", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn hog_trade(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "hog_trade", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn food_rejection(
    ctx: Context<'_>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "food_rejection", Endpoint::Latest, None, None, page).await
}
#[poise::command(slash_command)]
pub async fn real_estate(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "real_estate", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn dengue_daily(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "dengue_daily", Endpoint::Latest, county, None, page).await
}
#[poise::command(slash_command)]
pub async fn nhi_visit(
    ctx: Context<'_>,
    #[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,
    #[description = "顯示頁碼"] page: Option<u32>,
) -> Result<(), Error> {
    send_dataset(ctx, "nhi_visit", Endpoint::Latest, county, None, page).await
}
pub fn command() -> poise::Command<crate::bot::Data, Error> {
    let mut command = living();
    command.description = Some("民生資訊查詢".into());
    for subcommand in &mut command.subcommands {
        subcommand.description = Some(
            match subcommand.name.as_ref() {
                "water_outage" => "查詢台灣自來水停水資訊",
                "water_outage_taipei" => "查詢臺北自來水停水公告",
                "power_outage" => "查詢台電停電資訊（災害期間）",
                "power_gen" => "查詢各機組發電量（每 10 分鐘）",
                "power_region" => "查詢各區發電與用電（每 10 分鐘）",
                "power_load" => "查詢今日用電量與供電能力（每 10 分鐘）",
                "power_peak_daily" => "查詢昨日尖峰備轉（每天）",
                "power_supply_daily" => "查詢過去電力供需（每日）",
                "power_supply_forecast" => "查詢未來二個月電力供需預測",
                "medical_facility" => "查詢健保特約醫事機構（醫院、診所、藥局）",
                "ltc_facility" => "查詢長照服務據點（ABC 據點）",
                "farm_price" => "查詢果菜花卉批發市場交易行情",
                "poultry_price" => "查詢家禽交易行情（雞蛋、白肉雞）",
                "hog_trade" => "查詢毛豬交易行情",
                "food_rejection" => "查詢邊境查驗不合格食品",
                "real_estate" => "查詢不動產實價登錄（買賣、預售屋、租賃）",
                "dengue_daily" => "查詢登革熱每日確定病例",
                "nhi_visit" => "查詢健保門急診與住院就診人次（類流感、腸病毒、新冠）",
                _ => "資訊查詢",
            }
            .into(),
        );
    }
    command
}
