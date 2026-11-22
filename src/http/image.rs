use std::time::Duration;

use poise::serenity_prelude::CreateAttachment;

use crate::bot::Error;
use super::HttpClient;

const MAX_IMAGE_BYTES: usize = 8 * 1024 * 1024;

fn image_extension(content_type: &str) -> Option<&'static str> {
    match content_type.split(';').next()?.trim() {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

pub async fn download(url: &str) -> Result<CreateAttachment, Error> {
    let mut response = HttpClient::get().client.get(url)
        .timeout(Duration::from_secs(15)).send().await
        .map_err(|_| "Failed to download image")?
        .error_for_status().map_err(|_| "Image service returned an error")?;
    let extension = response.headers().get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok()).and_then(image_extension)
        .ok_or("Unsupported image content type")?;
    if response.content_length().is_some_and(|size| size > MAX_IMAGE_BYTES as u64) {
        return Err("Image exceeds the 8 MiB size limit".into());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| "Image download interrupted")? {
        if bytes.len() + chunk.len() > MAX_IMAGE_BYTES {
            return Err("Image exceeds the 8 MiB size limit".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    if bytes.is_empty() {
        return Err("Image response is empty".into());
    }
    Ok(CreateAttachment::bytes(bytes, format!("dataset-image.{extension}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn image_download_uses_memory_and_rejects_invalid_responses() {
        use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

        for (headers, body, valid) in [
            ("Content-Type: image/png\r\n", b"image bytes".as_slice(), true),
            ("Content-Type: text/html\r\n", b"not an image".as_slice(), false),
            ("Content-Type: image/png\r\n", b"".as_slice(), false),
            ("Content-Type: image/png\r\nContent-Length: 8388609\r\n", b"".as_slice(), false),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("http://{}/image", listener.local_addr().unwrap());
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = [0; 4096];
                socket.read(&mut request).await.unwrap();
                let response = format!("HTTP/1.1 200 OK\r\n{headers}Connection: close\r\n\r\n");
                socket.write_all(response.as_bytes()).await.unwrap();
                socket.write_all(body).await.unwrap();
            });
            let result = download(&url).await;
            if valid {
                let attachment = result.unwrap();
                assert_eq!(attachment.filename, "dataset-image.png");
                assert_eq!(attachment.data, body);
            } else {
                assert!(result.is_err());
            }
            server.await.unwrap();
        }
        assert_eq!(image_extension("image/jpeg; charset=binary"), Some("jpg"));
        assert_eq!(image_extension("image/svg+xml"), None);
    }
}