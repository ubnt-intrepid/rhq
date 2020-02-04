//! Defines cache file format

use crate::repository::Repository;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

// inner representation of cache format.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    timestamp: DateTime<Local>,
    inner: Option<CacheData>,
}

impl Cache {
    pub fn new(cache_path: &Path) -> Result<Self> {
        if cache_path.exists() {
            let mut file = OpenOptions::new().read(true).open(cache_path)?;
            let cache = serde_json::from_reader(&mut file)?;
            Ok(cache)
        } else {
            Ok(Cache {
                timestamp: Local::now(),
                inner: None,
            })
        }
    }

    pub fn get_opt(&self) -> Option<&CacheData> {
        self.inner.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut CacheData {
        if self.inner.is_none() {
            self.inner = Some(Default::default());
        }
        self.inner.as_mut().unwrap()
    }

    pub fn dump(&mut self, cache_path: &Path) -> Result<()> {
        self.timestamp = Local::now();
        crate::util::write_content(cache_path, |f| {
            serde_json::to_writer_pretty(f, &self).map_err(Into::into)
        })
    }
}
