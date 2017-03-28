//! Defines configuration file format.

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use toml;
use util::make_path_buf;


/// configuration load from config files
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub root: String,
  pub supplements: Option<Vec<String>>,
}

impl Config {
  pub fn roots(&self) -> Vec<&str> {
    let mut result = vec![self.root.as_str()];
    if let Some(ref supp) = self.supplements {
      result.extend(supp.into_iter().map(|p| p.as_str()));
    }
    result
  }
}

pub fn read_config() -> ::Result<Config> {
  let path: PathBuf = make_path_buf("~/.config/rhq/config.toml")?;
  if !path.is_file() {
    debug!("Saving default config into ~/.config/rhq/config.toml...");
    const CONTENT: &'static str = include_str!("config.toml");
    fs::create_dir_all(path.parent().unwrap())?;
    fs::OpenOptions::new().write(true)
      .create(true)
      .truncate(true)
      .open(&path)?
      .write_all(CONTENT.as_bytes())?;
  }

  debug!("Read content from ~/.config/rhq/config.toml...");
  let mut content = String::new();
  fs::File::open(path)?.read_to_string(&mut content)?;

  debug!("Deserialize config file...");
  let config = toml::from_str(&content)?;
  Ok(config)
}
