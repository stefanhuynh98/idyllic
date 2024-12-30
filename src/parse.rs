// 178.128.94.113 - - [04/Oct/2024:00:00:18 +0000] "GET /v1-health HTTP/1.1" 200 51 "-" "DigitalOcean Uptime Probe 0.22.0 (https://digitalocean.com)"

use polars::df;
use polars::frame::DataFrame;
use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ParseError(String),
}

#[derive(Debug)]
pub struct Hit {
    ip: String,
}

pub fn parse_lines(lines: &str) -> Result<Vec<Hit>> {
    let re = Regex::new(r"^(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})")?;
    // @TODO: Capture failures as well.
    let hits: Vec<Hit> = re.captures_iter(lines).map(|caps| {
        let (_, [ip]) = caps.extract();

        Hit {
            ip: ip.to_owned(),
        }
    }).collect();

    Ok(hits)
}
