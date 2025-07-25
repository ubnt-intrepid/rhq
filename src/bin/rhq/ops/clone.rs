use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use rhq::{query::Query, vcs::Vcs, vcs::POSSIBLE_VCS, Remote, Workspace};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CloneCommand<'a> {
    query: Query,
    dest: Option<PathBuf>,
    root: Option<&'a Path>,
    ssh: bool,
    vcs: Vcs,
}

impl<'a> CloneCommand<'a> {
    pub fn app<'help>(app: App<'help>) -> App<'help> {
        app.about("Clone remote repositories, and then add it under management")
            .arg_from_usage("<query>          'an URL or a string to determine the URL of remote repository'")
            .arg_from_usage("[dest]           'Destination directory of cloned repository'")
            .arg_from_usage("--root=[root]    'Path to determine the destination directory of cloned repository'")
            .arg_from_usage("-s, --ssh        'Use SSH protocol instead of HTTP(s)'")
            .arg(
                Arg::from_usage("--vcs=[vcs] 'Used Version Control System'")
                    .possible_values(POSSIBLE_VCS)
                    .default_value("git"),
            )
    }

    pub fn from_matches<'b: 'a>(m: &'b ArgMatches) -> CloneCommand<'b> {
        CloneCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            dest: m.value_of("dest").map(PathBuf::from),
            root: m.value_of("root").map(Path::new),
            ssh: m.is_present("ssh"),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()).unwrap(),
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
