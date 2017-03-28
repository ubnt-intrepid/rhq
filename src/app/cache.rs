//! Defines cache file format

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde;
use toml;

lazy_static!{
  static ref CACHE_PATH: PathBuf = {
    env::home_dir().unwrap().join(".cache/rhq/cache.toml")
  };
}

#[derive(Debug, Default)]
pub struct Cache {
  inner: Option<toml::Value>,
}

impl Cache {
  pub fn get_value<T>(&self) -> ::Result<Option<T>>
    where T: serde::Deserialize
  {
    if let Some(ref value) = self.inner {
      let value: T = value.clone().try_into()?;
      Ok(Some(value))
    } else {
      Ok(None)
    }
  }

  pub fn set_value<T>(&mut self, value: T) -> ::Result<()>
    where T: serde::Serialize
  {
    let value = toml::Value::try_from(value)?;
    self.inner = Some(value);
    Ok(())
  }

  pub fn load() -> ::Result<Cache> {
    let cache_path: &Path = CACHE_PATH.as_ref();

    let mut cache = Cache::default();
    if cache_path.exists() {
      debug!("Read content from cache file...");
      let mut content = String::new();
      let mut f = OpenOptions::new().read(true).open(cache_path)?;
      f.read_to_string(&mut content)?;

      debug!("Deserializing from TOML...");
      let value: toml::Value = toml::from_str(&content)?;
      cache.inner = Some(value);
    }
    Ok(cache)
  }

  pub fn dump(&self) -> ::Result<()> {
    if let Some(ref value) = self.inner {
      let cache_path: &Path = CACHE_PATH.as_ref();

      debug!("serializing to TOML...");
      let content = toml::to_string(value)?;

      debug!("saving to cache file...");
      fs::create_dir_all(cache_path.parent().unwrap())?;
      let mut f = OpenOptions::new().write(true)
        .create(true)
        .truncate(true)
        .open(cache_path)?;
      f.write_all(content.as_bytes())?;
    }
    Ok(())
  }
}
