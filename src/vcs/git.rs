use std::ffi::OsStr;
use std::path::Path;
use url::Url;
use util::process;


pub fn init<P: AsRef<Path>>(path: P) -> ::Result<()> {
  process::inherit("git")
    .arg("init")
    .arg(path.as_ref().as_os_str())
    .status()
    .map_err(Into::into)
    .and_then(|st| match st.code() {
                Some(0) => Ok(()),
                st => Err(format!("command 'git' is exited with return code {:?}.", st).into()),
              })
}

pub fn clone<P, U, I, S>(url: U, path: P, args: I) -> ::Result<()>
  where P: AsRef<Path>,
        U: AsRef<str>,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>
{
  let path = format!("{}", path.as_ref().display());
  process::inherit("git")
    .arg("clone")
    .args(&[url.as_ref(), &path])
    .args(args)
    .status()
    .map_err(Into::into)
    .and_then(|st| match st.code() {
                Some(0) => Ok(()),
                st => Err(format!("command 'git' is exited with return code {:?}.", st).into()),
              })
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

pub fn set_remote<P: AsRef<Path>>(path: P, url: &str) -> ::Result<()> {
  let st = process::piped("git").args(&["remote", "add", "origin", url])
    .current_dir(path)
    .status()?;
  match st.code() {
    Some(0) => Ok(()),
    st => Err(format!("command 'git' is exited with return code {:?}.", st).into()),
  }
}
