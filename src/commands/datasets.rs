use super::presentation::presentation;
use crate::{
    bot::{Context, Error},
    http::datasets::{DatasetResponse, Endpoint, Query},
    logger,
    visual::chart::{image_key, generated_chart, shorten, display_value},
    config::GITHUB_URL
};
use poise::{
    CreateReply,
    serenity_prelude::{CreateAllowedMentions, CreateAttachment, CreateEmbed, CreateEmbedFooter, Colour},
};
use serde_json::Value;

// rust test
#[cfg(test)]
mod tests;

fn county_code(input: Option<String>) -> Result<Option<String>, &'static str> {
    let Some(input) = input
        .map(|s| s.trim().replace('台', "臺"))
        .filter(|s| !s.is_empty())
    else {
        return Ok(None);
    };
    let counties = [
        ("臺北市", "63000"),
        ("新北市", "65000"),
        ("桃園市", "68000"),
        ("臺中市", "66000"),
        ("臺南市", "67000"),
        ("高雄市", "64000"),
        ("基隆市", "10017"),
        ("新竹市", "10018"),
        ("新竹縣", "10004"),
        ("苗栗縣", "10005"),
        ("彰化縣", "10007"),
        ("南投縣", "10008"),
        ("雲林縣", "10009"),
        ("嘉義市", "10020"),
        ("嘉義縣", "10010"),
        ("屏東縣", "10013"),
        ("宜蘭縣", "10002"),
        ("花蓮縣", "10015"),
        ("臺東縣", "10014"),
        ("澎湖縣", "10016"),
        ("金門縣", "09020"),
        ("連江縣", "09007"),
    ];
    counties
        .iter()
        .find(|(name, code)| input == *name || input == *code)
        .map(|(_, code)| Some((*code).to_owned()))
        .ok_or("請輸入有效的縣市名稱或代碼，例如臺北市、63000。")
}

pub fn build_embed(
    dataset: &str,
    endpoint: Endpoint,
    response: &DatasetResponse<Value>,
    page: u32,
) -> Result<CreateEmbed, &'static str> {
    let info = presentation(dataset);
    let rows = response.body["data"]
        .as_array()
        .ok_or("目前無法解析資料，請稍後再試。")?;
    let image_key = image_key(dataset, endpoint, info.fields);
    let page_size = if image_key.is_some() { 1 } else { 3 };
    let pages = rows.len().div_ceil(page_size).max(1);
    if page == 0 || page as usize > pages {
        return Err("頁碼超出範圍，請從第1頁開始查詢。");
    }
    let start = (page as usize - 1) * page_size;
    let shown = rows.len().saturating_sub(start).min(page_size);
    let mut description = if rows.is_empty() {
        "目前沒有符合條件的資料。".to_owned()
    } else {
        format!(
            "共 {} 筆，本頁顯示第 {}–{} 筆。\n可使用 page 參數切換頁碼。",
            rows.len(),
            start + 1,
            start + shown
        )
    };
    if response
        .body
        .pointer("/meta/next_cursor")
        .and_then(Value::as_str)
        .is_some()
    {
        description.push_str("\n來源尚有後續資料，本次僅展示已取得的第一批資料。");
    }
    if let Some(time) = response
        .body
        .pointer("/meta/last_observed_at")
        .and_then(Value::as_str)
    {
        description.push_str(&format!("\n資料時間：{}", shorten(time, 60)));
    }
    if response
        .body
        .pointer("/meta/status")
        .and_then(Value::as_str)
        .is_some_and(|s| s != "ok")
    {
        description.push_str("\n⚠️ 來源可能尚未更新，請留意資料時間。");
    }
    let agency = response
        .body
        .pointer("/meta/source/agency")
        .and_then(Value::as_str)
        .unwrap_or("臺灣儀表板");
    let license = response
        .body
        .pointer("/meta/source/license")
        .and_then(Value::as_str)
        .unwrap_or("依來源授權條款");
    let cache = if response.cached {
        format!("快取於{}秒前取得", response.age.as_secs())
    } else {
        "剛剛更新".to_owned()
    };
    let suffix = if matches!(endpoint, Endpoint::Sites) {
        "｜測站清單"
    } else {
        ""
    };
    let mut embed = CreateEmbed::new()
        .title(format!("{}{suffix}", info.title))
        .url("https://dashboard.tw")
        .colour(0x3498db)
        .description(description)
        .footer(CreateEmbedFooter::new(format!(
            "第{page}/{pages}頁・{cache}・來源：{}｜{}",
            shorten(agency, 70),
            shorten(license, 100)
        )));
    let fields = if matches!(endpoint, Endpoint::Sites) {
        info.site_fields
    } else {
        info.fields
    };
    for (offset, row) in rows.iter().skip(start).take(page_size).enumerate() {
        if let Some(url) = image_key.and_then(|key| row.get(key)).and_then(Value::as_str).filter(|url| !url.trim().is_empty()) {
            let base = reqwest::Url::parse("https://dashboard.tw").expect("Invalid image base URL");
            if let Ok(url) = base.join(url.trim()) {
                if matches!(url.scheme(), "http" | "https") && url.host_str().is_some() {
                    embed = embed.image(url.as_str());
                }
            }
        }
        let name = [
            "name",
            "station_name",
            "site_name",
            "headline",
            "location",
            "area",
            "station_id",
            "site_id",
        ]
        .iter()
        .find_map(|key| row.get(*key).and_then(display_value))
        .unwrap_or_else(|| format!("第{}筆資訊", start + offset + 1));
        let mut lines = Vec::new();
        let mut used = 0;
        for &(key, label, unit) in fields {
            if Some(key) == image_key {
                continue;
            }
            let Some(value) = row.get(key).and_then(display_value) else {
                continue;
            };
            let label = shorten(label.split('（').next().unwrap_or(label), 28);
            let line = format!(
                "**{label}**：{value}{}",
                if unit.is_empty() {
                    String::new()
                } else {
                    format!(" {}", shorten(unit, 20))
                }
            );
            let length = line.encode_utf16().count() + 1;
            if used + length > 850 {
                lines.push("更多資訊請縮小查詢範圍。".to_owned());
                break;
            }
            used += length;
            lines.push(line);
        }
        embed = embed.field(
            shorten(&name, 100),
            if lines.is_empty() {
                "目前沒有可顯示的資訊。".to_owned()
            } else {
                lines.join("\n")
            },
            false,
        );
    }
    Ok(embed)
}

