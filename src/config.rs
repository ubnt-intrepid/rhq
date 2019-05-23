use failure::{Fallible, ResultExt};
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum ConfigKind {
    /// The configuration was read from the specified file.
    File(PathBuf),

    /// The configuration is default values.
    Default,
}

impl Default for ConfigKind {
    fn default() -> Self {
        ConfigKind::Default
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    pub kind: ConfigKind,

    #[serde(default)]
    pub clone: CloneConfig,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(content) = toml::ser::to_string_pretty(self) {
            f.write_str(&content)?;
        }

        Ok(())
    }
}

impl Config {
    pub fn from_env() -> Fallible<Self> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_path = home_dir.join(".rhqconfig");
            if config_path.is_file() {
                return Self::open(config_path);
            }
        }

        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("rhq/config");
            if config_path.is_file() {
                return Self::open(config_path);
            }
        }

        Ok(Self::default())
    }

    pub fn open(path: impl AsRef<Path>) -> Fallible<Self> {
        let location = fs::canonicalize(path)?;

        let content = fs::read_to_string(&location) //
            .with_context(|_err| {
                format!(
                    "cannot read the content of `{}'.", //
                    location.display()
                )
            })?;

        let config: Self = toml::de::from_str(&content) //
            .with_context(|err| {
                format!(
                    "the configuration file `{}' is not a valid TOML file:\n{}",
                    location.display(),
                    err
                )
            })?;

        Ok(Self {
            kind: ConfigKind::File(location),
            ..config
        })
    }

    /// Returns whether the configuration uses the default value or not.
    pub fn is_default(&self) -> bool {
        match self.kind {
            ConfigKind::Default => true,
            _ => false,
        }
    }

    pub fn fill_default_fields(&mut self) {
        self.clone.fill_default_fields();
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CloneConfig {
    pub dest: Option<Destination>,

    #[serde(default)]
    pub args: Vec<String>,
}

impl CloneConfig {
    pub fn fill_default_fields(&mut self) {
        self.dest.get_or_insert_with(Default::default);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Destination(String);

impl Default for Destination {
    fn default() -> Self {
        Destination("~/repos/{host}/{user}/{project}".into())
    }
}

impl FromStr for Destination {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_bracket_position = None;
        let mut host = false;
        let mut user = false;
        let mut project = false;

        for (i, c) in s.char_indices() {
            match c {
                '{' => {
                    if let Some(_) = left_bracket_position.replace(i) {
                        failure::bail!("nested bracket");
                    }
                }
                '}' => {
                    let pos = left_bracket_position
                        .take()
                        .ok_or_else(|| failure::format_err!("missing left bracket"))?;
                    match &s[pos + 1..i] {
                        "host" => host = true,
                        "user" => user = true,
                        "project" => project = true,
                        pattern => failure::bail!("unknown pattern: {{{}}}", pattern),
                    }
                }
                _ => (),
            }
        }

        if !host {
            failure::bail!("missing pattern: {host}");
        }
        if !user {
            failure::bail!("missing pattern: {user}");
        }
        if !project {
            failure::bail!("missing pattern: {project}");
        }

        Ok(Destination(s.into()))
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Destination {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl Destination {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn render(&self, host: &str, user: &str, project: &str) -> String {
        self.0
            .replace("{host}", host)
            .replace("{user}", user)
            .replace("{project}", project)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_parse() {
        assert!(Destination::from_str("~/repos/{host}/{user}/{project}/repo").is_ok());
        assert!(Destination::from_str("~/repos/{host}/{user}/{project}").is_ok());
        assert!(Destination::from_str("~/repos/{host}-{user}-{project}").is_ok());
        assert!(Destination::from_str("~/repos/{host}.{user}.{project}").is_ok());

        assert!(Destination::from_str("~/repos/{user}/{project}").is_err());
        assert!(Destination::from_str("~/repos/{user{host}}/{project}").is_err());
        assert!(Destination::from_str("{user}/{project}/{hostfamily}").is_err());
    }

    #[test]
    fn test_destination_render() {
        let dest = Destination("~/repos/{host}/{user}/{project}".into());
        assert_eq!(
            dest.render("github.com", "foo", "bar"),
            "~/repos/github.com/foo/bar"
        );
    }
}
