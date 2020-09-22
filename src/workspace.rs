use crate::{
    cache::Cache,
    config::Config,
    printer::Printer,
    query::Query,
    remote::Remote,
    repository::Repository,
    vcs::{self, Vcs},
};
use anyhow::{anyhow, Result};
use glob::Pattern;
use std::{
    fmt::Arguments,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub struct Workspace {
    cache: Cache,
    config: Config,
    printer: Printer,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let config = Config::new(None)?;
        let cache = Cache::new(&config.cache_dir())?;
        Ok(Workspace {
            cache,
            config,
            printer: Printer::default(),
        })
    }

    pub fn set_root_dir<P: Into<PathBuf>>(&mut self, root: P) {
        self.config.root_dir = root.into();
    }

    pub fn verbose_output(mut self, verbose: bool) -> Self {
        self.printer.verbose = verbose;
        self
    }

    pub fn print(&self, args: Arguments) {
        self.printer.print(args)
    }

    /// Returns a list of managed repositories.
    /// Note that this method returns None if cache has not created yet.
    pub fn repositories(&self) -> Option<&[Repository]> {
        self.cache
            .get_opt()
            .map(|cache| cache.repositories.as_slice())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn import_repositories<P: AsRef<Path>>(
        &mut self,
        root: P,
        depth: Option<usize>,
    ) -> Result<()> {
        for path in collect_repositories(root, depth, &self.config.exclude_patterns) {
            match self.new_repository_from_path(&path) {
                Ok(Some(repo)) => self.add_repository(repo),
                Ok(None) => {
                    self.printer.print(format_args!(
                        "Ignored: {} is not a repository\n",
                        path.display()
                    ));
                }
                Err(e) => self
                    .printer
                    .print(format_args!("{} {}\n", e, path.display())),
            }
        }
        Ok(())
    }

    pub fn add_repository(&mut self, repo: Repository) {
        let repos = &mut self.cache.get_mut().repositories;
        if let Some(r) = repos.iter_mut().find(|r| r.is_same_local(&repo)) {
            self.printer.print(format_args!(
                "Overwrite existed entry: {}\n",
                repo.path_string()
            ));
            *r = repo;
            return;
        }

        self.printer
            .print(format_args!("Add new entry: {}\n", repo.path_string()));
        repos.push(repo);
    }

    pub fn add_repository_if_exists(&mut self, path: &Path) -> Result<()> {
        let repo = match self.new_repository_from_path(path) {
            Ok(Some(repo)) => repo,
            Ok(None) => {
                self.printer.print(format_args!(
                    "Ignored: {} is not a repository\n",
                    path.display()
                ));
                return Ok(());
            }
            Err(e) => {
                self.printer
                    .print(format_args!("{} {}\n", e, path.display()));
                return Ok(());
            }
        };
        self.add_repository(repo);
        Ok(())
    }

    pub fn drop_invalid_repositories(&mut self) {
        let mut new_repo = Vec::new();
        for repo in &self.cache.get_mut().repositories {
            let repo = match repo.clone().refresh() {
                Some(r) => r,
                None => continue,
            };
            if self
                .config
                .exclude_patterns
                .iter()
                .all(|ex| !ex.matches(&repo.path_string()))
            {
                new_repo.push(repo.clone());
            } else {
                self.printer
                    .print(format_args!("Dropped: {}\n", repo.path_string()));
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
    pub fn save_cache(&mut self) -> Result<()> {
        self.cache.dump(&self.config.cache_dir())?;
        Ok(())
    }

    pub fn resolve_query(&self, query: &Query) -> Result<PathBuf> {
        let root = &self.config.root_dir;
        let host = query.host().unwrap_or_else(|| &self.config.host);
        let path = root.join(host).join(query.path());
        Ok(path)
    }

    pub fn default_host(&self) -> &str {
        &self.config.host
    }

    pub fn for_each_repo<F: Fn(&Repository) -> Result<()>>(&self, f: F) -> Result<()> {
        let repos = self
            .repositories()
            .ok_or_else(|| anyhow!("The cache has not initialized yet"))?;
        for repo in repos {
            f(&repo)?;
        }
        Ok(())
    }

    fn new_repository_from_path(&self, path: &Path) -> Result<Option<Repository>> {
        let vcs = match vcs::detect_from_path(&path) {
            Some(vcs) => vcs,
            None => return Ok(None),
        };
        let remote = match vcs.get_remote_url(&path)? {
            Some(remote) => remote,
            None => return Ok(None),
        };
        Repository::new(path, vcs, Remote::new(remote)).map(Some)
    }

    pub fn create_repository(&mut self, query: &Query, vcs: Vcs, is_ssh: bool) -> Result<()> {
        let path = self.resolve_query(query)?;

        self.printer.print(format_args!(
            "Creating an empty repository at \"{}\" (VCS: {:?})\n",
            path.display(),
            vcs
        ));
        if vcs::detect_from_path(&path).is_some() {
            self.printer.print(format_args!(
                "[info] The repository {} has already existed.\n",
                path.display()
            ));
            return Ok(());
        }
        vcs.do_init(&path)?;
        let remote = Remote::from_query(&query, is_ssh, self.default_host()).ok();
        if let Some(ref remote) = remote {
            vcs.set_remote_url(&path, remote.url())?;
        }
        let repo = Repository::new(path, vcs, remote)?;
        self.add_repository(repo);

        Ok(())
    }

    pub fn clone_repository(&mut self, remote: Remote, dest: &Path, vcs: Vcs) -> Result<()> {
        self.printer.print(format_args!(
            "[info] Clone from {} into {} by using {:?}\n",
            remote.url(),
            dest.display(),
            vcs,
        ));
        if vcs::detect_from_path(&dest).is_some() {
            self.printer.print(format_args!(
                "The repository {} has already existed.\n",
                dest.display()
            ));
            return Ok(());
        }
        vcs.do_clone(&dest, &remote.url(), &[] as &[String])?;
        let repo = Repository::new(dest, vcs, remote)?;
        self.add_repository(repo);
        Ok(())
    }
}

fn collect_repositories<P>(root: P, depth: Option<usize>, excludes: &[Pattern]) -> Vec<PathBuf>
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
                        let path = path.to_str().unwrap().trim_start_matches(r"\\?\");
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
