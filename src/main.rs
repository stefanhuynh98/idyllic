use std::env;
use anyhow::{Context, Result};
use idyllic::parse::dataframe_from_log;
use idyllic::config::Config;

fn get_config_path() -> Result<String> {
    let config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(dir) => dir,
        Err(_) => {
            let home_dir = env::var("XDG_HOME")
                .or(env::var("HOME"))
                .context("Unable to find config directory")?;

            format!("{}/.config", home_dir)
        }
    };
    let path = format!("{}/idyllic/idyllic.json", config_dir);

    Ok(path)
}

fn main() -> Result<()> {
    let config_path = get_config_path()?;
    let Config { logs } = Config::load(&config_path)?;
    let _df = dataframe_from_log(&logs["nginx"])?;

    Ok(())
}
