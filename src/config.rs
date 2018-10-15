//! Defines configuration file format.

use dirs;
use glob::Pattern;
use std::fs;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

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
    fn from_raw(raw: RawConfigData) -> ::Result<Self> {
        let root_dir = raw.root.as_ref().map(|s| s.as_str()).unwrap_or("~/rhq");
        let root_dir = ::util::make_path_buf(root_dir)?;

        let include_dirs = raw
            .includes
            .as_ref()
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .into_iter()
            .filter_map(|root| ::util::make_path_buf(&root).ok())
            .collect();

        let exclude_patterns = raw
            .excludes
            .as_ref()
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .into_iter()
            .filter_map(|ex| {
                ::shellexpand::full(&ex)
                    .ok()
                    .map(|ex| ex.replace(r"\", "/"))
                    .and_then(|ex| ::glob::Pattern::new(&ex).ok())
            }).collect();

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
    pub fn new(config_path: Option<&Path>) -> ::Result<Self> {
        let config_path: &Path = config_path.unwrap_or_else(|| &*CONFIG_PATH);
        if !config_path.is_file() {
            Err(::ErrorKind::ConfigLoad(config_path.into()))?;
        }

        let mut content = String::new();
        fs::File::open(config_path)?.read_to_string(&mut content)?;
        let data = ::toml::from_str(&content)?;

        Ok(Config {
            path: config_path.into(),
            data: ConfigData::from_raw(data)?,
        })
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
