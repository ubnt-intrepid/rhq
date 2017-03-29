//! Defines configuration file format.

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use toml;
use super::InitialStr;

lazy_static! {
  static ref CONFIG_PATH: PathBuf = {
    env::home_dir().unwrap().join(".config/rhq/config.toml")
  };
}

/// configuration load from config files
#[derive(Debug, Serialize, Deserialize)]
pub struct Config<T> {
  inner: T,
}

impl<T> Config<T>
  where T: Serialize + Deserialize + InitialStr
{
  pub fn load() -> ::Result<Self> {
    let path: &Path = CONFIG_PATH.as_ref();

    if !path.is_file() {
      debug!("Saving default config into ~/.config/rhq/config.toml...");
      let content = T::initial_str();
      fs::create_dir_all(path.parent().unwrap())?;
      fs::OpenOptions::new().write(true)
        .create(true)
        .truncate(true)
        .open(&path)?
        .write_all(content.as_bytes())?;
    }

    debug!("Read content from ~/.config/rhq/config.toml...");
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;

    debug!("Deserialize config file...");
    let inner = toml::from_str(&content)?;

    Ok(Config { inner: inner })
  }

  pub fn get(&self) -> &T {
    &self.inner
  }
}
