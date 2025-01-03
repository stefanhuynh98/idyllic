use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::Ipv4Addr;
use std::str::FromStr;

use polars::prelude::*;
use anyhow::Result;
use regex::Regex;

use crate::Hit;
use crate::config::Log;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Couldn't parse token \"{0}\"")]
    Token(String),

    #[error("Missing token:\n\t{0}\nin pattern:\n\t{1}")]
    MissingTokenInPattern(String, String),

    #[error("Missing token \"{0}\" while parsing")]
    MissingToken(String),

    #[error("Pattern didn't match")]
    Match,
}

pub fn dataframe_from_log(Log { pattern, path, .. }: &Log) -> Result<DataFrame> {
    let re = build_regex(&pattern)?;
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut addr_vec: Vec<String> = vec![];
    let mut status_vec: Vec<u64> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let hit = parse_line(&line, &re)?;

            addr_vec.push(hit.addr.to_string());
            status_vec.push(hit.status);
        }
    }

    let df = DataFrame::new(vec![
        Column::new("addr".into(), addr_vec),
        Column::new("status".into(), status_vec),
    ])?;

    Ok(df)
}

fn parse_line(line: &str, re: &Regex) -> Result<Hit> {
    let captures = re.captures(line).ok_or(ParseError::Match)?;
    let addr = captures.name("addr").ok_or(ParseError::MissingToken("addr".into()))?.as_str();
    let addr = Ipv4Addr::from_str(addr).map_err(|_| ParseError::Token("addr".into()))?;
    let status = captures.name("status").ok_or(ParseError::MissingToken("status".into()))?.as_str();
    let status: u64 = status.parse()?;

    Ok(Hit {
        addr,
        status,
    })
}

fn build_regex(pattern: &str) -> Result<Regex> {
    // Check for required tokens' existence
    pattern.find ("{{addr}}").ok_or       (ParseError::MissingTokenInPattern("{{addr}}".into()       , pattern.into()))?;
    pattern.find ("{{time}}").ok_or       (ParseError::MissingTokenInPattern("{{time}}".into()       , pattern.into()))?;
    pattern.find ("{{request}}").ok_or    (ParseError::MissingTokenInPattern("{{request}}".into()    , pattern.into()))?;
    pattern.find ("{{status}}").ok_or     (ParseError::MissingTokenInPattern("{{status}}".into()     , pattern.into()))?;
    pattern.find ("{{bytes}}").ok_or      (ParseError::MissingTokenInPattern("{{bytes}}".into()      , pattern.into()))?;
    pattern.find ("{{user_agent}}").ok_or (ParseError::MissingTokenInPattern("{{user_agent}}".into() , pattern.into()))?;

    let re_addr               = r#"(?<addr>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})"#;
    let re_user               = r#"(?<user>\S*)"#;
    let re_time               = r#"(?<time>\d\d\/\w*\/\d{4}:\d\d:\d\d:\d\d\s\+\d{4})"#;
    let re_request            = r#"(?<request>GET\s\S+\s\S+)"#;
    let re_status             = r#"(?<status>\d{3})"#;
    let re_bytes              = r#"(?<bytes>\d+)"#;
    let re_referrer           = r#"(?<referrer>\S+)"#;
    let re_user_agent         = r#"(?<user_agent>[\w\d\s\.\(\):;\/\/]*)"#;
    let re_http_forwarded_for = r#"(?<http_forwarded_for>\S+)"#;

    let re = pattern
        .replace(r#" "#, r#"\s"#)
        .replace(r#"["#, r#"\["#)
        .replace(r#"]"#, r#"\]"#)
        .replace(r#"""#, r#"\""#)
        .replacen("{{addr}}", re_addr, 1)
        .replacen("{{user}}", re_user, 1)
        .replacen("{{time}}", re_time, 1)
        .replacen("{{request}}", re_request, 1)
        .replacen("{{status}}", re_status, 1)
        .replacen("{{bytes}}", re_bytes, 1)
        .replacen("{{referrer}}", re_referrer, 1)
        .replacen("{{user_agent}}", re_user_agent, 1)
        .replacen("{{http_forwarded_for}}", re_http_forwarded_for, 1);

    Ok(Regex::new(&re)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_regex_case_1() -> anyhow::Result<()> {
        let pattern = "{{addr}} - {{user}} [{{time}}] \"{{request}}\" {{status}} {{bytes}} \"{{referrer}}\" \"{{user_agent}}\"";
        let regex = build_regex(pattern)?;
        let expected = r#"(?<addr>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s-\s(?<user>\S*)\s\[(?<time>\d\d\/\w*\/\d{4}:\d\d:\d\d:\d\d\s\+\d{4})\]\s\"(?<request>GET\s\S+\s\S+)\"\s(?<status>\d{3})\s(?<bytes>\d+)\s\"(?<referrer>\S+)\"\s\"(?<user_agent>[\w\d\s\.\(\):;\/\/]*)\""#;

        assert_eq!(regex.as_str(), expected);

        Ok(())
    }

    #[test]
    fn build_regex_case_2() -> anyhow::Result<()> {
        let pattern = "[{{time}}] {{user}} {{addr}} \"{{request}}\" {{status}} - {{bytes}} \"{{referrer}}\" \"{{user_agent}}\"";
        let regex = build_regex(pattern)?;
        let expected = r#"\[(?<time>\d\d\/\w*\/\d{4}:\d\d:\d\d:\d\d\s\+\d{4})\]\s(?<user>\S*)\s(?<addr>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s\"(?<request>GET\s\S+\s\S+)\"\s(?<status>\d{3})\s-\s(?<bytes>\d+)\s\"(?<referrer>\S+)\"\s\"(?<user_agent>[\w\d\s\.\(\):;\/\/]*)\""#;

        assert_eq!(regex.as_str(), expected);

        Ok(())
    }
}
