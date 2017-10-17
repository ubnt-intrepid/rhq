use query::Query;
use url::Url;


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
    pub fn from_query(query: &Query, is_ssh: bool) -> ::Result<Self> {
        let url = match *query {
            Query::Url(ref url) => normalize_scp(url)?,
            Query::Scp {
                ref username,
                ref host,
                ref path,
            } => format!("{}@{}:{}", username, host, path),
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
                normalize_scp(&url)?
            }
        };
        Ok(Self { url })
    }
}


fn normalize_scp(url: &Url) -> ::Result<String> {
    if url.scheme() == "ssh" {
        let username = url.username();
        let host = url.host_str().ok_or("empty host")?;
        let path = url.path().trim_left_matches("/");
        Ok(format!("{}@{}:{}", username, host, path))
    } else {
        Ok(url.as_str().to_owned())
    }
}


#[cfg(test)]
mod tests_build_url {
    use super::Query;

    #[test]
    fn path_https() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();

        if let Ok(url) = query.to_url(false) {
            assert_eq!(url, "https://github.com/ubnt-intrepid/rhq.git");
        } else {
            panic!("does not match");
        }
    }

    #[test]
    fn path_scp() {
        let s = "ubnt-intrepid/rhq";
        let query: Query = s.parse().unwrap();

        if let Ok(url) = query.to_url(true) {
            assert_eq!(url, "git@github.com:ubnt-intrepid/rhq.git");
        } else {
            panic!("does not match");
        }
    }
}
