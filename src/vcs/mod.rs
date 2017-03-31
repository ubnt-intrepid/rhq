pub mod git;

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use url::Url;
use util::StrSkip;


#[derive(Debug, Clone, Copy)]
pub enum Vcs {
  Git,
  Subversion,
  Mercurial,
  Darcs,
}

impl Vcs {
  pub fn do_init<P: AsRef<Path>>(self, path: P) -> ::Result<()> {
    match self {
      Vcs::Git => git::init(path)?,
      _ => Err(format!("{:?} has not supported yet.", self))?,
    }
    Ok(())
  }

  pub fn do_clone<P, U, I, S>(self, path: P, url: U, args: I) -> ::Result<()>
    where P: AsRef<Path>,
          U: AsRef<str>,
          I: IntoIterator<Item = S>,
          S: AsRef<OsStr> + Display
  {
    match self {
      Vcs::Git => git::clone(url.as_ref(), path, args)?,
      _ => Err(format!("{:?} has not supported yet.", self))?,
    }

    Ok(())
  }
}

pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Option<Vcs> {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .find(|vcs| path.as_ref().join(vcs).exists())
    .and_then(|s| s.skip(1).parse().ok())
}

impl FromStr for Vcs {
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
