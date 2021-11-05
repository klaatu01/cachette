use crate::config::ItemConfig;
use anyhow::Result;
use bytes::Bytes;
use std::io::Write;

pub fn store(file_name: String, data: Bytes) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("/tmp/{}", file_name))?;
    file.write_all(&data)?;
    file.flush()?;
    Ok(())
}

fn get_file_age_millis(file_name: String) -> Result<u128> {
    let metadata = std::fs::metadata(format!("tmp/{}", file_name))?;
    Ok(metadata.modified()?.elapsed()?.as_millis())
}

pub fn has_expired(config: &ItemConfig) -> bool {
    match config.lifetime {
        None => false,
        Some(lifetime) => match get_file_age_millis(config.file_name.to_owned()) {
            Ok(age) => age > lifetime,
            Err(_) => false,
        },
    }
}
