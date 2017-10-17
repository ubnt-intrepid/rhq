use regex::Regex;


pub struct ScpPath {
    pub username: String,
    pub host: String,
    pub path: String,
}

lazy_static! {
    static ref RE_SCP: Regex = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$").unwrap();
}

impl ::std::str::FromStr for ScpPath {
    type Err = ::Error;

    fn from_str(s: &str) -> ::Result<ScpPath> {
        let cap = RE_SCP.captures(s).ok_or_else(|| "does not match")?;
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
        Ok(ScpPath {
            username,
            host,
            path,
        })
    }
}
