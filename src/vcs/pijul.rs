use failure::Fallible;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use util::process;

pub fn init<P>(path: P) -> Fallible<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(&path)?;
    process::inherit("pijul")
        .arg("init")
        .current_dir(path)
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(format_err!(
                "command 'pijul' is exited with return code {:?}.",
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
    process::inherit("pijul")
        .arg("clone")
        .args(args)
        .args(&[url.as_ref(), &path])
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(format_err!(
                "command 'pijul' is exited with return code {:?}.",
                st
            )),
        })
}
