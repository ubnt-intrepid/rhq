use clap::{App, ArgMatches};
use failure::Fallible;
use std::env;
use std::path::PathBuf;

use workspace::Workspace;

#[derive(Debug)]
pub struct AddCommand {
    paths: Option<Vec<PathBuf>>,
    verbose: bool,
}

impl AddCommand {
    pub fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
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

    pub fn run(self) -> Fallible<()> {
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
