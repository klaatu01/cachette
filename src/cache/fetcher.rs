use crate::config::HttpTarget;
use anyhow::Result;
use bytes::Bytes;
use reqwest::blocking::Client;

pub async fn http_fetch(target: HttpTarget) -> Result<Bytes> {
    let client = Client::new();
    let body = client.get(&target.url).send()?.bytes()?;
    Ok(body)
}
