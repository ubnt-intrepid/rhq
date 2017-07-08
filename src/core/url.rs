use std::borrow::Cow;
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
    Scp {
        username: String,
        host: String,
        path: String,
    },
    Path(Vec<String>),
}

impl Query {
    /// Returns the host name if available.
    pub fn host(&self) -> Option<&str> {
        match *self {
            Query::Url(ref url) => url.host_str(),
            Query::Scp { ref host, .. } => Some(host.as_str()),
            Query::Path(_) => None,
        }
    }

    pub fn path(&self) -> Cow<str> {
        match *self {
            Query::Url(ref url) => url.path().trim_right_matches(".git").into(),
            Query::Scp { ref path, .. } => path.as_str().into(),
            Query::Path(ref path) => path.join("/").into(),
        }
    }
}

impl FromStr for Query {
    type Err = ::Error;

    fn from_str(s: &str) -> ::std::result::Result<Query, Self::Err> {
        lazy_static! {
      static ref RE_URL: Regex = Regex::new(r"^([^:]+)://").unwrap();
      static ref RE_SCP: Regex = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$").unwrap();
    }

        if let Some(cap) = RE_URL.captures(s) {
            match cap.get(1).unwrap().as_str() {
                "http" | "https" | "ssh" | "git" => Url::parse(s).map(Query::Url).map_err(Into::into),
                scheme => Err(format!("'{}' is invalid scheme", scheme).into()),
            }

        } else if let Some(cap) = RE_SCP.captures(s) {
            let username = cap.get(1)
                              .and_then(|s| if s.as_str() != "" {
                Some(s.as_str())
            } else {
                None
            })
                              .map(|s| s.trim_right_matches("@"))
                              .unwrap_or("git")
                              .to_owned();
            let host = cap.get(2).unwrap().as_str().to_owned();
            let path = cap.get(3)
                          .unwrap()
                          .as_str()
                          .trim_right_matches(".git")
                          .to_owned();
            Ok(Query::Scp {
                username: username,
                host: host,
                path: path,
            })

        } else {
            if s.starts_with("./") || s.starts_with("../") || s.starts_with(".\\") || s.starts_with("..\\") {
                Err("The path must be not a relative path.")?;
            }
            Ok(Query::Path(s.split("/").map(ToOwned::to_owned).collect()))
        }
    }
}


pub fn build_url(query: &Query, is_ssh: bool) -> ::Result<String> {
    match *query {
        Query::Url(ref url) => {
            if url.scheme() == "ssh" {
                let username = url.username();
                let host = url.host_str().ok_or("empty host")?;
                let path = url.path().trim_left_matches("/");
                Ok(format!("{}@{}:{}", username, host, path))
            } else {
                Ok(url.as_str().to_owned())
            }
        }

        Query::Scp {
            ref username,
            ref host,
            ref path,
        } => Ok(format!("{}@{}:{}", username, host, path)),

        Query::Path(ref path) => {
            let url = {
                let host = "github.com";
                let url = if is_ssh {
                    format!("ssh://git@{}/{}.git", host, path.join("/"))
                } else {
                    format!("https://{}/{}.git", host, path.join("/"))
                };
                Url::parse(&url)?
            };
            if url.scheme() == "ssh" {
                let username = url.username();
                let host = url.host_str().ok_or("empty host")?;
                let path = url.path().trim_left_matches("/");
                Ok(format!("{}@{}:{}", username, host, path))
            } else {
                Ok(url.as_str().to_owned())
            }
        }
    }
}


#[cfg(test)]
mod tests_query {
    use super::Query;

    #[test]
    fn https_url() {
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
    fn ssh_url() {
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
    fn scp_pattern() {
        let ss = &["git@github.com:peco/peco.git", "git@github.com:peco/peco"];
        for s in ss {
            if let Ok(Query::Scp {
                          username,
                          host,
                          path,
                      }) = s.parse()
            {
                assert_eq!(username, "git");
                assert_eq!(host, "github.com");
                assert_eq!(path, "peco/peco");
            } else {
                panic!("does not matches");
            }
        }
    }

    #[test]
    fn short_pattern_with_host() {
        let s = "github.com/peco/peco";

        if let Ok(Query::Path(path)) = s.parse() {
            assert_eq!(path, ["github.com", "peco", "peco"]);
        } else {
            panic!("does not matches")
        }
    }

    #[test]
    fn short_pattern_without_host() {
        let s = "peco/peco";

        if let Ok(Query::Path(path)) = s.parse() {
            assert_eq!(path, ["peco", "peco"]);
        } else {
            panic!("does not matches")
        }
    }
}


#[cfg(test)]
mod tests_build_url {
    use super::{Query, build_url};

    #[test]
    fn path_https() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();

        if let Ok(url) = build_url(&query, false) {
            assert_eq!(url, "https://github.com/ubnt-intrepid/rhq.git");
        } else {
            panic!("does not match");
        }
    }

    #[test]
    fn path_scp() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();

        if let Ok(url) = build_url(&query, true) {
            assert_eq!(url, "git@github.com:ubnt-intrepid/rhq.git");
        } else {
            panic!("does not match");
        }
    }
}
