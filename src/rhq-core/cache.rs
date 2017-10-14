//! Defines cache file format

use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde_json;

use repository::Repository;


lazy_static! {
    static ref CACHE_PATH: PathBuf = env::home_dir().unwrap().join(".cache/rhq/cache.json");
}


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
    pub fn load() -> ::Result<Self> {
        if CACHE_PATH.exists() {
            let mut file = OpenOptions::new().read(true).open(&*CACHE_PATH)?;
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

    pub fn dump(&mut self) -> ::Result<()> {
        self.timestamp = Local::now();
        ::util::write_content(&*CACHE_PATH, |f| {
            serde_json::to_writer_pretty(f, &self).map_err(Into::into)
        })
    }
}
