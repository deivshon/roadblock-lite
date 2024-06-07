use super::models::Config;

use std::{fs, path::PathBuf};

use anyhow::Result;

pub fn get_config(path: &PathBuf) -> Result<Config> {
    let file_content = fs::read_to_string(path)?;
    let parsed_config: Config = serde_json::from_str(&file_content)?;

    Ok(parsed_config)
}
