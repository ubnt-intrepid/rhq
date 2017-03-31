//! defines functions/types related to local repository access.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use util::{self, process};
use vcs;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
  path: PathBuf,
  url: Option<String>,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> ::Result<Self> {
    Ok(Repository {
         path: util::canonicalize_pretty(path)?,
         url: None,
       })
  }

  /// Set URL of remote repository.
  pub fn set_url<S: Into<String>>(&mut self, url: S) {
    self.url = Some(url.into());
  }

  pub fn is_same_local(&self, other: &Self) -> bool {
    self.path.as_path() == other.path.as_path()
  }

  pub fn is_vcs(&self) -> bool {
    vcs::detect_from_path(&self.path).is_some()
  }

  pub fn is_contained<P: AsRef<Path>>(&self, path: P) -> bool {
    self.path.starts_with(path)
  }

  /// Run command into the repository.
  pub fn run_command<I, S>(&self, command: &str, args: I) -> ::Result<bool>
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr> + Display
  {
    let output = process::inherit(command).args(args)
      .current_dir(&self.path)
      .output()?;
    Ok(output.status.success())
  }

  pub fn path_string(&self) -> String {
    format!("{}", self.path.display())
  }

  pub fn remote_url(&self) -> Option<&str> {
    self.url.as_ref().map(|s| s.as_str())
  }
}
