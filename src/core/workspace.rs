use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

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
  pub fn root_dir(&self) -> Option<PathBuf> {
    self.root
        .as_ref()
        .and_then(|s| util::make_path_buf(s).ok())
        .or_else(|| util::make_path_buf(&self.config.get().root).ok())
  }

  pub fn base_dirs(&self) -> Vec<PathBuf> {
    self.config
        .get()
        .roots()
        .into_iter()
        .filter_map(|root| util::make_path_buf(&root).ok())
        .collect()
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
    self.cache
        .get_opt()
        .map(|cache| cache.repositories.as_slice())
  }

  /// Scan repositories and update state.
  pub fn scan_repositories(&mut self, verbose: bool, prune: bool, depth: Option<usize>) -> ::Result<()> {
    let mut repos = Vec::new();
    for repo in self.collect_base_dirs(depth) {
      if verbose {
        println!("Found at {}", repo.path_string());
      }
      repos.push(repo);
    }

    let outside_repos = self.collect_outsides();
    for repo in outside_repos {
      if prune {
        println!("Dropped: {}", repo.path_string());
      } else {
        if verbose {
          println!("Found at outside: {}", repo.path_string());
        }
        repos.push(repo);
      }
    }

    self.cache.get_mut().repositories = repos;
    Ok(())
  }

  /// Save current state of workspace to cache file.
  pub fn save_cache(&self) -> ::Result<()> {
    self.cache.dump()?;
    Ok(())
  }

  /// Collect repositories located at inside of base directories
  fn collect_base_dirs(&self, depth: Option<usize>) -> Vec<Repository> {
    self.base_dirs()
        .into_iter()
        .flat_map(|root| collect_repositories_from(root, depth))
        .filter_map(|path| Repository::from_path(path).ok())
        .collect()
  }

  /// Collect managed repositories located at outside of base directories
  fn collect_outsides(&self) -> Vec<Repository> {
    let cache = match self.cache.get_opt() {
      Some(cache) => cache,
      None => return Vec::new(),
    };

    let mut repos = Vec::with_capacity(cache.repositories.len());
    for repo in cache.repositories.clone() {
      let under_management = self.base_dirs()
                                 .into_iter()
                                 .any(|root| repo.is_contained(root));
      if !under_management && repo.is_vcs() {
        repos.push(repo);
      }
    }
    repos
  }
}


fn collect_repositories_from<P: AsRef<Path>>(root: P, depth: Option<usize>) -> Vec<PathBuf> {
  let filter = |entry: &DirEntry| {
    entry.path() == root.as_ref() ||
    entry.path()
         .parent()
         .map(|path| vcs::detect_from_path(&path).is_none())
         .unwrap_or(true)
  };

  let mut walkdir = WalkDir::new(root.as_ref()).follow_links(true);
  if let Some(depth) = depth {
    walkdir = walkdir.max_depth(depth);
  }
  walkdir.into_iter()
         .filter_entry(filter)
         .filter_map(|e| e.ok())
         .filter(|ref entry| vcs::detect_from_path(entry.path()).is_some())
         .map(|entry| entry.path().into())
         .collect()
}
