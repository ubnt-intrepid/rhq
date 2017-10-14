use std::borrow::Cow;
use std::path::{Path, PathBuf};

use glob::Pattern;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use cache::Cache;
use config::Config;
use vcs;

use super::repository::Repository;


pub struct Workspace<'a> {
    cache: Cache,
    config: Config,
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
            .or_else(|| self.config.root_dir().map(Into::into))
    }

    /// Returns a list of managed repositories.
    /// Note that this method returns None if cache has not created yet.
    pub fn repositories(&self) -> Option<&[Repository]> {
        self.cache
            .get_opt()
            .map(|cache| cache.repositories.as_slice())
    }

    pub fn scan_repositories_default(&mut self, verbose: bool, depth: Option<usize>) -> ::Result<()> {
        for root in self.config.include_dirs() {
            self.scan_repositories(root, verbose, depth)?;
        }
        Ok(())
    }

    /// Scan repositories and update state.
    pub fn scan_repositories<P: AsRef<Path>>(&mut self, root: P, verbose: bool, depth: Option<usize>) -> ::Result<()> {
        for path in collect_repositories(root, depth, self.config.exclude_patterns()) {
            let repo = Repository::from_path(path)?;
            self.add_repository(repo, verbose);
        }
        Ok(())
    }

    pub fn add_repository(&mut self, repo: Repository, verbose: bool) {
        let ref mut repos = self.cache.get_mut().repositories;
        if let Some(r) = repos.iter_mut().find(|r| r.is_same_local(&repo)) {
            if verbose {
                println!("Overwrite existed entry: {}", repo.path_string());
            }
            *r = repo;
            return;
        }

        if verbose {
            println!("Add new entry: {}", repo.path_string());
        }
        repos.push(repo);
    }

    pub fn drop_invalid_repositories(&mut self, verbose: bool) {
        let mut new_repo = Vec::new();
        for repo in &self.cache.get_mut().repositories {
            let repo = match repo.clone().refresh() {
                Some(r) => r,
                None => continue,
            };
            if self.config
                .exclude_patterns()
                .into_iter()
                .all(|ex| !ex.matches(&repo.path_string()))
            {
                new_repo.push(repo.clone());
            } else {
                if verbose {
                    println!("Dropped: {}", repo.path_string());
                }
            }
        }
        self.cache.get_mut().repositories = new_repo;
    }

    pub fn sort_repositories(&mut self) {
        self.cache
            .get_mut()
            .repositories
            .sort_by(|a, b| a.name().cmp(b.name()));
    }


    /// Save current state of workspace to cache file.
    pub fn save_cache(&mut self) -> ::Result<()> {
        self.cache.dump()?;
        Ok(())
    }
}


fn collect_repositories<P>(root: P, depth: Option<usize>, excludes: Vec<Pattern>) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    let filter = {
        let root = root.as_ref();
        move |entry: &DirEntry| {
            if entry.path() == root {
                return true;
            }
            !entry
                .path()
                .parent()
                .map(|path| vcs::detect_from_path(&path).is_some())
                .unwrap_or(false)
                && entry
                    .path()
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
    walkdir
        .into_iter()
        .filter_entry(filter)
        .filter_map(Result::ok)
        .filter(|entry| vcs::detect_from_path(entry.path()).is_some())
        .map(|entry| entry.path().into())
        .collect()
}
