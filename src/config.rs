use serde_derive::Deserialize;
use toml;

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct SearchConfig {
    conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Deserialize)]
struct Condition {
    squeeze: Vec<String>,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let input = fs::read_to_string(path).expect("should exist condition.toml");
    let result: SearchConfig = toml::from_str(&input)?;

    let conditions = result.conditions.expect("should have field conditions");
    Ok(conditions
        .iter()
        .map(|condition| condition.squeeze.join(","))
        .collect())
}
