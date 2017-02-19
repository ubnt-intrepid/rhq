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


#[derive(Default, Deserialize)]
struct RawConfig {
  roots: Option<Vec<String>>,
  clone_arg: Option<String>,
}

impl RawConfig {
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

    if let Some(oarg) = other.clone_arg {
      self.clone_arg = Some(oarg);
    }
  }
}

fn read_all_config() -> Result<RawConfig> {
  let mut config = RawConfig::default();
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
  pub clone_arg: Option<String>,
}

impl Config {
  /// Returns the path of directory to determine cloned repository's path.
  pub fn default_root(&self) -> &Path {
    self.roots.iter().next().expect("config.roots is empty")
  }
}

pub fn load_from_home() -> Result<Config> {
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

  Ok(Config {
    roots: roots,
    clone_arg: raw_config.clone_arg,
  })
}


impl ::std::fmt::Display for Config {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
