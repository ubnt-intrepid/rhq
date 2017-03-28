//! defines functions/types related to local repository access.

use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use shlex;

use super::query::Query;
use util::process;
use vcs;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
  path: PathBuf,
  url: Option<String>,
  #[serde(skip_serializing, skip_deserializing)]
  dry_run: bool,
}

impl Repository {
  /// Make an instance of `Repository` from local path.
  pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    Repository {
      path: path.as_ref().to_owned(),
      url: None,
      dry_run: false,
    }
  }

  pub fn from_query<P: AsRef<Path>>(root: P, query: Query, is_ssh: bool) -> ::Result<Self> {
    let root = root.as_ref();
    let path = query.to_local_path()?;
    let path = root.join(path);

    let url = query.to_url(is_ssh)?;

    Ok(Repository {
         path: path,
         url: Some(url),
         dry_run: false,
       })
  }

  pub fn set_dry_run(&mut self, dry_run: bool) {
    self.dry_run = dry_run;
  }

  pub fn is_same_local(&self, other: &Self) -> bool {
    self.path.as_path() == other.path.as_path()
  }

  pub fn do_init(&self) -> ::Result<()> {
    if self.path.is_dir() {
      println!("The repository {} has already existed.", self.path_string());
      return Ok(());
    }

    if self.dry_run {
      println!("+ git init {}'", self.path_string());
      Ok(())
    } else {
      vcs::git::init(&self.path)?;

      let url = self.url
        .as_ref()
        .ok_or("empty URL")?;
      vcs::git::set_remote(&self.path, url)?;

      Ok(())
    }
  }

  pub fn do_clone(&self, args: &[String]) -> ::Result<()> {
    if vcs::detect_from_path(&self.path).is_some() {
      println!("The repository has already cloned.");
      return Ok(());
    }

    let url = self.url
      .as_ref()
      .ok_or("empty URL")?;

    if self.dry_run {
      println!("+ git clone '{}' '{}' {}",
               url.as_str(),
               self.path.display(),
               args.join(" "));
    } else {
      vcs::git::clone(&url, &self.path, args)?;
    }
    Ok(())
  }

  pub fn run_command<S>(&self, command: &str, args: &[S]) -> ::Result<bool>
    where S: AsRef<::std::ffi::OsStr> + ::std::fmt::Display
  {
    if self.dry_run {
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
      let output = process::inherit(command).args(args)
        .current_dir(&self.path)
        .output()?;
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
