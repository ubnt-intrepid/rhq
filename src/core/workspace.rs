use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use glob::Pattern;
use shellexpand;
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
  root: Option<String>,
  includes: Option<Vec<String>>,
  excludes: Option<Vec<String>>,
}

impl ConfigData {
  pub fn includes(&self) -> &[String] {
    self.includes
        .as_ref()
        .map(Vec::as_slice)
        .unwrap_or(&[])
  }

  #[allow(dead_code)]
  pub fn excludes(&self) -> &[String] {
    self.excludes
        .as_ref()
        .map(Vec::as_slice)
        .unwrap_or(&[])
  }
}

impl InitialStr for ConfigData {
  #[inline]
  fn initial_str() -> &'static str {
    include_str!("config.toml")
  }
}


pub struct Workspace<'a> {
  cache: Cache<CacheData>,
  config: Config<ConfigData>,
  root: Option<&'a Path>,
}

impl<'a> Workspace<'a> {
  pub fn new(root: Option<&'a Path>) -> ::Result<Workspace<'a>> {
    let cache = Cache::load()?;
    let config = Config::load()?;
    Ok(Workspace {
         cache: cache,
         config: config,
         root: root,
       })
  }

  /// Returns root directory of the workspace.
  pub fn root_dir(&self) -> Option<Cow<Path>> {
    self.root
        .map(Into::into)
        .or_else(|| {
                   self.config
                       .get()
                       .root
                       .as_ref()
                       .and_then(|root| util::make_path_buf(root).ok())
                       .map(Into::into)
                 })
  }

  pub fn base_dirs(&self) -> Vec<PathBuf> {
    self.config
        .get()
        .includes()
        .into_iter()
        .filter_map(|root| util::make_path_buf(&root).ok())
        .collect()
  }

  pub fn exclude_patterns(&self) -> Vec<Pattern> {
    self.config
        .get()
        .excludes()
        .into_iter()
        .filter_map(|ex| {
                      shellexpand::full(&ex)
                        .ok()
                        .map(|ex| ex.replace(r"\", "/"))
                        .and_then(|ex| Pattern::new(&ex).ok())
                    })
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
        .flat_map(|root| collect_repositories_from(root, depth, self.exclude_patterns()))
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


fn collect_repositories_from<P>(root: P, depth: Option<usize>, excludes: Vec<Pattern>) -> Vec<PathBuf>
  where P: AsRef<Path>
{
  let filter = {
    let root = root.as_ref();
    move |entry: &DirEntry| {
      if entry.path() == root {
        return true;
      }
      !entry.path()
            .parent()
            .map(|path| vcs::detect_from_path(&path).is_some())
            .unwrap_or(false) &&
      entry.path()
           .canonicalize()
           .ok()
           .map(|path| {
                  let path = path.to_str().unwrap().trim_left_matches(r"\\?\");
                  excludes.iter().all(|ex| !ex.matches(path))
                })
           .unwrap_or(false)
    }
  };

  let mut walkdir = WalkDir::new(root.as_ref()).follow_links(true);
  if let Some(depth) = depth {
    walkdir = walkdir.max_depth(depth);
  }

  walkdir.into_iter()
         .filter_entry(filter)
         .filter_map(|entry| {
                       entry.ok()
                            .and_then(|entry| fs::canonicalize(entry.path()).ok())
                     })
         .filter(|ref path| vcs::detect_from_path(path).is_some())
         .collect()
}
