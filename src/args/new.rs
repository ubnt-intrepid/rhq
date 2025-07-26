use crate::{query::Query, vcs::Vcs, Workspace};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(
    name = "new",
    about = "Create a new repository and add it into management"
)]
pub struct NewCommand {
    #[arg(help = "Path of target repository, or URL-like pattern")]
    query: Query,

    #[arg(
        long = "root",
        help = "Path to determine the destination of new repository"
    )]
    root: Option<PathBuf>,

    #[arg(long = "vcs", help = "Used Version Control System", default_value_t = Vcs::Git)]
    vcs: Vcs,

    #[arg(
        short = 's',
        long = "ssh",
        help = "Use SSH protocol instead of HTTP(s)"
    )]
    ssh: bool,
}

impl NewCommand {
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
