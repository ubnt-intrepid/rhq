//! defines functions/types related to local repository access.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use util::{self, process};
use vcs::{self, Vcs};


/// Information of remote repository
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Remote {
    url: String,
}

impl Remote {
    pub fn new<S: Into<String>>(url: S) -> Remote {
        // TODO: verify URL
        Remote { url: url.into() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}


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
    pub fn from_path<P: AsRef<Path>>(path: P) -> ::Result<Self> {
        let path = util::canonicalize_pretty(path)?;
        let name = path.file_name()
                       .map(|s| s.to_string_lossy().into_owned())
                       .ok_or("cannot determine repository name")?;
        let vcs = vcs::detect_from_path(&path).ok_or("cannot detect VCS")?;
        let remote = vcs.get_remote_url(&path)?.map(Remote::new);
        Ok(Repository {
            name: name,
            path: path,
            vcs: vcs,
            remote: remote,
        })
    }

    pub fn from_path_with_remote<P: AsRef<Path>>(path: P, remote: Remote) -> ::Result<Self> {
        let path = util::canonicalize_pretty(path)?;
        let name = path.file_name()
                       .map(|s| s.to_string_lossy().into_owned())
                       .ok_or("cannot determine repository name")?;
        let vcs = vcs::detect_from_path(&path).ok_or("cannot detect VCS")?;
        Ok(Repository {
            name: name,
            path: path,
            vcs: vcs,
            remote: Some(remote),
        })
    }

    /// Check existence of repository and drop if not exists.
    pub fn refresh(self) -> Option<Self> {
        Self::from_path(self.path).ok()
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
        let output = process::inherit(command)
            .args(args)
            .current_dir(&self.path)
            .output()?;
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
