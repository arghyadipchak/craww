use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use rustls::client::{ServerCertVerified, ServerCertVerifier};
use rustls::{
  Certificate, ClientConfig, ClientConnection, RootCertStore, ServerName,
  Stream,
};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

use crate::store::Store;

const TIMEOUT: u64 = 30;

struct DummyVerifier {}
impl DummyVerifier {
  fn new() -> Self {
    DummyVerifier {}
  }
}

impl ServerCertVerifier for DummyVerifier {
  fn verify_server_cert(
    &self,
    _end_entity: &Certificate,
    _intermediates: &[Certificate],
    _server_name: &ServerName,
    _scts: &mut dyn Iterator<Item = &[u8]>,
    _ocsp_response: &[u8],
    _now: SystemTime,
  ) -> Result<ServerCertVerified, rustls::Error> {
    return Ok(ServerCertVerified::assertion());
  }
}

pub struct Crawler {
  resolver: Resolver,
  to_visit: VecDeque<String>,
  store: Store,
}

impl Crawler {
  pub fn new(seeds: Vec<String>, store: Store) -> Result<Crawler, ()> {
    match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
      Ok(x) => Ok(Crawler {
        resolver: x,
        to_visit: VecDeque::from_iter(seeds.iter().map(|x| {
          if x.starts_with("gemini://") {
            x.as_str()[9..].to_string()
          } else {
            x.to_string()
          }
        })),
        store: store,
      }),
      Err(_) => Err(()),
    }
  }

  pub fn is_done(&self) -> bool {
    self.to_visit.is_empty()
  }

  fn get_page(&self, full_url: &String) -> Option<String> {
    let base_url = get_base_url(full_url);
    let ip = match self.resolver.lookup_ip(&base_url) {
      Ok(lip) => match lip.iter().next() {
        Some(ip) => ip,
        _ => return None,
      },
      _ => return None,
    };

    let mut cfg = ClientConfig::builder()
      .with_safe_defaults()
      .with_root_certificates(RootCertStore::empty())
      .with_no_client_auth();

    let mut config = ClientConfig::dangerous(&mut cfg);
    config.set_certificate_verifier(Arc::new(DummyVerifier::new()));

    let mut client = match base_url.as_str().try_into() {
      Ok(x) => match ClientConnection::new(Arc::new(cfg), x) {
        Ok(cc) => cc,
        _ => return None,
      },
      _ => return None,
    };

    let mut socket = match TcpStream::connect_timeout(
      &SocketAddr::new(ip, 1965),
      Duration::new(TIMEOUT, 0),
    ) {
      Ok(x) => x,
      _ => return None,
    };

    let mut stream = Stream::new(&mut client, &mut socket);
    match stream.write(format!("gemini://{}/\r\n", full_url).as_bytes()) {
      Ok(_) => (),
      Err(_) => return None,
    }

    let mut content = String::new();
    match stream.read_to_string(&mut content) {
      Ok(_) => (),
      Err(_) => return None,
    }

    Some(content)
  }

  pub fn next(&mut self) -> Option<(String, String)> {
    match self.to_visit.pop_front() {
      Some(full_url) => match self.get_page(&full_url) {
        Some(content) => {
          for u in parse_links(&full_url, &content) {
            // println!("{}", u);
            self.to_visit.push_back(u);
          }
          match self.store.add_webpage(&full_url, &content) {
            _ => (),
          };
          return Some((full_url, content));
        }
        _ => return None,
      },
      _ => return None,
    };
  }
}

fn get_base_url(url: &String) -> String {
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

fn parse_links(full_url: &String, txt: &String) -> Vec<String> {
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
