#[macro_use]
extern crate serde;
extern crate log;
extern crate serde_json;

mod cache;
mod config;
mod extension;

use anyhow::Result;
use extension::runtime;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::debug!("Building {} Client", extension::get_extension_name());
    let client = Client::builder().build()?;
    log::debug!("Registering Extension...");
    let ext_id = extension::register_extension(&client).await?;
    log::debug!("Registered.");
    let config = config::parse_config_from_envar()?;
    cache::process_all(config.clone()).await?;
    log::debug!("Starting Runtime Consumer...");
    let response = runtime::run(&client, ext_id, config).await;
    response
}
