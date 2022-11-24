use regex::Regex;

pub fn get_base_url(url: &String) -> String {
  let mut url = url.to_string();
  if url.starts_with("gemini://") {
    url = url[9..].to_string();
  }

  match url.find("/") {
    Some(x) => {
      url = url[..x].to_string();
    }
    None => (),
  }
  url
}

pub fn parse_links(full_url: &String, txt: &String) -> Vec<String> {
  let re = match Regex::new(r"(/\w+/\.\.)") {
    Ok(x) => x,
    Err(_) => return vec![],
  };
  let mut urls = vec![];
  for l in txt.lines() {
    if !l.starts_with("=>") {
      continue;
    }
    let xs: Vec<&str> = l[2..].split_whitespace().collect();
    match xs.get(0) {
      Some(x) => {
        let mut url = String::from(*x);
        if url.starts_with("gemini://") {
          url = url[9..].trim_end_matches("/").to_string();
        } else if url.contains("://") {
          continue;
        } else {
          url = format!(
            "{}/{}",
            full_url,
            url.trim_end_matches('/').trim_start_matches('/')
          )
        }
        urls.push(re.replace_all(url.as_str(), "").to_string());
      }
      None => (),
    }
  }

  urls
}
