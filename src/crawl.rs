use std::{
  collections::VecDeque,
  io::{Error, Read, Write},
  net::{SocketAddr, TcpStream},
  sync::Arc,
  time::{Duration, SystemTime},
};

use rustls::{
  client::{ServerCertVerified, ServerCertVerifier},
  Certificate, ClientConfig, ClientConnection, RootCertStore, ServerName,
  Stream,
};
use trust_dns_resolver::{
  config::{ResolverConfig, ResolverOpts},
  Resolver,
};

use crate::{store::Store, utils};

struct DummyVerifier {}
impl DummyVerifier {
  fn new() -> Self {
    DummyVerifier {}
  }
}

impl ServerCertVerifier for DummyVerifier {
  fn verify_server_cert(
    &self,
    _: &Certificate,
    _: &[Certificate],
    _: &ServerName,
    _: &mut dyn Iterator<Item = &[u8]>,
    _: &[u8],
    _: SystemTime,
  ) -> Result<ServerCertVerified, rustls::Error> {
    Ok(ServerCertVerified::assertion())
  }
}

pub struct Crawler {
  resolver: Resolver,
  to_visit: VecDeque<String>,
  timeout: u64,
  store: Store,
}

impl Crawler {
  pub fn new(
    seeds: Vec<String>,
    timeout: u64,
    store: Store,
  ) -> Result<Crawler, Error> {
    Ok(Crawler {
      resolver: Resolver::new(
        ResolverConfig::default(),
        ResolverOpts::default(),
      )?,
      to_visit: VecDeque::from_iter(seeds.iter().map(|x| {
        if x.starts_with("gemini://") {
          x.as_str()[9..].to_string()
        } else {
          x.to_string()
        }
      })),
      timeout,
      store,
    })
  }

  fn get_page(&self, full_url: &String) -> Option<String> {
    let mut cfg = ClientConfig::builder()
      .with_safe_defaults()
      .with_root_certificates(RootCertStore::empty())
      .with_no_client_auth();

    ClientConfig::dangerous(&mut cfg)
      .set_certificate_verifier(Arc::new(DummyVerifier::new()));

    let base_url = utils::get_base_url(full_url);
    let domain = base_url.as_str().try_into().ok()?;
    let ip = self.resolver.lookup_ip(&base_url).ok()?.iter().next()?;

    let mut client = ClientConnection::new(Arc::new(cfg), domain).ok()?;

    let mut socket = TcpStream::connect_timeout(
      &SocketAddr::new(ip, 1965),
      Duration::new(self.timeout, 0),
    )
    .ok()?;

    let mut stream = Stream::new(&mut client, &mut socket);
    stream
      .write_all(format!("gemini://{}/\r\n", full_url).as_bytes())
      .ok()?;

    let mut content = String::new();
    stream.read_to_string(&mut content).ok()?;

    if content.starts_with("20 ") {
      content.find("\n").map(|i| content[i + 1..].to_string())
    } else {
      None
    }
  }

  pub fn next(&mut self) -> Option<(String, String)> {
    let full_url = self.to_visit.pop_front()?;
    let content = self.get_page(&full_url)?;

    for u in utils::parse_links(&full_url, &content) {
      if !self.store.have_visited(&u) {
        self.to_visit.push_back(u);
      }
    }

    let _ = self.store.add_webpage(&full_url, &content);
    Some((full_url, content))
  }
}
