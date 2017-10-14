//! Defines cache file format

use std::env;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde_json;

use repository::Repository;

// inner representation of cache format.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub repositories: Vec<Repository>,
}

lazy_static! {
    static ref CACHE_PATH: PathBuf = env::home_dir()
        .unwrap()
        .join(".cache/rhq/cache.json");
}

mod serde_datetime {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";

    pub fn serialize<S>(date: &DateTime<Local>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        ser.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<DateTime<Local>, D::Error> {
        let s = String::deserialize(de)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    #[serde(with = "serde_datetime")] timestamp: DateTime<Local>,
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

        let cache_dir = CACHE_PATH
            .parent()
            .ok_or("cannot get parent directory of cache file")?;

        fs::create_dir_all(cache_dir)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*CACHE_PATH)?;
        serde_json::to_writer_pretty(&mut file, &self)?;

        Ok(())
    }
}
