//! defines functions/types related to local repository access.

use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, WalkDirIterator};
use shlex;

use vcs;
use process;


pub struct Repository {
  path: PathBuf,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    Repository { path: path.as_ref().to_owned() }
  }

  pub fn is_same_local(&self, other: &Self) -> bool {
    self.path.as_path() == other.path.as_path()
  }

  pub fn run_command<S>(&self, command: &str, args: &[S], dry_run: bool) -> ::std::io::Result<bool>
    where S: AsRef<::std::ffi::OsStr> + ::std::fmt::Display
  {
    if dry_run {
      println!("({}) {}{}",
               self.path.display(),
               command,
               args.iter().fold(String::new(), |a, s| {
                 format!("{} {}",
                         a,
                         shlex::quote(s.as_ref().to_string_lossy().borrow()))
               }));
      Ok(true)
    } else {
      let output: ::std::process::Output =
        process::inherit(command).args(args).current_dir(&self.path).output()?;
      Ok(output.status.success())
    }
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
