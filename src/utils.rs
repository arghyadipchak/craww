pub fn get_base_url(url: &String) -> String {
  let mut url = url.to_string();
  if url.starts_with("gemini://") {
    url = url[9..].to_string();
  }

  match url.find("/") {
    Some(x) => {
      url = url[..x].to_string();
    }
    _ => (),
  }
  url
}

pub fn parse_links(full_url: &String, txt: &String) -> Vec<String> {
  let mut urls = vec![];
  for l in txt.lines() {
    if l.starts_with("=>") {
      let xs: Vec<&str> = l[2..].split_whitespace().collect();
      match xs.get(0) {
        Some(x) => urls.push(String::from(*x)),
        None => continue,
      }
    }
  }

  let mut clean_urls = vec![];
  for u in urls {
    if u.starts_with("gemini://") {
      clean_urls.push(u[9..].to_string())
    } else if !u.contains("://") {
      if full_url.ends_with("/") {
        clean_urls.push(format!("{}{}", full_url, u))
      } else {
        clean_urls.push(format!("{}/{}", full_url, u))
      }
    }
  }

  clean_urls
}
