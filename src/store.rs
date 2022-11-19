use chrono;
use rusqlite::{params, Connection, Result};

const CREATE_DB_QUERY: &str = r#"
  create table if not exists ?1 (
    url text primary key,
    content text,
    last_visited datetime
  );
"#;

pub struct Store {
  conn: Connection,
  table: String,
}

impl Store {
  pub fn new(db_path: String, table: String) -> Result<Store, ()> {
    match Connection::open(db_path) {
      Ok(x) => match x.execute(CREATE_DB_QUERY, params![table]) {
        Ok(_) => Ok(Store {
          conn: x,
          table: table,
        }),
        Err(err) => {
          println!("{}", err);
          Err(())
        }
      },
      _ => Err(()),
    }
  }

  pub fn add_webpage(&self, url: &String, content: &String) -> Result<(), ()> {
    match self.conn.execute(
      "insert into ?1 (url, content, last_visited) values (?2, ?3, ?4)",
      params![
        self.table,
        url,
        content,
        chrono::offset::Utc::now().to_string(),
      ],
    ) {
      Ok(_) => Ok(()),
      Err(_) => Err(()),
    }
  }
}
