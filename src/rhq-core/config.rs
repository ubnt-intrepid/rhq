//! Defines configuration file format.

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};


lazy_static! {
  static ref CONFIG_PATH: PathBuf = {
    env::home_dir().unwrap().join(".config/rhq/config.toml")
  };
}


/// configuration load from config files
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub root: Option<String>,
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
}

impl Config {
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


impl Config {
    pub fn load() -> ::Result<Self> {
        let path: &Path = CONFIG_PATH.as_ref();

        if !path.is_file() {
            debug!("Saving default config into ~/.config/rhq/config.toml...");
            let content = include_str!("config.toml");
            fs::create_dir_all(path.parent().unwrap())?;
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)?
                .write_all(content.as_bytes())?;
        }

        debug!("Read content from ~/.config/rhq/config.toml...");
        let mut content = String::new();
        fs::File::open(path)?.read_to_string(&mut content)?;

        debug!("Deserialize config file...");
        let config = ::toml::from_str(&content)?;

        Ok(config)
    }
}
