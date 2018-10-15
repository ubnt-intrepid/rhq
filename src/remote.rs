use failure::Fallible;
use url::Url;

use query::Query;
use scp::ScpPath;

/// Information of remote repository
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Remote {
    url: String,
}

impl Remote {
    pub fn new<S: Into<String>>(url: S) -> Remote {
        // TODO: verify URL
        Remote { url: url.into() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Remote {
    pub fn from_url(url: &Url) -> Fallible<Self> {
        let url = if url.scheme() == "ssh" {
            let username = url.username();
            let host = url.host_str().ok_or_else(|| format_err!("empty host"))?;
            let path = url.path().trim_left_matches("/");
            format!("{}@{}:{}", username, host, path)
        } else {
            url.as_str().to_owned()
        };
        Ok(Self { url })
    }

    pub fn from_scp(scp: &ScpPath) -> Self {
        Self {
            url: scp.to_string(),
        }
    }

    pub fn from_path(path: &str, is_ssh: bool, host: &str) -> Fallible<Self> {
        if is_ssh {
            let scp: ScpPath = format!("git@{}:{}", host, path).parse()?;
            Ok(Self::from_scp(&scp))
        } else {
            let url = Url::parse(&format!("https://{}/{}.git", host, path))?;
            Self::from_url(&url)
        }
    }

    pub fn from_query(query: &Query, is_ssh: bool, host: &str) -> Fallible<Self> {
        match *query {
            Query::Url(ref url) => Self::from_url(url),
            Query::Scp(ref path) => Ok(Self::from_scp(path)),
            Query::Path(ref path) => Self::from_path(path, is_ssh, host),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_https() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();
        let remote = Remote::from_query(&query, false, "github.com").unwrap();
        assert_eq!(remote.url, "https://github.com/ubnt-intrepid/rhq.git");
    }

    #[test]
    fn path_scp() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();
        let remote = Remote::from_query(&query, true, "github.com").unwrap();
        assert_eq!(remote.url, "git@github.com:ubnt-intrepid/rhq.git");
    }
}
