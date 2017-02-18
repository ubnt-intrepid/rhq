//! defines functions/types related to local repository access.

use std::path::{Path, PathBuf};
use walkdir::{WalkDir, WalkDirIterator};

use errors::Result;
use query::Query;
use remote::Remote;
use vcs;


pub struct Repository {
  path: PathBuf,
  remote: Option<Remote>,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    Repository {
      path: path.as_ref().to_owned(),
      remote: None,
    }
  }

  /// Make an instance of `Repository` from query.
  pub fn new<P: AsRef<Path>>(root: P, query: Query) -> Result<Self> {
    // guess local path from query.
    let url = query.to_url()?;
    let mut path = url.host_str().map(ToOwned::to_owned).ok_or("url.host() is empty")?;
    path += url.path().trim_right_matches(".git");
    let path = root.as_ref().join(path);

    // guess remote repository.
    let remote = Remote::from_url(url);

    Ok(Repository {
      path: path,
      remote: Some(remote),
    })
  }

  /// Perform to clone repository into local path.
  pub fn do_clone(&self, args: &[String], dry_run: bool) -> Result<()> {
    if vcs::detect_from_path(&self.path).is_some() {
      println!("The repository has already cloned.");
      return Ok(());
    }
    let ref remote = self.remote.as_ref().ok_or("empty remote")?;
    remote.clone_into(&self.path, args, dry_run)
  }

  pub fn is_same_local(&self, other: &Self) -> bool {
    self.path.as_path() == other.path.as_path()
  }

  #[cfg(windows)]
  pub fn path_string(&self) -> String {
    self.path.to_string_lossy().replace("\\", "/")
  }

  #[cfg(not(windows))]
  pub fn path_string(&self) -> String {
    format!("{}", self.path.display())
  }
}

pub fn collect_from<P: AsRef<Path>>(root: P) -> Vec<Repository> {
  WalkDir::new(root.as_ref())
    .follow_links(true)
    .into_iter()
    .filter_entry(|ref entry| {
      if entry.path() == root.as_ref() {
        return true;
      }
      entry.path()
        .parent()
        .map(|path| vcs::detect_from_path(&path).is_none())
        .unwrap_or(true)
    })
    .filter_map(|e| e.ok())
    .filter(|ref entry| vcs::detect_from_path(entry.path()).is_some())
    .map(|entry| Repository::from_path(entry.path()))
    .collect()
}
