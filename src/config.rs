//! defines configuration

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;
use shellexpand;
use errors::Result;

#[derive(Default, Deserialize)]
struct RawConfig {
  root: Option<String>,
}

impl RawConfig {
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
    #[cfg_attr(rustfmt, rustfmt_skip)]
  const DEFAULT_CONFIG: &'static str =
      include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rhqconfig"));

    #[cfg_attr(rustfmt, rustfmt_skip)]
  const CANDIDATES: &'static [&'static str] =
      &["~/.config/rhq/config", "~/.rhqconfig"];

  let mut config: RawConfig = toml::from_str(DEFAULT_CONFIG)
    .expect("failed to decode default config");
  for path in CANDIDATES {
    let path = shellexpand::full(path).unwrap().into_owned();
    if let Some(conf) = RawConfig::from_file(path)? {
      config.merge(conf);
    }
  }

  Ok(config)
}

/// configuration load from config.toml
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
