//! defines functions/types related to local repository access.

use std::path::Path;
use walkdir::{WalkDir, DirEntry, WalkDirIterator};

pub fn list_repositories<P: AsRef<Path>>(root: P) -> Vec<DirEntry> {
  WalkDir::new(root)
    .follow_links(true)
    .into_iter()
    .filter_entry(|ref entry| {
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
