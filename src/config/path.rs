use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
};

const CONIFG_DIR_NAME: &str = "roadblock-lite";
const CONFIG_FILE_NAME: &str = "config.json";

pub fn default_config() -> Result<PathBuf> {
    let home_env = env::var("HOME")?;
    let home_path = Path::new(&home_env);

    Ok(Path::join(home_path, ".config")
        .join(CONIFG_DIR_NAME)
        .join(CONFIG_FILE_NAME))
}
