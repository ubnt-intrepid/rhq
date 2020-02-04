use crate::scp::ScpPath;
use anyhow::{anyhow, Result};
use std::str::FromStr;
use url::Url;

/// Represents query from user.
///
/// Available patterns are:
///
/// * `<scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git`
///   - Available schemes are: `http[s]`, `ssh` and `git`.
/// * `<username>@<host>:<path-to-repo>`
///   - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
/// * `<path-to-repo>`
#[derive(Debug)]
pub enum Query {
    Url(Url),
    Scp(ScpPath),
    Path(String),
}

impl Query {
    /// Returns the host name if available.
    pub fn host(&self) -> Option<&str> {
        match *self {
            Query::Url(ref url) => url.host_str(),
            Query::Scp(ref scp) => Some(scp.host()),
            Query::Path(_) => None,
        }
    }

    pub fn path(&self) -> &str {
        match *self {
            Query::Url(ref url) => url.path().trim_start_matches('/').trim_end_matches(".git"),
            Query::Scp(ref scp) => scp.path(),
            Query::Path(ref path) => path,
        }
    }
}

impl FromStr for Query {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Query> {
        if let Ok(url) = Url::parse(s) {
            match url.scheme() {
                "http" | "https" | "ssh" | "git" => {}
                scheme => return Err(anyhow!("'{}' is invalid scheme", scheme)),
            }
            Ok(Query::Url(url))
        } else if let Ok(scp) = ScpPath::from_str(s) {
            Ok(Query::Scp(scp))
        } else {
            if s.starts_with("./")
                || s.starts_with("../")
                || s.starts_with(".\\")
                || s.starts_with("..\\")
            {
                return Err(anyhow!("The path must be not a relative path."));
            }
            Ok(Query::Path(s.to_owned()))
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
            if let Ok(Query::Scp(scp)) = s.parse() {
                assert_eq!(scp.username(), "git");
                assert_eq!(scp.host(), "github.com");
                assert_eq!(scp.path(), "peco/peco");
            } else {
                panic!("does not matches");
            }
        }
    }

    #[test]
    fn short_pattern_with_host() {
        let s = "github.com/peco/peco";

        if let Ok(Query::Path(path)) = s.parse() {
            assert_eq!(path, "github.com/peco/peco");
        } else {
            panic!("does not matches")
        }
    }

    #[test]
    fn short_pattern_without_host() {
        let s = "peco/peco";

        if let Ok(Query::Path(path)) = s.parse() {
            assert_eq!(path, "peco/peco");
        } else {
            panic!("does not matches")
        }
    }
}

#[cfg(test)]
mod test_methods {
    use super::Query;

    #[test]
    fn case_url() {
        let url = "https://github.com/ubnt-intrepid/dot.git";
        let query: Query = url.parse().unwrap();

        assert_eq!(query.host(), Some("github.com"));
        assert_eq!(query.path(), "ubnt-intrepid/dot");
    }

    #[test]
    fn case_scp() {
        let s = "git@github.com:ubnt-intrepid/dot.git";
        let query: Query = s.parse().unwrap();

        assert_eq!(query.host(), Some("github.com"));
        assert_eq!(query.path(), "ubnt-intrepid/dot");
    }

    #[test]
    fn case_relative() {
        let s = "ubnt-intrepid/dot";
        let query: Query = s.parse().unwrap();

        assert_eq!(query.host(), None);
        assert_eq!(query.path(), "ubnt-intrepid/dot");
    }
}
