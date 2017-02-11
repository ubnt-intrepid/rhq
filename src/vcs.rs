use std::path::Path;
use url::Url;
use errors;

pub trait VcsStatus {}

pub trait VcsBackend {
  type Status: VcsStatus;

  fn clone(&mut self, url: Url, dest: &Path) -> errors::Result<()>;
  fn status(&mut self, path: &Path) -> errors::Result<Self::Status>;
}


#[derive(Debug, Default)]
pub struct GitStatus {}

impl VcsStatus for GitStatus {}

pub struct GitBackend;

impl VcsBackend for GitBackend {
  type Status = GitStatus;

  fn clone(&mut self, url: Url, dest: &Path) -> errors::Result<()> {
    info!("url={}, dest={}", url.as_str(), dest.display());
    Ok(())
  }

  fn status(&mut self, path: &Path) -> errors::Result<GitStatus> {
    info!("path = {}", path.display());
    Ok(GitStatus::default())
  }
}
