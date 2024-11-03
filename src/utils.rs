use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref LINKS_REGEX: Regex = Regex::new(r"(/\w+/\.\.)").unwrap();
}

pub fn get_base_url(mut url: &str) -> String {
  if url.starts_with("gemini://") {
    url = &url[9..];
  }

  match url.find("/") {
    Some(x) => url[..x].to_string(),
    None => String::new(),
  }
}

pub fn parse_links(full_url: &str, txt: &str) -> Vec<String> {
  txt
    .lines()
    .filter(|line| line.starts_with("=>"))
    .filter_map(|line| line[2..].split_whitespace().next())
    .filter_map(|line| {
      let url = if let Some(link) = line.strip_prefix("gemini://") {
        link.trim_end_matches('/')
      } else if line.contains("://") {
        return None;
      } else {
        &format!(
          "{}/{}",
          full_url.trim_end_matches('/'),
          line.trim_matches('/')
        )
      };

      Some(LINKS_REGEX.replace_all(url, "").to_string())
    })
    .collect()
}
