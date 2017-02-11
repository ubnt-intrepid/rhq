use std::path::Path;
use url::Url;
use errors;

pub trait VcsBackend {
  fn clone(&mut self, url: Url, dest: &Path) -> errors::Result<()>;
}

pub struct GitBackend;

impl VcsBackend for GitBackend {
  fn clone(&mut self, url: Url, dest: &Path) -> errors::Result<()> {
    info!("url={}, dest={}", url.as_str(), dest.display());
    Ok(())
  }
}
