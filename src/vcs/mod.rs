pub mod git;

use std::path::Path;

pub fn detect_vcs(path: &Path) -> Option<&'static str> {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .find(|vcs| path.join(vcs).exists())
    .map(|s| *s)
}
