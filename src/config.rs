//! Defines configuration file format.

use anyhow::{anyhow, Result};
use glob::Pattern;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{
    fs,
    io::Read,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

lazy_static! {
    static ref CONFIG_PATH: PathBuf = dirs::config_dir()
        .map(|config_path| config_path.join("rhq/config.toml"))
        .expect("failed to determine the configuration path");
}

/// configuration load from config files
#[derive(Deserialize)]
struct RawConfigData {
    root: Option<String>,
    default_host: Option<String>,
    includes: Option<Vec<String>>,
    excludes: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ConfigData {
    pub root_dir: PathBuf,
    pub host: String,
    pub include_dirs: Vec<PathBuf>,
    pub exclude_patterns: Vec<Pattern>,
}

impl ConfigData {
    fn from_raw(raw: RawConfigData) -> Result<Self> {
        let root_dir = raw.root.as_deref().unwrap_or("~/rhq");
        let root_dir = crate::util::make_path_buf(root_dir)?;

        let include_dirs = raw
            .includes
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|root| crate::util::make_path_buf(&root).ok())
            .collect();

        let exclude_patterns = raw
            .excludes
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|ex| {
                ::shellexpand::full(&ex)
                    .ok()
                    .map(|ex| ex.replace(r"\", "/"))
                    .and_then(|ex| ::glob::Pattern::new(&ex).ok())
            })
            .collect();

        let host = raw.default_host.unwrap_or_else(|| "github.com".to_owned());

        Ok(Self {
            root_dir,
            host,
            include_dirs,
            exclude_patterns,
        })
    }
}

#[derive(Debug)]
pub struct Config {
    path: PathBuf,
    data: ConfigData,
}

impl Config {
    pub fn new(config_path: Option<&Path>) -> Result<Self> {
        let config_path: &Path = config_path.unwrap_or_else(|| &*CONFIG_PATH);
        if !config_path.is_file() {
            return Err(anyhow!(
                "Failed to load configuration file (config_path = {})",
                config_path.display()
            ));
        }

        let mut content = String::new();
        fs::File::open(config_path)?.read_to_string(&mut content)?;
        let data = ::toml::from_str(&content)?;

        Ok(Config {
            path: config_path.into(),
            data: ConfigData::from_raw(data)?,
        })
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.root_dir.join(".cache.json")
    }
}

impl Deref for Config {
    type Target = ConfigData;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Config {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
