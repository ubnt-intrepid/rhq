use crate::{query::Query, vcs::Vcs, vcs::POSSIBLE_VCS, Remote, Workspace};
use anyhow::Result;
use clap::{builder::PossibleValuesParser, ArgMatches, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CloneCommand {
    query: Query,
    dest: Option<PathBuf>,
    root: Option<PathBuf>,
    ssh: bool,
    vcs: Vcs,
}

impl CloneCommand {
    pub fn command() -> Command {
        Command::new("clone")
        .about("Clone remote repositories, and then add it under management")
        .args(&[
            clap::arg!(<query>       "An URL or a string to determine the URL of remote repository"),
            clap::arg!([dest]        "Destination directory of cloned repository"),
            clap::arg!(--root [root] "Path to determine the destination directory of cloned repository"),
            clap::arg!(-s --ssh      "Use SSH protocol instead of HTTP(s)"),
            clap::arg!(--vcs [vcs]   "Used Version Control System")
                .value_parser(PossibleValuesParser::new(POSSIBLE_VCS))
                .default_value("git"),
        ])
        .aliases(&["cl"])
    }

    pub fn from_matches(m: &ArgMatches) -> CloneCommand {
        CloneCommand {
            query: m
                .get_one::<String>("query")
                .and_then(|s| s.parse().ok())
                .unwrap(),
            dest: m.get_one::<String>("dest").map(PathBuf::from),
            root: m.get_one::<String>("root").map(PathBuf::from),
            ssh: m.contains_id("ssh"),
            vcs: m
                .get_one::<String>("vcs")
                .and_then(|s| s.parse().ok())
                .unwrap(),
        }
    }

    pub fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;
        if let Some(root) = self.root {
            workspace.set_root_dir(root);
        }

        let remote = Remote::from_query(&self.query, self.ssh, workspace.default_host())?;
        let dest = match self.dest {
            Some(dest) => dest,
            None => workspace.resolve_query(&self.query)?,
        };
        workspace.clone_repository(remote, &dest, self.vcs)?;

        workspace.save_cache()?;
        Ok(())
    }
}
