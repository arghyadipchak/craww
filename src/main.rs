mod crawl;
mod store;

use crawl::Crawler;
use store::Store;

fn main() {
  let store = match Store::new("store.db".to_string(), "page_store".to_string())
  {
    Ok(x) => x,
    Err(_) => {
      println!("Database Connection Error!");
      return;
    }
  };

  let mut craww = match Crawler::new(
    vec!["gemini.circumlunar.space".to_string()],
    // vec!["geminispace.info/known-hosts".to_string()],
    store,
  ) {
    Ok(x) => x,
    Err(_) => {
      println!("Unable to create Crawler!");
      return;
    }
  };

  let mut c = 1;

  while !craww.is_done() {
    match craww.next() {
      Some((url, content)) => {
        println!("{} {} {}", c, url, content.len());
        c += 1;
      }
      _ => println!("{}", c),
    };
  }
}
