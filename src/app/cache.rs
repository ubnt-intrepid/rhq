//! Defines cache file format

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json;

pub trait CacheContent: Default + Serialize + Deserialize {
  fn name() -> &'static str;
}

fn cache_path<T: CacheContent>() -> PathBuf {
  env::home_dir()
    .unwrap()
    .join(format!(".cache/rhq/{}.json", T::name()))
}


#[derive(Debug, Default)]
pub struct Cache<T> {
  inner: Option<T>,
}

impl<T: CacheContent> Cache<T> {
  pub fn load() -> ::Result<Self> {
    let cache_path = cache_path::<T>();

    let mut cache = Cache::default();
    if cache_path.exists() {
      debug!("Read content from cache file...");
      let mut content = String::new();
      let mut f = OpenOptions::new().read(true).open(cache_path)?;
      f.read_to_string(&mut content)?;

      debug!("Deserializing from JSON...");
      let value: T = serde_json::from_str(&content)?;
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
      let cache_path = cache_path::<T>();

      debug!("serializing to JSON...");
      let content = serde_json::to_string_pretty(value)?;

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
