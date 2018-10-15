use clap::{App, Arg, ArgMatches};
use failure::Fallible;
use std::path::Path;

use query::Query;
use vcs::{Vcs, POSSIBLE_VCS};
use workspace::Workspace;

#[derive(Debug)]
pub struct NewCommand<'a> {
    query: Query,
    root: Option<&'a Path>,
    vcs: Vcs,
    ssh: bool,
}

impl<'a> NewCommand<'a> {
    pub fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Create a new repository and add it into management")
            .arg_from_usage("<query>           'Path of target repository, or URL-like pattern'")
            .arg_from_usage(
                "--root=[root]    'Path to determine the destination of new repository'",
            ).arg(
                Arg::from_usage("--vcs=[vcs] 'Used Version Control System'")
                    .possible_values(POSSIBLE_VCS)
                    .default_value("git"),
            ).arg_from_usage("-s, --ssh        'Use SSH protocol instead of HTTP(s)'")
    }

    pub fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> NewCommand<'a> {
        NewCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            root: m.value_of("root").map(Path::new),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()).unwrap(),
            ssh: m.is_present("ssh"),
        }
    }

    pub fn run(self) -> Fallible<()> {
        let mut workspace = Workspace::new()?;
        if let Some(root) = self.root {
            workspace.set_root_dir(root);
        }

        workspace.create_repository(&self.query, self.vcs, self.ssh)?;

        workspace.save_cache()?;
        Ok(())
    }
}
