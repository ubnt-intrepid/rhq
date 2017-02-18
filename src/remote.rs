use std::path::Path;
use url::Url;
use errors::Result;
use vcs;

pub struct Remote {
  url: Url,
}

impl Remote {
  pub fn from_url(url: Url) -> Remote {
    Remote { url: url }
  }

  pub fn clone_into(&self, path: &Path, args: &[String], dry_run: bool) -> Result<()> {
    if dry_run {
      println!("clone from {:?} into {:?} (args = {:?})",
               self.url,
               path,
               args);
      Ok(())
    } else {
      vcs::git::clone(&self.url, path, args)
    }
  }
}
