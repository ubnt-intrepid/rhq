use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fs;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use shellexpand;
use shlex;


pub fn make_path_buf<S: AsRef<str>>(s: S) -> ::Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}

#[cfg(windows)]
pub fn canonicalize_pretty<P: AsRef<Path>>(path: P) -> ::Result<PathBuf> {
    path.as_ref()
        .canonicalize()
        .map_err(Into::into)
        .map(|path| {
            path.to_string_lossy()
                .trim_left_matches(r"\\?\")
                .replace(r"\", "/")
        })
        .map(|s| PathBuf::from(s))
}


#[cfg(not(windows))]
pub fn canonicalize_pretty<P: AsRef<Path>>(path: P) -> ::Result<PathBuf> {
    path.as_ref().canonicalize().map_err(Into::into)
}


pub fn join_str<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> + Display,
{
    use std::borrow::Borrow;
    args.into_iter().fold(String::new(), |mut acc, s| {
        if !acc.is_empty() {
            acc.push_str(" ");
        }
        let s = s.as_ref().to_string_lossy();
        let s = shlex::quote(s.borrow());
        acc.push_str(s.borrow());
        acc
    })
}


pub trait StrSkip {
    fn skip<'a>(&'a self, n: usize) -> &'a str;
}

impl StrSkip for str {
    fn skip<'a>(&'a self, n: usize) -> &'a str {
        let mut s = self.chars();
        for _ in 0..n {
            s.next();
        }
        s.as_str()
    }
}

#[test]
fn test_skipped_1() {
    assert_eq!("hoge".skip(1), "oge");
    assert_eq!("あいueo".skip(1), "いueo");
}


pub fn write_content<P, F>(path: P, write_fn: F) -> ::Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut fs::File) -> ::Result<()>,
{
    fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write_fn(&mut file)
}


pub mod process {
    use std::process::{Command, Stdio};

    pub fn inherit(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
    }

    pub fn piped(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command
    }
}
