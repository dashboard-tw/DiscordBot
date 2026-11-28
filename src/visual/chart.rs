use plotters::prelude::*;
use poise::{serenity_prelude::{CreateAttachment, CreateEmbed}};
use serde_json::Value;

use crate::{
    http::datasets::{DatasetResponse, Endpoint},
    logger,
    commands::presentation::{presentation},
};

// Some data problem still happened

pub fn shorten(text: &str, limit: usize) -> String {
    let text = text
        .replace(['`', '*', '_', '~', '|'], "")
        .replace('@', "＠");
    let mut result: String = text.chars().take(limit).collect();
    if text.chars().count() > limit {
        result.push('…');
    }
    result
}

pub fn image_key(
    dataset: &str,
    endpoint: Endpoint,
    fields: &[(&'static str, &'static str, &'static str)],
) -> Option<&'static str> {
    if matches!(endpoint, Endpoint::Sites) {
        None
    } else if fields.iter().any(|(key, _, _)| *key == "image_url") {
        Some("image_url")
    } else if matches!(
        dataset,
        "radar_image" | "satellite_image" | "radar_layer" | "cloud_layer" | "wind_layer"
    ) {
        Some("url")
    } else {
        None
    }
}

pub fn display_value(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(s) if s.trim().is_empty() => None,
        Value::String(s) => Some(shorten(s, 100)),
        Value::Bool(b) => Some(if *b { "是" } else { "否" }.to_owned()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

pub fn generated_chart(
    embed: &CreateEmbed,
    dataset: &str,
    endpoint: Endpoint,
    response: &DatasetResponse<Value>,
    page: u32,
) -> Result<Option<CreateAttachment>, Box<dyn std::error::Error>> {
    let embed_value = serde_json::to_value(embed)?;
    if embed_value.pointer("/image/url").is_some() {
        logger::info!("chart skipped: embed already has image");
        return Ok(None);
    }

    let info = presentation(dataset);
    let fields = if matches!(endpoint, Endpoint::Sites) {
        info.site_fields
    } else {
        info.fields
    };
    let Some(rows) = response.body["data"].as_array() else {
        return Ok(None);
    };
    let page_size = if image_key(dataset, endpoint, info.fields).is_some() {
        1
    } else {
        3
    };
    let start = (page as usize - 1) * page_size;
    let shown_rows: Vec<_> = rows.iter().take(24).collect();

    let Some((label, unit, points)) = fields.iter().find_map(|(key, label, unit)| {
        if unit.is_empty() {
            return None;
        }
        let points: Vec<_> = shown_rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                let value = row.get(*key)?;
                let value = match value {
                    Value::Number(number) => number.as_f64()?,
                    Value::String(text) => text.parse().ok()?,
                    _ => return None,
                };
                if !value.is_finite() {
                    return None;
                }
                let label = [
                    "name",
                    "station_name",
                    "site_name",
                    "county_name",
                    "station_id",
                    "site_id",
                    "observed_at",
                    "starts_at",
                    "valid_at",
                ]
                .iter()
                .find_map(|name| row.get(*name).and_then(display_value))
                .unwrap_or_else(|| format!("第{}筆", start + index + 1));
                Some((shorten(&label, 12), value))
            })
            .collect();
        (points.len() >= 2).then_some((*label, *unit, points))
    }) else {
        return Ok(None);
    };

    let width = 1200;
    let height = 630;
    let mut pixels = vec![255; width * height * 3];
    {
        let root = BitMapBackend::with_buffer(&mut pixels, (width as u32, height as u32))
            .into_drawing_area();
        root.fill(&WHITE)?;
        let min_value = points
            .iter()
            .map(|(_, value)| *value)
            .fold(f64::INFINITY, f64::min);
        let max_value = points
            .iter()
            .map(|(_, value)| *value)
            .fold(f64::NEG_INFINITY, f64::max);
        let padding = ((max_value - min_value) * 0.1).max(1.0);
        let y_min = if min_value >= 0.0 {
            0.0
        } else {
            min_value - padding
        };
        let y_max = max_value + padding;
        let chart_title = format!("{}｜{}（{}）", presentation(dataset).title, label, unit);
        let labels: Vec<_> = points.iter().map(|(label, _)| label.clone()).collect();
        let mut chart = ChartBuilder::on(&root)
            .caption(chart_title, ("sans-serif", 32))
            .margin(20)
            .x_label_area_size(55)
            .y_label_area_size(75)
            .build_cartesian_2d(0..points.len() as i32, y_min..y_max)?;

        chart
            .configure_mesh()
            .x_labels(points.len())
            .x_label_formatter(&|index| {
                labels
                    .get((*index).max(0) as usize)
                    .cloned()
                    .unwrap_or_default()
            })
            .y_desc(format!("{label}（{unit}）"))
            .draw()?;
        chart.draw_series(LineSeries::new(
            points
                .iter()
                .enumerate()
                .map(|(index, (_, value))| (index as i32, *value)),
            BLUE.stroke_width(3),
        ))?;
        chart.draw_series(points.iter().enumerate().map(|(index, (_, value))| {
            Circle::new((index as i32, *value), 5, BLUE.filled())
        }))?;
        root.present()?;
    }

    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&pixels)?;
    drop(writer);
    Ok(Some(CreateAttachment::bytes(bytes, "dataset-chart.png")))
}