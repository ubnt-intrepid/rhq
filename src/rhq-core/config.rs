//! Defines configuration file format.

use std::env;
use std::fs;
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};


lazy_static! {
    static ref CONFIG_PATH: PathBuf = env::home_dir().unwrap().join(".config/rhq/config.toml");
}


/// configuration load from config files
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub root: Option<String>,
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
}

impl ConfigData {
    pub fn root_dir(&self) -> Option<PathBuf> {
        self.root
            .as_ref()
            .and_then(|root| ::util::make_path_buf(root).ok())
    }

    pub fn include_dirs(&self) -> Vec<PathBuf> {
        self.includes
            .as_ref()
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .into_iter()
            .filter_map(|root| ::util::make_path_buf(&root).ok())
            .collect()
    }

    pub fn exclude_patterns(&self) -> Vec<::glob::Pattern> {
        self.excludes
            .as_ref()
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .into_iter()
            .filter_map(|ex| {
                ::shellexpand::full(&ex)
                    .ok()
                    .map(|ex| ex.replace(r"\", "/"))
                    .and_then(|ex| ::glob::Pattern::new(&ex).ok())
            })
            .collect()
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
            data,
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
