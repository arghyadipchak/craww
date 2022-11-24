use std::fs;
use std::io::Error;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Cache {
  pub expected_web_pages: u32,
  pub false_positive_rate: f32,
}

#[derive(Deserialize)]
pub struct Config {
  pub root: String,
  pub timeout: u64,
  pub database: String,
  pub cache: Cache,
}

impl Config {
  pub fn new(path: String) -> Result<Config, Error> {
    let st = fs::read_to_string(path)?;
    return Ok(toml::from_str(st.as_str())?);
  }
}
