use std::path::Path;
use url::Url;
use errors::Result;
use query::Query;
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

/// Perform to clone repository into local path.
pub fn do_clone<P: AsRef<Path>>(query: Query,
                                root: P,
                                args: &[String],
                                dry_run: bool)
                                -> Result<()> {
  let path = query.to_local_path()?;
  let path = root.as_ref().join(path);
  let remote = query.to_remote()?;
  if vcs::detect_from_path(&path).is_some() {
    println!("The repository has already cloned.");
    return Ok(());
  }
  remote.clone_into(&path, args, dry_run)
}
