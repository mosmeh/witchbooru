use witchbooru::image::{self, DynamicImage};

use anyhow::anyhow;
use futures::StreamExt;
use multipart::server::Multipart;
use netlify_lambda_http::{http, Request, RequestExt};
use serde::Deserialize;
use std::{
    io::{Cursor, Read},
    time::Duration,
};

pub async fn extract_image(req: &Request) -> anyhow::Result<DynamicImage> {
    if let Some(img) = from_multipart(req).await? {
        return Ok(img);
    }
    if let Some(img) = from_form_urlencoded(req).await? {
        return Ok(img);
    }
    if let Some(img) = from_query_params(req).await? {
        return Ok(img);
    }

    Err(anyhow!("Missing file or url"))
}

async fn from_multipart(req: &Request) -> anyhow::Result<Option<DynamicImage>> {
    let content_type = if let Some(content_type) = req.headers().get(http::header::CONTENT_TYPE) {
        content_type.to_str()?
    } else {
        return Ok(None);
    };

    if !content_type.starts_with("multipart/form-data") {
        return Ok(None);
    }

    let mut body = Cursor::new(req.body());
    let boundary = content_type
        .split_once("=")
        .map(|(_, boundary)| boundary)
        .ok_or_else(|| anyhow!("Invalid Content-Type"))?;

    let mut multipart = Multipart::with_body(&mut body, boundary);
    let mut url = String::new();
    while let Some(mut field) = multipart.read_entry()? {
        match &*field.headers.name {
            "url" => {
                field.data.read_to_string(&mut url)?;
            }
            "file" => {
                let mut buf = Vec::new();
                field.data.read_to_end(&mut buf)?;

                if !buf.is_empty() {
                    let img = tokio::task::spawn_blocking(move || image::load_from_memory(&buf))
                        .await??;
                    return Ok(Some(img));
                }
            }
            _ => (),
        }
    }

    if url.is_empty() {
        Ok(None)
    } else {
        download_image(&url).await.map(Some)
    }
}

async fn from_form_urlencoded(req: &Request) -> anyhow::Result<Option<DynamicImage>> {
    #[derive(Deserialize)]
    struct RequestParams {
        url: String,
    }

    if let Some(url) = req.payload::<RequestParams>()?.map(|params| params.url) {
        download_image(&url).await.map(Some)
    } else {
        Ok(None)
    }
}

async fn from_query_params(req: &Request) -> anyhow::Result<Option<DynamicImage>> {
    if let Some(url) = req.query_string_parameters().get("url") {
        download_image(url).await.map(Some)
    } else {
        Ok(None)
    }
}

async fn download_image(url: &str) -> anyhow::Result<DynamicImage> {
    const TIMEOUT_SECS: u64 = 5;
    const MAX_SIZE: u64 = 8 * 1024 * 1024; // 8 MiB

    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()?;
    let response = client.get(url).send().await?;
    if response
        .content_length()
        .map(|len| len > MAX_SIZE)
        .unwrap_or(false)
    {
        return Err(anyhow!("Image is too large"));
    }

    let mut buf = response
        .content_length()
        .map(|len| Vec::with_capacity(len as usize))
        .unwrap_or_default();
    let mut stream = response.bytes_stream();
    let mut len = 0;
    while let Some(bytes) = stream.next().await {
        let bytes = bytes?;
        len += bytes.len() as u64;
        if len < MAX_SIZE {
            buf.extend_from_slice(&bytes);
        } else {
            return Err(anyhow!("Image is too large"));
        }
    }

    tokio::task::spawn_blocking(move || image::load_from_memory(&buf))
        .await?
        .map_err(Into::into)
}
