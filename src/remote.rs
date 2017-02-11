//! defines functions/types related to remote repository access.

use url::Url;
use regex::Regex;
use errors;

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
/// # Examples
///
/// ```
/// use rhq::remote::build_url;
/// let url = build_url("https://github.com/rust-lang/rust.git");
/// assert!(url.is_ok());
/// ```
///
/// ```
/// use rhq::remote::build_url;
/// let url = build_url("gituser@my-githosting.org:organization1/project1");
/// assert!(url.is_ok());
/// ```
///
/// ```
/// use rhq::remote::build_url;
/// let url = build_url("rust-lang/rust");
/// assert!(url.is_ok());
/// ```
pub fn build_url(s: &str) -> errors::Result<Url> {
  let re_scheme = Regex::new(r"^([^:]+)://").unwrap();
  let re_scplike = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$").unwrap();

  if let Some(cap) = re_scheme.captures(s) {
    info!("has scheme");
    match cap.get(1).unwrap().as_str() {
      "http" | "https" | "ssh" | "git" => Url::parse(s).map_err(Into::into),
      scheme => Err(format!("'{}' is invalid scheme", scheme).into()),
    }

  } else if let Some(cap) = re_scplike.captures(s) {
    info!("SCP-like");
    let username = cap.get(1)
      .and_then(|s| if s.as_str() != "" {
        Some(s.as_str())
      } else {
        None
      })
      .unwrap_or("git@");
    let host = cap.get(2).unwrap().as_str();
    let path = cap.get(3).unwrap().as_str();

    Url::parse(&format!("ssh://{}{}/{}.git", username, host, path)).map_err(Into::into)

  } else {
    info!("query");
    // TODO: take collection of host names from Config
    let url = if let Some(_) = s.split("/").next().and_then(|host| match host {
      "github.com" | "bitbucket.org" | "gitlab.com" => Some(host),
      _ => None,
    }) {
      format!("https://{}.git", s)
    } else {
      format!("https://github.com/{}.git", s)
    };
    Url::parse(&url).map_err(Into::into)
  }
}

#[test]
fn test_https_pattern() {
  let result = build_url("https://github.com/peco/peco.git");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_ssh_pattern() {
  let result = build_url("ssh://gituser@github.com:2222/peco/peco.git");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "ssh");
  assert_eq!(result.username(), "gituser");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.port(), Some(2222));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_scplike_pattern() {
  let result = build_url("git@github.com:peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "ssh");
  assert_eq!(result.username(), "git");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_short_pattern_with_host() {
  let result = build_url("github.com/peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_short_pattern_without_host() {
  let result = build_url("peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}
