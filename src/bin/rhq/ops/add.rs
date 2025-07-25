use anyhow::Result;
use clap::{arg, ArgMatches, Command};
use rhq::Workspace;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct AddCommand {
    paths: Option<Vec<PathBuf>>,
    verbose: bool,
}

impl AddCommand {
    pub fn app(app: Command) -> Command {
        app.about("Add existed repositories into management")
            .args(&[
                arg!([paths] ... "Location of local repositories"),
                arg!(-v --verbose "Use verbose output"),
            ])
    }

    pub fn from_matches(m: &ArgMatches) -> AddCommand {
        AddCommand {
            paths: m
                .get_many::<String>("paths")
                .map(|values| values.into_iter().map(PathBuf::from).collect()),
            verbose: m.contains_id("verbose"),
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
