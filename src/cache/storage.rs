use anyhow::Result;
use bytes::Bytes;
use std::{fs::File, io::Write};

pub fn store(file_name: String, data: Bytes) -> Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(&data)?;
    Ok(())
}
