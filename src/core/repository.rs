//! defines functions/types related to local repository access.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use util::{self, process};
use vcs::{self, Vcs};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
  path: PathBuf,
  vcs: Vcs,
  url: Option<String>,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> ::Result<Self> {
    let path = util::canonicalize_pretty(path)?;
    let vcs = vcs::detect_from_path(&path).ok_or("cannot detect VCS")?;
    // TODO: get remote URL from repository
    Ok(Repository {
         path: path,
         vcs: vcs,
         url: None,
       })
  }

  pub fn from_path_with_remote<P: AsRef<Path>, S: AsRef<str>>(path: P, url: S) -> ::Result<Self> {
    let path = util::canonicalize_pretty(path)?;
    let vcs = vcs::detect_from_path(&path).ok_or("cannot detect VCS")?;
    // TODO: verify URL
    Ok(Repository {
         path: path,
         vcs: vcs,
         url: Some(url.as_ref().into()),
       })
  }

  /// Check existence of repository and drop if not exists.
  pub fn refresh(self) -> Option<Self> {
    if vcs::detect_from_path(&self.path).is_some() {
      Some(self)
    } else {
      None
    }
  }

  pub fn is_same_local(&self, other: &Self) -> bool {
    self.path.as_path() == other.path.as_path()
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
