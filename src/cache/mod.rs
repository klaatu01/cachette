pub mod fetcher;
pub mod storage;
use anyhow::Result;
use futures::Future;

use crate::config::{CacheConfig, HttpTarget, ItemConfig, Target};

pub async fn process_all(cfg: CacheConfig) -> Result<()> {
    process(&cfg.item_configs).await
}

pub async fn process(items: &Vec<ItemConfig>) -> Result<()> {
    let processors = get_processors(items);
    futures::future::join_all(processors).await;
    Ok(())
}

pub async fn process_expired(cfg: CacheConfig) -> Result<()> {
    let targets = get_expired_targets(cfg);
    process(&targets).await
}

pub fn get_processors(items: &Vec<ItemConfig>) -> Vec<impl Future<Output = Result<()>>> {
    items
        .iter()
        .map(|item| match item.target.clone() {
            Target::Http(target) => process_http(item.file_name.clone(), target),
        })
        .collect()
}

pub async fn process_http(file_name: String, target: HttpTarget) -> Result<()> {
    let data = fetcher::http_fetch(target).await?;
    storage::store(file_name, data)
}

pub fn get_expired_targets(cfg: CacheConfig) -> Vec<ItemConfig> {
    cfg.item_configs
        .iter()
        .cloned()
        .filter(|item| storage::has_expired(item))
        .collect()
}
