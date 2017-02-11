//! defines functions/types related to remote repository access.

use std::borrow::{Cow, Borrow};
use url::Url;
use regex::Regex;
use errors;

pub fn resolve_query(query: &str) -> errors::Result<Url> {
  let re_hasscheme = Regex::new(r"^[^:]+://").unwrap();
  let re_scplike = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$").unwrap();

  let url: Cow<str> = if re_hasscheme.is_match(query) {
    info!("has scheme");
    query.into()
  } else if let Some(cap) = re_scplike.captures(query) {
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
    format!("ssh://{}{}/{}.git", username, host, path).into()
  } else {
    info!("query");
    if let Some(_) = query.split("/").next().and_then(|host| match host {
      "github.com" | "bitbucket.org" | "gitlab.com" => Some(host),
      _ => None,
    }) {
      format!("https://{}.git", query).into()
    } else {
      format!("https://github.com/{}.git", query).into()
    }
  };
  Url::parse(url.borrow()).map_err(Into::into)
}

#[test]
fn test_hostpattern() {
  let result = resolve_query("https://github.com/peco/peco.git");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_ssh_hostpattern() {
  let result = resolve_query("ssh://gituser@github.com:2222/peco/peco.git");
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
  let result = resolve_query("git@github.com:peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "ssh");
  assert_eq!(result.username(), "git");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_shortpattern1() {
  let result = resolve_query("github.com/peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}

#[test]
fn test_shortpattern2() {
  let result = resolve_query("peco/peco");
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.scheme(), "https");
  assert_eq!(result.username(), "");
  assert_eq!(result.password(), None);
  assert_eq!(result.host_str(), Some("github.com"));
  assert_eq!(result.path(), "/peco/peco.git");
}