async fn attach_image(embed: CreateEmbed) -> Result<(CreateEmbed, Option<poise::serenity_prelude::CreateAttachment>), Error> {
    let value = serde_json::to_value(&embed)?;
    let image_url = value.pointer("/image/url").and_then(Value::as_str);
    let mut file = None;
    let embed = if let Some(url) = image_url.filter(|url| !url.starts_with("attachment://")) {
        match crate::http::image::download(url).await {
            Ok(attachment) => {
                let embed = embed.image(format!("attachment://{}", attachment.filename));
                file = Some(attachment);
                embed
            }
            Err(error) => {
                logger::error!("Dataset image download failed: {error}");
                let mut value = value;
                value.as_object_mut().expect("Embed must be an object").remove("image");
                poise::serenity_prelude::CreateEmbed::from(serde_json::from_value::<poise::serenity_prelude::Embed>(value)?)
            }
        }
    } else {
        embed
    };
    Ok((embed, file))
}

async fn build_reply(
    embed: CreateEmbed,
    generated_chart: Option<CreateAttachment>,
) -> Result<CreateReply, Error> {
    let (embed, attachment) = attach_image(embed).await?;
    let mut reply = CreateReply::new().embed(embed).allowed_mentions(CreateAllowedMentions::new());
    if let Some(attachment) = attachment.or(generated_chart) {
        reply = reply.attachment(attachment);
    }
    Ok(reply)
}

// Image dataset haven't set up
pub async fn send_dataset(
    ctx: Context<'_>,
    dataset: &str,
    endpoint: Endpoint,
    county: Option<String>,
    site: Option<String>,
    page: Option<u32>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let county = match county_code(county) {
        Ok(county) => county,
        Err(message) => {
            ctx.say(message).await?;
            return Ok(());
        }
    };
    let site = site.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty());
    if site
        .as_ref()
        .is_some_and(|s| s.chars().count() > 100 || s.chars().any(char::is_control))
        || page == Some(0)
    {
        ctx.say("測站代碼錯誤。").await?;
        return Ok(());
    }
    let site = if matches!(endpoint, Endpoint::Sites) {
        None
    } else {
        site
    };
    match ctx
        .data()
        .datasets
        .request::<Value>(dataset, endpoint, Query { county, site })
        .await
    {
        Ok(response) => match build_embed(dataset, endpoint, &response, page.unwrap_or(1)) {
            Ok(embed) => {
                let page = page.unwrap_or(1);
                let generated_chart =
                    match generated_chart(&embed, dataset, endpoint, &response, page) {
                        Ok(chart) => {
                            logger::info!("Generated chart: is_some={}", chart.is_some());
                            chart
                        },
                        Err(error) => {
                            logger::error!("Dataset chart generation failed: {error}");
                            None
                        }
                    };
                let mut embed = embed;
                if generated_chart.is_some() {
                    logger::info!("Embed image bytes");
                    embed = embed.image("attachment://dataset-chart.png");
                }
                ctx.send(build_reply(embed, generated_chart).await?).await?;
            }
            Err(message) => {
                let embed = CreateEmbed::new().title("發生錯誤")
                    .description(format!("```\n{}```", message))
                    .color(Colour::new(0xDA373C))
                    .url(GITHUB_URL);
                let builder = CreateReply::new().embed(embed);
                ctx.send(builder).await?;
            }
        },
        Err(error) => {
            logger::error!("Dataset {dataset}/{} failed: {error}", endpoint.as_str());
            ctx.say(
                "目前無法取得資訊，可能是來源服務暫時異常或查詢條件不受支援，請稍後再試。",
            )
            .await?;
        }
    }
    Ok(())
}
