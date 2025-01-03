use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub logs: HashMap<String, Log>,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub name: String,
    pub path: String,
    pub pattern: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Config> {
        let mut file = File::open(path).context("Failed to open config file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).context("Failed to read config file")?;
        let config = serde_json::from_str(&contents)?;

        Ok(config)
    }
}
