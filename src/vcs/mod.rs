use std::path::Path;
use url::Url;

use errors::Result;
use query::Query;
use vcs;

pub mod git;


#[derive(Debug)]
pub enum Vcs {
  Git,
  Subversion,
  Mercurial,
  Darcs,
}

impl ::std::str::FromStr for Vcs {
  type Err = String;
  fn from_str(s: &str) -> ::std::result::Result<Vcs, String> {
    match s {
      "git" => Ok(Vcs::Git),
      "svn" => Ok(Vcs::Subversion),
      "hg" => Ok(Vcs::Mercurial),
      "darcs" => Ok(Vcs::Darcs),
      s => Err(format!("{} is invalid string", s)),
    }
  }
}

trait StrSkip {
  fn skip<'a>(&'a self, n: usize) -> &'a str;
}

impl StrSkip for str {
  fn skip<'a>(&'a self, n: usize) -> &'a str {
    let mut s = self.chars();
    for _ in 0..n {
      s.next();
    }
    s.as_str()
  }
}

#[test]
fn test_skipped_1() {
  assert_eq!("hoge".skip(1), "oge");
  assert_eq!("あいueo".skip(1), "いueo");
}

pub fn detect_from_path(path: &Path) -> Option<Vcs> {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .find(|vcs| path.join(vcs).exists())
    .and_then(|s| s.skip(1).parse().ok())
}

pub fn detect_from_remote(_: &Url) -> Option<Vcs> {
  None
}

/// Perform to clone repository into local path.
pub fn clone_from_query<P, S>(query: Query, root: P, args: &[S], dry_run: bool) -> Result<()>
  where P: AsRef<Path>,
        S: AsRef<::std::ffi::OsStr> + ::std::fmt::Display
{
  let path = query.to_local_path()?;
  let path = root.as_ref().join(path);
  let url = query.to_url()?;
  if vcs::detect_from_path(&path).is_some() {
    println!("The repository has already cloned.");
    return Ok(());
  }
  if dry_run {
    println!("[debug] git clone '{}' '{}' {}",
             url.as_str(),
             path.display(),
             args.iter().fold(String::new(), |s, a| format!("{} {}", s, a)));
    Ok(())
  } else {
    vcs::git::clone(&url, &path, args)
  }
}
