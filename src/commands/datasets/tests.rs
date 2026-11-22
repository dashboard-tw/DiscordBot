use super::*;
use serde_json::json;
use std::time::Duration;

#[tokio::test]
async fn image_reply_uploads_attachment_and_falls_back_to_text() {
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/image", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        socket.read(&mut request).await.unwrap();
        socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: 3\r\nConnection: close\r\n\r\npng").await.unwrap();
    });
    let (embed, attachment) = attach_image(CreateEmbed::new().title("test").image(url)).await.unwrap();
    assert_eq!(attachment.unwrap().data, b"png");
    let embed = serde_json::to_value(embed).unwrap();
    assert_eq!(embed["image"]["url"], "attachment://dataset-image.png");
    server.await.unwrap();

    let (embed, attachment) = attach_image(CreateEmbed::new().title("test").image("invalid://image")).await.unwrap();
    assert!(attachment.is_none());
    let embed = serde_json::to_value(embed).unwrap();
    assert_eq!(embed["title"], "test");
    assert!(embed.get("image").is_none());
}

#[test]
fn single_page_commands_have_no_page_parameter() {
    let commands = crate::commands::all();
    let single_page = ["radar_image", "satellite_image", "radar_layer", "cloud_layer", "wind_layer", "power_load"];
    for name in single_page {
        let command = commands.iter().flat_map(|group| &group.subcommands).find(|command| command.name == name).unwrap();
        assert!(!command.parameters.iter().any(|parameter| parameter.name == "page"), "{name}");
    }
    let observation = commands.iter().flat_map(|group| &group.subcommands).find(|command| command.name == "observation").unwrap();
    assert!(observation.parameters.iter().any(|parameter| parameter.name == "page"));
}

#[test]
fn image_urls_are_rendered_as_images_without_truncation() {
    for (dataset, key, url, expected) in [
        ("radar_image", "url", "/images/radar_test.png", "https://dashboard.tw/images/radar_test.png"),
        ("earthquake", "image_url", "https://example.com/earthquake_test.png", "https://example.com/earthquake_test.png"),
    ] {
        let long_url = format!("{url}?signature={}", "a".repeat(150));
        let response = DatasetResponse {
            body: json!({"data": [{key: long_url, "observed_at": "2026-11-22"}, {key: "/images/second.png"}]}),
            cached: false,
            age: Duration::ZERO,
        };
        let first = serde_json::to_value(build_embed(dataset, Endpoint::Latest, &response, 1).unwrap()).unwrap();
        assert_eq!(first["image"]["url"], format!("{expected}?signature={}", "a".repeat(150)));
        assert!(!first["fields"].to_string().contains("signature="));
        let second = serde_json::to_value(build_embed(dataset, Endpoint::Latest, &response, 2).unwrap()).unwrap();
        assert_eq!(second["image"]["url"], "https://dashboard.tw/images/second.png");
    }
}

#[test]
fn missing_or_unsupported_image_urls_are_not_embedded() {
    for url in [Value::Null, json!("  "), json!("javascript:alert(1)"), json!("file:///image.png")] {
        let response = DatasetResponse {
            body: json!({"data": [{"url": url}]}),
            cached: false,
            age: Duration::ZERO,
        };
        let embed = serde_json::to_value(build_embed("radar_image", Endpoint::Latest, &response, 1).unwrap()).unwrap();
        assert!(embed.get("image").is_none());
    }
}

#[test]
fn county_input_accepts_names_codes_and_empty_values() {
    for input in ["台北市", "臺北市", " 63000 "] {
        assert_eq!(county_code(Some(input.into())), Ok(Some("63000".into())));
    }
    assert_eq!(county_code(None), Ok(None));
    assert_eq!(county_code(Some("  ".into())), Ok(None));
    assert!(county_code(Some("invalid".into())).is_err());
}

#[test]
fn embed_pagination_selects_rows_and_rejects_out_of_range_pages() {
    let rows: Vec<_> = (1..=4)
        .map(|id| json!({"station_id": format!("station-{id}"), "temp_c": 25.5}))
        .collect();
    let response = DatasetResponse {
        body: json!({"data": rows}),
        cached: true,
        age: Duration::from_secs(30),
    };

    let first =
        serde_json::to_value(build_embed("weather_obs", Endpoint::Latest, &response, 1).unwrap())
            .unwrap();
    assert_eq!(first["fields"].as_array().unwrap().len(), 3);
    assert_eq!(first["fields"][0]["name"], "station-1");
    assert_eq!(first["fields"][2]["name"], "station-3");

    let second =
        serde_json::to_value(build_embed("weather_obs", Endpoint::Latest, &response, 2).unwrap())
            .unwrap();
    assert_eq!(second["fields"].as_array().unwrap().len(), 1);
    assert_eq!(second["fields"][0]["name"], "station-4");
    assert!(
        second["fields"][0]["value"]
            .as_str()
            .unwrap()
            .contains("25.5")
    );
    assert!(
        second["footer"]["text"]
            .as_str()
            .unwrap()
            .contains("第2/2頁")
    );
    assert!(build_embed("weather_obs", Endpoint::Latest, &response, 0).is_err());
    assert!(build_embed("weather_obs", Endpoint::Latest, &response, 3).is_err());
}
