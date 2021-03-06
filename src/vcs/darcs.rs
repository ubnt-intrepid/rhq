use crate::util::process;
use anyhow::{anyhow, Result};
use std::{ffi::OsStr, path::Path};

pub fn initialize<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    process::inherit("darcs")
        .arg("initialize")
        .arg(path.as_ref().as_os_str())
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'darcs' is exited with return code {:?}.",
                st
            )),
        })
}

pub fn clone<P, U, I, S>(url: U, path: P, args: I) -> Result<()>
where
    P: AsRef<Path>,
    U: AsRef<str>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let path = format!("{}", path.as_ref().display());
    process::inherit("darcs")
        .arg("clone")
        .args(args)
        .args(&[url.as_ref(), &path])
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'darcs' is exited with return code {:?}.",
                st
            )),
        })
}
