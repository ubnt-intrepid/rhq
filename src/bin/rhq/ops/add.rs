use anyhow::Result;
use clap::{App, ArgMatches};
use rhq::Workspace;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct AddCommand {
    paths: Option<Vec<PathBuf>>,
    verbose: bool,
}

impl AddCommand {
    pub fn app<'help>(app: App<'help>) -> App<'help> {
        app.about("Add existed repositories into management")
            .arg_from_usage("[paths]...      'Location of local repositories'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
    }

    pub fn from_matches(m: &ArgMatches) -> AddCommand {
        AddCommand {
            paths: m.values_of("paths").map(|s| s.map(PathBuf::from).collect()),
            verbose: m.is_present("verbose"),
        }
    }

    pub fn run(self) -> Result<()> {
        let paths = self
            .paths
            .unwrap_or_else(|| vec![env::current_dir().expect("env::current_dir()")]);

        let mut workspace = Workspace::new()?.verbose_output(self.verbose);
        for path in paths {
            workspace.add_repository_if_exists(&path)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}
