use std::{fs, io::Error, path::Path};

use serde::Deserialize;

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
  pub fn read_from_file(path: impl AsRef<Path>) -> Result<Config, Error> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
  }
}
