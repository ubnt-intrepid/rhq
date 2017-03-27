use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use shellexpand;
use walkdir::WalkDir;
use walkdir::WalkDirIterator;

use app::config::Config;
use core::repository::Repository;
use core::query::Query;
use vcs;
use util;


pub struct Workspace {
  config: Config,
  clone_args: Vec<String>,
  dry_run: bool,
  root: Option<String>,
}

impl Workspace {
  pub fn new(config: Config) -> Workspace {
    Workspace {
      config: config,
      dry_run: false,
      clone_args: Vec::new(),
      root: None,
    }
  }

  pub fn set_dry_run(&mut self, dry_run: bool) {
    self.dry_run = dry_run;
  }

  pub fn set_clone_args<I, S>(&mut self, args: I)
    where I: IntoIterator<Item = S>,
          S: Into<String>
  {
    self.clone_args = args.into_iter().map(Into::into).collect();
  }

  pub fn set_root(&mut self, root: &str) {
    self.root = Some(root.to_owned());
  }

  fn root_path(&self) -> PathBuf {
    self.root
      .as_ref()
      .and_then(|s| util::make_path_buf(s).ok())
      .unwrap_or_else(|| self.config.root.clone())
  }

  // Create an empty repository into workspace.
  pub fn add_new_repository(&self, query: Query, is_ssh: bool) -> ::Result<()> {
    let root = self.root_path();

    let local_path = root.join(query.to_local_path()?);
    if local_path.is_dir() {
      println!("The directory {} has already existed.",
               local_path.display());
      return Ok(());
    }

    if self.dry_run {
      println!("launch 'git init {}'", local_path.display());
      Ok(())
    } else {
      vcs::init_repo(&local_path)?;
      vcs::set_remote(&local_path, &query.to_url(is_ssh)?)?;
      Ok(())
    }
  }

  pub fn clone_repository(&self, query: Query, is_ssh: bool) -> ::Result<()> {
    let root = self.root_path();

    let path = query.to_local_path()?;
    let path = root.join(path);

    let url = query.to_url(is_ssh)?;
    if vcs::detect_from_path(&path).is_some() {
      println!("The repository has already cloned.");
      return Ok(());
    }
    if self.dry_run {
      println!("[debug] git clone '{}' '{}' {}",
               url.as_str(),
               path.display(),
               self.clone_args.join(" "));
    } else {
      vcs::git::clone(&url, &path, &self.clone_args)?;
    }
    Ok(())
  }


  /// Collect managed repositories
  pub fn repositories(&self) -> Vec<Repository> {
    let mut result = Vec::new();
    for root in self.config.roots() {
      for repo in collect_repositories_from(root) {
        result.push(repo);
      }
    }
    result
  }
}


fn collect_repositories_from<P: AsRef<Path>>(root: P) -> Vec<Repository> {
  WalkDir::new(root.as_ref())
    .follow_links(true)
    .into_iter()
    .filter_entry(|ref entry| {
      if entry.path() == root.as_ref() {
        return true;
      }
      entry.path()
        .parent()
        .map(|path| vcs::detect_from_path(&path).is_none())
        .unwrap_or(true)
    })
    .filter_map(|e| e.ok())
    .filter(|ref entry| vcs::detect_from_path(entry.path()).is_some())
    .map(|entry| Repository::from_path(entry.path()))
    .collect()
}
