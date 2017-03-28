use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use walkdir::WalkDirIterator;

use app::config::Config;
use app::cache::Cache;
use core::repository::Repository;
use core::query::Query;
use vcs;
use util;

// inner representation of cache format.
#[derive(Default, Serialize, Deserialize)]
struct Inner {
  repositories: Vec<Repository>,
}

#[allow(dead_code)]
pub struct Workspace {
  cache: Cache,
  config: Config,
  clone_args: Vec<String>,
  dry_run: bool,
  root: Option<String>,
}

impl Workspace {
  pub fn new(cache: Cache, config: Config) -> Workspace {
    Workspace {
      cache: cache,
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
      .unwrap_or_else(|| util::make_path_buf(&self.config.root).unwrap())
  }

  // Create an empty repository into workspace.
  pub fn add_new_repository(&self, query: Query, is_ssh: bool) -> ::Result<()> {
    let root = self.root_path();
    let mut repository = Repository::from_query(root, query, is_ssh)?;
    repository.set_dry_run(self.dry_run);
    repository.do_init()
  }

  pub fn clone_repository(&self, query: Query, is_ssh: bool) -> ::Result<()> {
    let root = self.root_path();
    let mut repository = Repository::from_query(root, query, is_ssh)?;
    repository.set_dry_run(self.dry_run);
    repository.do_clone(&self.clone_args)
  }

  /// Collect managed repositories
  pub fn repositories(&mut self) -> ::Result<Vec<Repository>> {
    if let Some(cache) = self.cache.get_value::<Inner>()? {
      debug!("Workspace::repositories - use cache");
      Ok(cache.repositories.clone())
    } else {
      debug!("Workspace::repositories - collect directories from roots");
      let repos = self.collect_repositories(false)?;
      self.cache.set_value(Inner { repositories: repos.clone() })?;
      self.cache.dump()?;
      Ok(repos)
    }
  }

  pub fn refresh_cache(&mut self, verbose: bool) -> ::Result<()> {
    let mut inner = Inner::default();
    inner.repositories = self.collect_repositories(verbose)?;
    self.cache.set_value(inner)?;
    self.cache.dump()?;
    Ok(())
  }

  fn collect_repositories(&self, verbose: bool) -> ::Result<Vec<Repository>> {
    let mut repos = Vec::new();
    for root in self.config.roots() {
      if let Ok(root) = util::make_path_buf(&root) {
        for path in self.collect_repositories_from(root) {
          if verbose {
            println!("Found: {}", path.display());
          }
          let repo = Repository::from_path(path);
          repos.push(repo);
        }
      }
    }
    Ok(repos)
  }

  fn collect_repositories_from<P: AsRef<Path>>(&self, root: P) -> Vec<PathBuf> {
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
      .map(|entry| entry.path().into())
      .collect()
  }
}
