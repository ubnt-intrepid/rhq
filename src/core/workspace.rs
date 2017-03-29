use std::path::{Path, PathBuf};
use walkdir::{WalkDir, WalkDirIterator};

use app::{Cache, Config, InitialStr};
use vcs;
use util;

use super::repository::Repository;


// inner representation of cache format.
#[derive(Default, Serialize, Deserialize)]
struct CacheData {
  repositories: Vec<Repository>,
}


#[derive(Serialize, Deserialize)]
struct ConfigData {
  root: String,
  supplements: Option<Vec<String>>,
}

impl ConfigData {
  pub fn roots(&self) -> Vec<&str> {
    let mut result = vec![self.root.as_str()];
    if let Some(ref supp) = self.supplements {
      result.extend(supp.into_iter().map(|p| p.as_str()));
    }
    result
  }
}

impl InitialStr for ConfigData {
  #[inline]
  fn initial_str() -> &'static str {
    include_str!("config.toml")
  }
}


pub struct Workspace {
  cache: Cache<CacheData>,
  config: Config<ConfigData>,
  root: Option<String>,
}

impl Workspace {
  pub fn new(root: Option<&str>) -> ::Result<Workspace> {
    let cache = Cache::load()?;
    let config = Config::load()?;
    Ok(Workspace {
         cache: cache,
         config: config,
         root: root.map(ToOwned::to_owned),
       })
  }

  /// Returns root directory of the workspace.
  pub fn get_root(&self) -> Option<PathBuf> {
    self.root
      .as_ref()
      .and_then(|s| util::make_path_buf(s).ok())
      .or_else(|| util::make_path_buf(&self.config.get().root).ok())
  }

  pub fn add_repository(&mut self, repo: Repository) {
    let ref mut repos = self.cache.get_mut().repositories;
    if let Some(mut r) = repos.iter_mut().find(|r| r.is_same_local(&repo)) {
      *r = repo;
      return;
    }
    repos.push(repo);
  }

  /// Returns a list of managed repositories.
  /// Note that this method returns None if cache has not created yet.
  pub fn repositories(&self) -> Option<&[Repository]> {
    self.cache.get_opt().map(|cache| cache.repositories.as_slice())
  }

  pub fn scan_repositories(&mut self, verbose: bool) -> ::Result<()> {
    let mut repos = Vec::new();
    for root in self.config
          .get()
          .roots()
          .into_iter()
          .filter_map(|root| util::make_path_buf(&root).ok()) {
      for path in self.collect_repositories_from(root) {
        if verbose {
          println!("Found: {}", path.display());
        }
        let repo = Repository::from_path(path);
        repos.push(repo);
      }
    }

    self.cache.get_mut().repositories = repos;
    Ok(())
  }

  pub fn save_cache(&self) -> ::Result<()> {
    self.cache.dump()?;
    Ok(())
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
