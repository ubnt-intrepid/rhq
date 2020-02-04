use failure::Fallible;
use std::ffi::OsStr;
use std::path::Path;

use util::process;

pub fn init<P>(path: P) -> Fallible<()>
where
    P: AsRef<Path>,
{
    process::inherit("hg")
        .arg("init")
        .arg(path.as_ref().as_os_str())
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(format_err!(
                "command 'hg' is exited with return code {:?}.",
                st
            )),
        })
}

pub fn clone<P, U, I, S>(url: U, path: P, args: I) -> Fallible<()>
where
    P: AsRef<Path>,
    U: AsRef<str>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let path = format!("{}", path.as_ref().display());
    process::inherit("hg")
        .arg("clone")
        .args(args)
        .args(&[url.as_ref(), &path])
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(format_err!(
                "command 'hg' is exited with return code {:?}.",
                st
            )),
        })
}

pub fn get_remote_url<P: AsRef<Path>>(repo_path: P) -> Fallible<Option<String>> {
    // 1. get current branch
    let output = process::piped("hg")
        .arg("branch")
        .current_dir(&repo_path)
        .output()?;
    if !output.status.success() {
        return Err(format_err!("hg: failed to get branch name"));
    }
    let branch = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_owned();

    // 2. get URL
    let output = process::piped("hg")
        .arg("paths")
        .arg(branch)
        .current_dir(repo_path)
        .output()?;
    if !output.status.success() {
        return Ok(None);
    }
    let url = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if url == "" {
        Ok(None)
    } else {
        Ok(Some(url))
    }
}
