use std::borrow::Borrow;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};
use shlex;
use config::Config;
use errors::Result;
use remote;
use local;

pub struct Client {
  config: Config,
}

impl Client {
  /// Creates a new instance of rhq client.
  pub fn new() -> Result<Client> {
    let config = Config::load()?;
    Ok(Client { config: config })
  }

  /// Performs to clone repository from query.
  ///
  /// If `query` is omitted, use standard input to take queries.
  pub fn command_clone(&self, query: Option<&str>, arg: Option<&str>) -> Result<()> {
    let args = arg.and_then(|a| shlex::split(a)).unwrap_or_default();

    if let Some(query) = query {
      return clone_repository(self.default_root(), query, &args);
    }

    let stdin = io::stdin();
    for ref query in stdin.lock().lines().filter_map(|l| l.ok()) {
      clone_repository(self.default_root(), query, &args)?;
    }
    Ok(())
  }

  /// List all of local repositories's path managed from rhq.
  ///
  /// On Windows, the path separaters are replated to '/'.
  pub fn command_list(&self) -> Result<()> {
    for root in &self.config.roots {
      for repo in local::collect_repositories(root) {
        #[cfg(windows)]
        println!("{}", repo.path().to_string_lossy().replace("\\", "/"));
        #[cfg(not(windows))]
        println!("{}", repo.path().display());
      }
    }
    Ok(())
  }

  /// Returns the path of directory to determine cloned repository's path.
  ///
  /// TODO: replace definition into `Config`
  pub fn default_root(&self) -> &Path {
    self.config.roots.iter().next().expect("config.roots is empty")
  }

  /// Returns the reference of configuration.
  pub fn command_config(&self) -> Result<()> {
    println!("{}", self.config);
    Ok(())
  }
}

fn clone_repository(root: &Path, query: &str, args: &[String]) -> Result<()> {
  let url = remote::build_url(query)?;

  let path = local::make_path_from_url(&url, root)?;
  for repo in local::collect_repositories(root) {
    if path == repo.path() {
      println!("The repository has already cloned.");
      return Ok(());
    }
  }

  debug!("clone from {:?} into {:?} (args = {:?})", url, path, args);
  Command::new("git").arg("clone")
    .arg(url.as_str())
    .arg(path.to_string_lossy().borrow() as &str)
    .args(&args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()?;
  Ok(())
}
