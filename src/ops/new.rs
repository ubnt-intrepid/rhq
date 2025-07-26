use crate::{
    query::Query,
    vcs::{Vcs, POSSIBLE_VCS},
    Workspace,
};
use anyhow::Result;
use clap::{builder::PossibleValuesParser, ArgMatches, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct NewCommand {
    query: Query,
    root: Option<PathBuf>,
    vcs: Vcs,
    ssh: bool,
}

impl NewCommand {
    pub fn command() -> Command {
        Command::new("new")
            .about("Create a new repository and add it into management")
            .args(&[
                clap::arg!(<query>        "Path of target repository, or URL-like pattern"),
                clap::arg!(--root [root]  "Path to determine the destination of new repository"),
                clap::arg!(--vcs [vcs]    "Used Version Control System")
                    .value_parser(PossibleValuesParser::new(POSSIBLE_VCS))
                    .default_value("git"),
                clap::arg!(-s --ssh       "Use SSH protocol instead of HTTP(s)"),
            ])
    }

    pub fn from_matches(m: &ArgMatches) -> NewCommand {
        NewCommand {
            query: m
                .get_one::<String>("query")
                .and_then(|s| s.parse().ok())
                .unwrap(),
            root: m.get_one::<String>("root").map(PathBuf::from),
            vcs: m
                .get_one::<String>("vcs")
                .and_then(|s| s.parse().ok())
                .unwrap(),
            ssh: m.contains_id("ssh"),
        }
    }

    pub fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;
        if let Some(root) = self.root {
            workspace.set_root_dir(root);
        }

        workspace.create_repository(&self.query, self.vcs, self.ssh)?;

        workspace.save_cache()?;
        Ok(())
    }
}
