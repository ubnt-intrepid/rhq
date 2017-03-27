use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::Path;
use url::Url;

use util::process;

pub fn clone<S: AsRef<OsStr>>(url: &str, path: &Path, args: &[S]) -> ::Result<()> {
  process::inherit("git")
    .arg("clone")
    .args(&[url, path.to_string_lossy().borrow()])
    .args(args)
    .status()
    .map(|_| ())
    .map_err(Into::into)
}

pub fn get_upstream_url<P: Clone + AsRef<Path>>(repo_path: P) -> ::Result<Url> {
  // 1. get current branch name.
  let output = process::piped("git").current_dir(repo_path.clone())
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()?;
  if !output.status.success() {
    Err("failed to get branch name")?;
  }
  let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();

  // 2. get remote name of upstream ref
  let arg = format!("{}@{{upstream}}", branch);
  let output = process::piped("git").current_dir(repo_path.clone())
    .args(&["rev-parse", "--abbrev-ref", &arg])
    .output()?;
  if !output.status.success() {
    Err(format!("failed to get upstream name: {}",
                repo_path.as_ref().display()))?;
  }
  let upstream = String::from_utf8_lossy(&output.stdout)
    .trim()
    .trim_right_matches(&format!("/{}", branch))
    .to_owned();

  // 3. get remote URL of upstream ref
  let output = process::piped("git").current_dir(repo_path)
    .args(&["remote", "get-url", &upstream])
    .output()?;
  if !output.status.success() {
    Err("failed to get remote URL")?;
  }
  let url = String::from_utf8_lossy(&output.stdout).trim().to_owned();

  Url::parse(&url).map_err(Into::into)
}
