//! defines configuration

use std::borrow::{Borrow, Cow};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;
use shellexpand;

#[cfg_attr(rustfmt, rustfmt_skip)]
const CANDIDATES: &'static [&'static str] =
  &[
    "~/.config/rhq/config"
  , "~/.config/rhq/config.toml"
  , "~/.rhqconfig"
  ];


/// configuration load from config files
#[derive(Debug)]
pub struct Config {
  pub root: PathBuf,
  pub supplements: Vec<PathBuf>,
}

impl Config {
  pub fn roots(&self) -> Vec<&Path> {
    let mut roots = vec![self.root.as_path()];
    roots.extend(self.supplements.iter().map(|ref s| s.as_path()));
    roots
  }
}

fn make_path_buf<'a>(s: Cow<'a, str>) -> ::Result<PathBuf> {
  shellexpand::full(s.borrow() as &str)
    .map(|s| PathBuf::from(s.borrow() as &str))
    .map_err(Into::into)
}

fn read_toml_table<P: AsRef<Path>>(path: P) -> ::Result<toml::value::Table> {
  let mut content = String::new();
  File::open(path)?.read_to_string(&mut content)?;
  toml::de::from_str(&content).map_err(Into::into)
}

pub fn read_all_config() -> ::Result<Config> {
  let mut root = None;
  let mut supplements = Vec::new();

  for path in CANDIDATES.iter()
        .map(|&path| make_path_buf(path.into()).unwrap())
        .filter(|ref path| path.is_file()) {
    let config = read_toml_table(path)?;

    if let Some(r) = config.get("root") {
      let r = r.as_str().ok_or("config.root is not a string")?;
      root = Some(r.to_owned());
    }

    if let Some(supp) = config.get("supplements") {
      let supp = supp.as_array().ok_or("config.supplements is not an array")?;
      for s in supp {
        let s = s.as_str().ok_or("config.supplements contains an invalid element")?;
        let s = make_path_buf(s.into())?;
        supplements.push(s);
      }
    }
  }

  let root = root.map(|r| Cow::Owned(r)).unwrap_or("~/rhq".into());
  let root = make_path_buf(root)?;

  Ok(Config {
       root: root,
       supplements: supplements,
     })
}
