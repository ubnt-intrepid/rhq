//! Defines cache file format

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use toml;

lazy_static!{
  static ref CACHE_PATH: PathBuf = {
    env::home_dir().unwrap().join(".cache/rhq/cache.toml")
  };
}

#[derive(Debug, Default)]
pub struct Cache<T> {
  inner: Option<T>,
}

impl<T> Cache<T>
  where T: Default + Serialize + Deserialize
{
  pub fn load() -> ::Result<Self> {
    let cache_path: &Path = CACHE_PATH.as_ref();

    let mut cache = Cache::default();
    if cache_path.exists() {
      debug!("Read content from cache file...");
      let mut content = String::new();
      let mut f = OpenOptions::new().read(true).open(cache_path)?;
      f.read_to_string(&mut content)?;

      debug!("Deserializing from TOML...");
      let value: T = toml::from_str(&content)?;
      cache.inner = Some(value);
    }
    Ok(cache)
  }

  pub fn get_opt(&self) -> Option<&T> {
    self.inner.as_ref()
  }

  pub fn get_mut(&mut self) -> &mut T {
    if self.inner.is_none() {
      self.inner = Some(T::default());
    }
    self.inner.as_mut().unwrap()
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
