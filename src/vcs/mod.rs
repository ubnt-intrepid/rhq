pub mod git;
pub mod hg;
pub mod darcs;
pub mod pijul;

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use util::StrSkip;


#[derive(Debug, Clone, Copy)]
pub enum Vcs {
  Git,
  Hg,
  Darcs,
  Pijul,
}

impl Vcs {
  #[inline]
  pub fn possible_values() -> &'static [&'static str] {
    static POSSIBLE_VALUES: [&'static str; 4] = ["git", "hg", "darcs", "pijul"];
    &POSSIBLE_VALUES
  }

  pub fn do_init<P: AsRef<Path>>(self, path: P) -> ::Result<()> {
    match self {
      Vcs::Git => git::init(path),
      Vcs::Hg => hg::init(path),
      Vcs::Darcs => darcs::initialize(path),
      Vcs::Pijul => pijul::init(path),
    }
  }

  pub fn do_clone<P, U, I, S>(self, path: P, url: U, args: I) -> ::Result<()>
    where P: AsRef<Path>,
          U: AsRef<str>,
          I: IntoIterator<Item = S>,
          S: AsRef<OsStr> + Display
  {
    match self {
      Vcs::Git => git::clone(url, path, args),
      Vcs::Hg => hg::clone(url, path, args),
      Vcs::Darcs => darcs::clone(url, path, args),
      Vcs::Pijul => pijul::clone(url, path, args),
    }
  }
}

pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Option<Vcs> {
  [".git", ".hg", "_darcs", ".pijul"]
    .into_iter()
    .find(|vcs| path.as_ref().join(vcs).exists())
    .and_then(|s| s.skip(1).parse().ok())
}

impl FromStr for Vcs {
  type Err = String;
  fn from_str(s: &str) -> ::std::result::Result<Vcs, String> {
    match s {
      "git" => Ok(Vcs::Git),
      "hg" => Ok(Vcs::Hg),
      "darcs" => Ok(Vcs::Darcs),
      "pijul" => Ok(Vcs::Pijul),
      s => Err(format!("{} is invalid string", s)),
    }
  }
}
