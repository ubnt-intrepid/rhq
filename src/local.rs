//! defines functions/types related to local repository access.

use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry, WalkDirIterator};
use url::Url;
use errors;

pub fn collect_repositories<P: AsRef<Path>>(root: P) -> Vec<DirEntry> {
  WalkDir::new(root.as_ref())
    .follow_links(true)
    .into_iter()
    .filter_entry(|ref entry| {
      if entry.path() == root.as_ref() {
        return true;
      }
      entry.path()
        .parent()
        .map(|path| detect_vcs(&path).is_none())
        .unwrap_or(true)
    })
    .filter_map(Result::ok)
    .filter(|ref entry| detect_vcs(entry.path()).is_some())
    .collect()
}

fn detect_vcs(path: &Path) -> Option<&'static str> {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .find(|vcs| path.join(vcs).exists())
    .map(|s| *s)
}

pub fn make_path_from_url<P: AsRef<Path>>(url: &Url, root: P) -> errors::Result<PathBuf> {
  let mut path = url.host_str().map(ToOwned::to_owned).ok_or("url.host() is empty".to_owned())?;
  path += url.path().trim_right_matches(".git");
  Ok(root.as_ref().join(path))
}
