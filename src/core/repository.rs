//! defines functions/types related to local repository access.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use super::query::Query;
use util::process;
use vcs;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
  path: PathBuf,
  url: Option<String>,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    Repository {
      path: path.as_ref().to_owned(),
      url: None,
    }
  }

  pub fn from_query<P: AsRef<Path>>(root: P, query: Query, is_ssh: bool) -> ::Result<Self> {
    let root = root.as_ref();
    let path = query.to_local_path()?;
    let path = root.join(path);

    let url = query.to_url(is_ssh)?;

    Ok(Repository {
         path: path,
         url: Some(url),
       })
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

  pub fn do_init(&self) -> ::Result<()> {
    if self.path.is_dir() {
      println!("The repository {} has already existed.", self.path_string());
      return Ok(());
    }
    vcs::git::init(&self.path)?;
    Ok(())
  }

  pub fn do_clone<I, S>(&self, args: I) -> ::Result<()>
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr> + Display
  {
    if let Some(_) = vcs::detect_from_path(&self.path) {
      println!("The repository has already cloned.");
      return Ok(());
    }

    let url = self.url
      .as_ref()
      .ok_or("empty URL")?;
    vcs::git::clone(&url, &self.path, args)?;
    Ok(())
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

  #[cfg(windows)]
  pub fn path_string(&self) -> String {
    self.path.to_string_lossy().replace("\\", "/")
  }

  #[cfg(not(windows))]
  pub fn path_string(&self) -> String {
    format!("{}", self.path.display())
  }

  pub fn url_string(&self) -> Option<String> {
    self.url.as_ref().map(|url| url.as_str().to_owned())
  }
}
