//! defines functions/types related to local repository access.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use remote::Remote;
use util::{self, process};
use vcs::Vcs;

/// local repository
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    /// name of repository
    name: String,
    /// (canonicalized) absolute path of the repository
    path: PathBuf,
    /// used version control system
    vcs: Vcs,
    /// information of remote repository
    #[serde(skip_serializing_if = "Option::is_none")]
    remote: Option<Remote>,
}

impl Repository {
    /// Make an instance of `Repository` from local path.
    pub fn new<P: AsRef<Path>, R: Into<Option<Remote>>>(path: P, vcs: Vcs, remote: R) -> ::Result<Self> {
        let path = util::canonicalize_pretty(path)?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or("cannot determine repository name")?;
        Ok(Repository {
            name,
            path,
            vcs,
            remote: remote.into(),
        })
    }

    /// Check existence of repository and drop if not exists.
    pub fn refresh(self) -> Option<Self> {
        match self.vcs.get_remote_url(&self.path) {
            Ok(url) => Self::new(self.path, self.vcs, url.map(Remote::new)).ok(),
            _ => None,
        }
    }

    pub fn is_same_local(&self, other: &Self) -> bool {
        self.path.as_path() == other.path.as_path()
    }

    pub fn is_contained<P: AsRef<Path>>(&self, path: P) -> bool {
        self.path.starts_with(path)
    }

    /// Run command into the repository.
    pub fn run_command<I, S>(&self, command: &str, args: I) -> ::Result<bool>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr> + Display,
    {
        let output = process::inherit(command).args(args).current_dir(&self.path).output()?;
        Ok(output.status.success())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path_string(&self) -> String {
        format!("{}", self.path.display())
    }

    pub fn remote(&self) -> Option<&Remote> {
        self.remote.as_ref()
    }
}
