use crate::{query::Query, vcs::Vcs, Remote, Workspace};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(
    name = "clone",
    aliases = ["cl"],
    about = "Clone remote repositories, and then add it under management"
)]
pub struct CloneCommand {
    #[arg(help = "An URL or a string to determine the URL of remote repository")]
    query: Query,

    #[arg(help = "Destination directory of cloned repository")]
    dest: Option<PathBuf>,

    #[arg(
        long = "root",
        help = "Path to determine the destination directory of cloned repository"
    )]
    root: Option<PathBuf>,

    #[arg(
        short = 's',
        long = "ssh",
        help = "Use SSH protocol instead of HTTP(s)"
    )]
    ssh: bool,

    #[arg(long = "vcs", help = "Used Version Control System", default_value_t = Vcs::Git)]
    vcs: Vcs,
}

impl CloneCommand {
    pub fn run(self, workspace: &mut Workspace) -> Result<()> {
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
