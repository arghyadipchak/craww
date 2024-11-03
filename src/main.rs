mod config;
mod crawl;
mod store;
mod utils;

use std::process;

use config::Config;
use crawl::Crawler;
use store::Store;

fn main() {
  let config = match Config::read_from_file("config.toml") {
    Ok(c) => c,
    Err(err) => {
      eprintln!("error reading config: {err}");
      process::exit(1);
    }
  };

  let store = match Store::new(&config) {
    Ok(s) => s,
    Err(err) => {
      eprintln!("error connecting to database: {err}");
      process::exit(1);
    }
  };

  let mut craww =
    match Crawler::new(vec![config.root.clone()], config.timeout, store) {
      Ok(c) => c,
      Err(err) => {
        eprintln!("error creating crawler: {err}");
        process::exit(1);
      }
    };

  while let Some((url, content)) = craww.next() {
    println!("{: >10} {}", content.len(), url);
  }
}
