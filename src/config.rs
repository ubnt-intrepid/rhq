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
  roots: Option<Vec<String>>,
}

impl RawConfig {
  fn new() -> RawConfig {
    RawConfig { roots: None }
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
    if let Some(oroots) = other.roots {
      if let Some(ref mut roots) = self.roots {
        roots.extend(oroots);
      } else {
        self.roots = Some(oroots);
      }
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
  pub roots: Vec<PathBuf>,
}

impl Config {
  pub fn load() -> Result<Config> {
    let raw_config = read_all_config()?;

    let mut roots: Vec<PathBuf> = raw_config.roots
      .map(|roots| {
        roots.into_iter()
          .filter_map(|s| shellexpand::full(&s).map(Cow::into_owned).ok())
          .map(|s| PathBuf::from(s))
          .collect()
      })
      .unwrap_or_default();
    if roots.len() == 0 {
      roots.push(shellexpand::full("~/.rhq")?.into_owned().into());
    }

    Ok(Config { roots: roots })
  }
}

impl ::std::fmt::Display for Config {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
