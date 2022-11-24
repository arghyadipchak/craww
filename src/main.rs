mod config;
mod crawl;
mod store;
mod utils;

use config::Config;
use crawl::Crawler;
use store::Store;

fn main() {
  let config = match Config::new("config.toml".to_string()) {
    Ok(x) => x,
    Err(_) => panic!("Unable to read Config!"),
  };

  let store = match Store::new(&config) {
    Ok(x) => x,
    Err(_) => panic!("Database Connection Error!"),
  };

  let mut craww =
    match Crawler::new(vec![config.root.clone()], config.timeout, store) {
      Ok(x) => x,
      Err(_) => panic!("Unable to create Crawler!"),
    };

  while !craww.is_done() {
    match craww.next() {
      Some((url, content)) => println!("{: >6} {}", content.len(), url),
      None => (),
    };
  }
}
