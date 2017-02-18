//! defines functions/types related to remote repository access.

use std::str::FromStr;
use url::{self, Url};
use regex::Regex;
use errors;

pub enum Query {
  Url(url::Url),
  Scp {
    username: String,
    host: String,
    path: String,
  },
  Path(String),
  PathWithHost(String),
}

impl Query {
  pub fn to_url(&self) -> Result<url::Url, url::ParseError> {
    match *self {
      Query::Url(ref url) => Ok(url.clone()),
      Query::Scp { ref username, ref host, ref path } => {
        Url::parse(&format!("ssh://{}{}/{}.git", username, host, path))
      }
      Query::PathWithHost(ref path) => Url::parse(&format!("https://{}.git", path)),
      Query::Path(ref path) => Url::parse(&format!("https://github.com/{}.git", path)),
    }
  }
}

impl FromStr for Query {
  type Err = errors::Error;

  /// build a new instance of `url::Url` from given str.
  ///
  /// available patterns are:
  ///
  /// * `<scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git`
  ///   - Available schemes are: `http[s]`, `ssh` and `git`.
  /// * `<username>@<host>:<path-to-repo>`
  ///   - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
  /// * `[<host>/]<path-to-repo>`
  ///   - When `<host>` is omitted, it is replaced by `github.com`.
  ///
  fn from_str(s: &str) -> Result<Query, Self::Err> {
    let re_scheme = Regex::new(r"^([^:]+)://").unwrap();
    let re_scplike = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$").unwrap();

    if let Some(cap) = re_scheme.captures(s) {
      match cap.get(1).unwrap().as_str() {
        "http" | "https" | "ssh" | "git" => Ok(Query::Url(Url::parse(s)?)),
        scheme => Err(format!("'{}' is invalid scheme", scheme).into()),
      }

    } else if let Some(cap) = re_scplike.captures(s) {
      let username = cap.get(1)
        .and_then(|s| if s.as_str() != "" {
          Some(s.as_str())
        } else {
          None
        })
        .unwrap_or("git@");
      let host = cap.get(2).unwrap().as_str();
      let path = cap.get(3).unwrap().as_str();
      Ok(Query::Scp {
        username: username.to_owned(),
        host: host.to_owned(),
        path: path.to_owned(),
      })

    } else {
      if let Some(_) = s.split("/").next().and_then(|host| match host {
        "github.com" | "bitbucket.org" | "gitlab.com" => Some(host),
        _ => None,
      }) {
        Ok(Query::PathWithHost(s.to_owned()))
      } else {
        Ok(Query::Path(s.to_owned()))
      }
    }
  }
}

#[test]
fn test_https_pattern() {
  let result = Query::from_str("https://github.com/peco/peco.git");
  assert!(result.is_ok());

  let result = result.unwrap().to_url().unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_ssh_pattern() {
  let result = Query::from_str("ssh://gituser@github.com:2222/peco/peco.git");
  assert!(result.is_ok());

  let result = result.unwrap().to_url().unwrap();
  assert_eq!(result.scheme(), "ssh");
  assert_eq!(result.username(), "gituser");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.port(), Some(2222));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_scplike_pattern() {
  let result = Query::from_str("git@github.com:peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap().to_url().unwrap();
  assert_eq!(result.scheme(), "ssh");
  assert_eq!(result.username(), "git");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_short_pattern_with_host() {
  let result = Query::from_str("github.com/peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap().to_url().unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_short_pattern_without_host() {
  let result = Query::from_str("peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap().to_url().unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}
