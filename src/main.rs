mod crawl;
mod store;

use crawl::Crawler;

fn main() {
  let craww = Crawler::new(
    // vec!["gemini.circumlunar.space".to_string()],
    vec!["geminispace.info/known-hosts".to_string()],
    "store.db".to_string(),
  );

  let mut craww = match craww {
    Err(()) => {
      println!("Can't open database!");
      return;
    }
    Ok(x) => x,
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
