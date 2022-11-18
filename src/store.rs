use chrono;
use rusqlite::{params, Connection, Result};
pub struct Store {
  conn: Connection,
}

const CREATE_DB_QUERY: &str = r#"
  create table if not exists page_store (
      url text primary key,
      content text,
      last_visited datetime
  );
  "#;

impl Store {
  pub fn new(db_path: String) -> Result<Store, ()> {
    match Connection::open(db_path) {
      Ok(x) => match x.execute(CREATE_DB_QUERY, []) {
        Ok(_) => Ok(Store { conn: x }),
        _ => Err(()),
      },
      _ => Err(()),
    }
  }

  pub fn add_webpage(
    &mut self,
    url: &String,
    content: &String,
  ) -> Result<(), ()> {
    match self.conn.execute(
      "INSERT INTO page_store (url, content, last_visited) values (?1, ?2, ?3)",
      params![url, content, chrono::offset::Utc::now().to_string()],
    ) {
      Ok(_) => Ok(()),
      Err(_) => Err(()),
    }
  }

  // pub fn needs_to_visit(url: String) -> bool {
  //   true
  // }
}
