//! defines configuration

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


#[derive(Default, Deserialize)]
struct RawConfig {
  root: Option<String>,
}

impl RawConfig {
  fn new() -> RawConfig {
    RawConfig { root: Some("~/.rhq".into()) }
  }

  fn from_file<P: AsRef<Path>>(path: P) -> Result<Option<RawConfig>> {
    if !path.as_ref().is_file() {
      return Ok(None);
    }

    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(Some(toml::from_str(&content).ok().unwrap_or_default()))
  }

  fn merge(&mut self, other: RawConfig) {
    if let Some(root) = other.root {
      self.root = Some(root);
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
}

impl Config {
  pub fn load() -> Result<Config> {
    let raw_config = read_all_config()?;

    let root = raw_config.root.expect("entry 'root' is not found");
    let root = PathBuf::from(shellexpand::full(&root)?.into_owned());

    Ok(Config { root: root })
  }
}
