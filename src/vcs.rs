pub mod darcs;
pub mod git;
pub mod hg;
pub mod pijul;

use crate::util::StrSkip;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Vcs {
    Git,
    Hg,
    Darcs,
    Pijul,
}

impl Vcs {
    pub fn do_init<P: AsRef<Path>>(self, path: P) -> Result<()> {
        match self {
            Vcs::Git => git::init(path),
            Vcs::Hg => hg::init(path),
            Vcs::Darcs => darcs::initialize(path),
            Vcs::Pijul => pijul::init(path),
        }
    }

    pub fn do_clone<P, U, I, S>(self, path: P, url: U, args: I) -> Result<()>
    where
        P: AsRef<Path>,
        U: AsRef<str>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr> + Display,
    {
        match self {
            Vcs::Git => git::clone(url, path, args),
            Vcs::Hg => hg::clone(url, path, args),
            Vcs::Darcs => darcs::clone(url, path, args),
            Vcs::Pijul => pijul::clone(url, path, args),
        }
    }

    pub fn get_remote_url<P: AsRef<Path>>(self, path: P) -> Result<Option<String>> {
        match self {
            Vcs::Git => git::get_remote_url(path),
            Vcs::Hg => hg::get_remote_url(path),
            _ => Err(anyhow!("This VCS has not supported yet")),
        }
    }

    pub fn set_remote_url(self, path: &Path, url: &str) -> Result<()> {
        match self {
            Vcs::Git => git::set_remote(path, url),
            _ => Err(anyhow!("This VCS has not supported yet")),
        }
    }
}

pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Option<Vcs> {
    [".git", ".hg", "_darcs", ".pijul"]
        .iter()
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

impl fmt::Display for Vcs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git => f.write_str("git"),
            Self::Hg => f.write_str("hg"),
            Self::Darcs => f.write_str("darcs"),
            Self::Pijul => f.write_str("pijul"),
        }
    }
}
