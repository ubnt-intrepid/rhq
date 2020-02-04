use failure::{format_err, Fallible};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct ScpPath {
    username: String,
    host: String,
    path: String,
}

impl ScpPath {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl FromStr for ScpPath {
    type Err = failure::Error;

    fn from_str(s: &str) -> Fallible<ScpPath> {
        lazy_static! {
            static ref RE_SCP: Regex = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$")
                .expect("should be a valid regex pattern");
        }
        let cap = RE_SCP
            .captures(s)
            .ok_or_else(|| format_err!("does not match"))?;

        let username = cap
            .get(1)
            .and_then(|s| {
                if s.as_str() != "" {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .map(|s| s.trim_end_matches('@'))
            .unwrap_or("git")
            .to_owned();
        let host = cap.get(2).unwrap().as_str().to_owned();
        let path = cap
            .get(3)
            .unwrap()
            .as_str()
            .trim_end_matches(".git")
            .to_owned();
        Ok(ScpPath {
            username,
            host,
            path,
        })
    }
}

impl fmt::Display for ScpPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}:{}.git", self.username, self.host, self.path)
    }
}
