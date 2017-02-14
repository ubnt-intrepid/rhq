//! defines configuration

use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;
use shellexpand;
use errors::Result;

#[cfg_attr(rustfmt, rustfmt_skip)]
const CANDIDATES: &'static [&'static str] =
  &[
    "~/.config/rhq/config"
  , "~/.rhqconfig"
  ];


#[derive(Deserialize)]
struct RawConfig {
  root: Option<String>,
  subroots: Vec<String>,
}

impl RawConfig {
  fn new() -> RawConfig {
    RawConfig {
      root: Some("~/.rhq".into()),
      subroots: Vec::new(),
    }
  }

  fn from_file<P: AsRef<Path>>(path: P) -> Result<Option<RawConfig>> {
    if !path.as_ref().is_file() {
      return Ok(None);
    }

    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(Some(toml::from_str(&content)?))
  }

  fn merge(&mut self, other: RawConfig) {
    if let Some(root) = other.root {
      self.root = Some(root);
      self.subroots.extend(other.subroots);
    }
  }
}

fn read_all_config() -> Result<RawConfig> {
  let mut config = RawConfig::new();
  for path in CANDIDATES {
    let path = shellexpand::full(path).unwrap().into_owned();
    if let Some(conf) = RawConfig::from_file(path)? {
      config.merge(conf);
    }
  }

  Ok(config)
}

/// configuration load from config files
#[derive(Debug)]
pub struct Config {
  pub root: PathBuf,
  pub subroots: Vec<PathBuf>,
}

impl Config {
  pub fn load() -> Result<Config> {
    let raw_config = read_all_config()?;

    let root = raw_config.root.expect("entry 'root' is not found");
    let root = PathBuf::from(shellexpand::full(&root)?.into_owned());

    let subroots = raw_config.subroots
      .into_iter()
      .filter_map(|s| shellexpand::full(&s).map(Cow::into_owned).ok())
      .map(|s| PathBuf::from(s))
      .collect();

    Ok(Config {
      root: root,
      subroots: subroots,
    })
  }
}

impl ::std::fmt::Display for Config {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "root = {}", self.root.display())?;
    write!(f, "lookups = {:?}", self.subroots)?;
    Ok(())
  }
}
