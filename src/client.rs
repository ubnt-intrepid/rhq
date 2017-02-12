use config::Config;
use errors::Result;
use remote;
use local;

pub struct Client {
  config: Config,
}

impl Client {
  pub fn new() -> Result<Client> {
    let config = Config::load()?;
    Ok(Client { config: config })
  }

  pub fn clone_repository(&self, query: &str, args: Vec<&str>) -> Result<()> {
    let url = remote::build_url(query)?;

    let path = local::make_path_from_url(&url, &self.config.root)?;
    for repo in local::collect_repositories(&self.config.root) {
      if path == repo.path() {
        println!("The repository has already cloned.");
        return Ok(());
      }
    }

    println!("[debug] clone: {:?} => {:?} (args = {:?})", url, path, args);
    // TODO: perform `git clone`.
    Ok(())
  }

  pub fn list_repositories(&self) -> Result<()> {
    for repo in local::collect_repositories(&self.config.root) {
      println!("{}", repo.path().display());
    }
    Ok(())
  }
}
