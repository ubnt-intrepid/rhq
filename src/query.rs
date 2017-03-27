use std::str::FromStr;
use url::{self, Url};
use regex::Regex;

/// Represents query from user.
///
/// Available patterns are:
///
/// * `<scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git`
///   - Available schemes are: `http[s]`, `ssh` and `git`.
/// * `<username>@<host>:<path-to-repo>`
///   - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
/// * `<path-to-repo>`
pub enum Query {
  Url(url::Url),
  Path(Vec<String>),
}

impl Query {
  pub fn to_local_path(&self) -> ::Result<String> {
    let url = self.to_url_impl(false)?;
    let mut path = url.host_str()
      .map(ToOwned::to_owned)
      .ok_or("url.host() is empty")?;
    path += url.path().trim_right_matches(".git");
    Ok(path)
  }

  pub fn to_url(&self, is_ssh: bool) -> ::Result<String> {
    let url = self.to_url_impl(is_ssh)?;

    if url.scheme() == "ssh" {
      let username = url.username();
      let host = url.host_str().ok_or("empty host")?;
      let path = url.path().trim_left_matches("/");
      Ok(format!("{}@{}:{}", username, host, path))
    } else {
      Ok(url.as_str().to_owned())
    }
  }

  fn to_url_impl(&self, is_ssh: bool) -> ::Result<Url> {
    match *self {
      Query::Url(ref url) => Ok(url.clone()),
      Query::Path(ref path) => resolve_url(path, is_ssh, None),
    }
  }
}

impl FromStr for Query {
  type Err = ::Error;

  fn from_str(s: &str) -> ::std::result::Result<Query, Self::Err> {
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
      let path = cap.get(3)
        .unwrap()
        .as_str()
        .trim_right_matches(".git");
      let url = Url::parse(&format!("ssh://{}{}/{}.git", username, host, path))?;
      Ok(Query::Url(url))

    } else {
      Ok(Query::Path(s.split("/").map(ToOwned::to_owned).collect()))
    }
  }
}

fn resolve_url(path: &[String], is_ssh: bool, host: Option<&str>) -> ::Result<Url> {
  let host = host.unwrap_or("github.com");
  let url = if is_ssh {
    format!("ssh://git@{}/{}.git", host, path.join("/"))
  } else {
    format!("https://{}/{}.git", host, path.join("/"))
  };
  let url = Url::parse(&url)?;

  Ok(url)
}


#[test]
fn test_to_local_path() {
  let s = "ubnt-intrepid/rhq";
  let query: Query = s.parse().unwrap();

  if let Ok(local_path) = query.to_local_path() {
    assert_eq!(local_path, "github.com/ubnt-intrepid/rhq");
  } else {
    panic!();
  }
}

#[test]
fn test_url() {
  let s = "ubnt-intrepid/rhq";
  let query: Query = s.parse().unwrap();

  if let Ok(url) = query.to_url(false) {
    assert_eq!(url, "https://github.com/ubnt-intrepid/rhq.git");
  } else {
    panic!();
  }

  if let Ok(url) = query.to_url(true) {
    assert_eq!(url, "git@github.com:ubnt-intrepid/rhq.git");
  } else {
    panic!();
  }
}


#[test]
fn test_https_pattern() {
  let s = "https://github.com/peco/peco.git";

  if let Ok(Query::Url(url)) = s.parse() {
    assert_eq!(url.scheme(), "https");
    assert_eq!(url.username(), "");
    assert_eq!(url.password(), None);
    assert_eq!(url.host_str(), Some("github.com"));
    assert_eq!(url.path(), "/peco/peco.git");
  } else {
    panic!("does not matches");
  }
}

#[test]
fn test_ssh_pattern() {
  let s = "ssh://gituser@github.com:2222/peco/peco.git";

  if let Ok(Query::Url(url)) = s.parse() {
    assert_eq!(url.scheme(), "ssh");
    assert_eq!(url.username(), "gituser");
    assert_eq!(url.password(), None);
    assert_eq!(url.host_str(), Some("github.com"));
    assert_eq!(url.port(), Some(2222));
    assert_eq!(url.path(), "/peco/peco.git");
  } else {
    panic!("does not matches");
  }
}

#[test]
fn test_scplike_pattern() {
  let ss = &["git@github.com:peco/peco.git", "git@github.com:peco/peco"];
  for s in ss {
    if let Ok(Query::Url(url)) = s.parse() {
      assert_eq!(url.scheme(), "ssh");
      assert_eq!(url.username(), "git");
      assert_eq!(url.password(), None);
      assert_eq!(url.host_str(), Some("github.com"));
      assert_eq!(url.path(), "/peco/peco.git");
    } else {
      panic!("does not matches");
    }
  }
}

#[test]
fn test_short_pattern_with_host() {
  let s = "github.com/peco/peco";

  if let Ok(Query::Path(path)) = s.parse() {
    assert_eq!(path, ["github.com", "peco", "peco"]);
  } else {
    panic!("does not matches")
  }
}

#[test]
fn test_short_pattern_without_host() {
  let s = "peco/peco";

  if let Ok(Query::Path(path)) = s.parse() {
    assert_eq!(path, ["peco", "peco"]);
  } else {
    panic!("does not matches")
  }
}
