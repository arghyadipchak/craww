use bloom::BloomFilter;
use chrono;
use rusqlite::{params, Connection, Result};

use crate::config::Config;
use crate::utils::get_base_url;

const CREATE_DB_QUERY: &str = "
  create table if not exists page_store (
    url text primary key,
    domain text,
    content text,
    last_visited datetime
  );
";

pub struct Store {
  conn: Connection,
  cache: BloomFilter,
}

impl Store {
  pub fn new(config: &Config) -> Result<Store> {
    let conn = Connection::open(config.database.clone())?;
    conn.execute(CREATE_DB_QUERY, [])?;

    return Ok(Store {
      conn: conn,
      cache: BloomFilter::with_rate(
        config.cache.false_positive_rate,
        config.cache.expected_web_pages,
      ),
    });
  }

  pub fn have_visited(&self, url: &String) -> bool {
    self.cache.contains(url)
  }

  pub fn add_webpage(&mut self, url: &String, content: &String) -> Result<()> {
    self.conn.execute(
      "insert into page_store (url, domain, content, last_visited) values (?1, ?2, ?3, ?4)",
      params![
        url,
        get_base_url(url),
        content,
        chrono::offset::Utc::now().to_string(),
      ],
    )?;
    self.cache.insert(url);

    return Ok(());
  }
}
