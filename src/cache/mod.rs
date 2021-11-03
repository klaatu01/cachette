pub mod fetcher;
pub mod storage;
use anyhow::Result;
use futures::Future;

use crate::config::{CacheConfig, HttpTarget, Target};

pub async fn process(cfg: CacheConfig) -> Result<()> {
    let processors = get_processors(&cfg);
    futures::future::join_all(processors).await;
    Ok(())
}

pub fn get_processors(cfg: &CacheConfig) -> Vec<impl Future<Output = Result<()>>> {
    cfg.item_configs
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
