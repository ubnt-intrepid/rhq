//! Defines cache file format

use std::env;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json;

pub trait CacheContent<'de>: Default + Serialize + Deserialize<'de> {
    fn name() -> &'static str;
}

fn cache_path<T>() -> PathBuf
where
    for<'de> T: CacheContent<'de>,
{
    env::home_dir()
        .unwrap()
        .join(format!(".cache/rhq/{}.json", T::name()))
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
pub struct Cache<T> {
    #[serde(with = "serde_datetime")] timestamp: DateTime<Local>,
    inner: Option<T>,
}

impl<T> Cache<T>
where
    for<'de> T: CacheContent<'de>,
{
    pub fn load() -> ::Result<Self> {
        let cache_path = cache_path::<T>();
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

    pub fn get_opt(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        if self.inner.is_none() {
            self.inner = Some(T::default());
        }
        self.inner.as_mut().unwrap()
    }

    pub fn dump(&mut self) -> ::Result<()> {
        self.timestamp = Local::now();

        let cache_path = cache_path::<T>();
        let cache_dir = cache_path
            .parent()
            .ok_or("cannot get parent directory of cache file")?;

        fs::create_dir_all(cache_dir)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&cache_path)?;
        serde_json::to_writer_pretty(&mut file, &self)?;

        Ok(())
    }
}
