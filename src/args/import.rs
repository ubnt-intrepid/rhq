use crate::Workspace;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(
    name = "import",
    aliases = ["imp"],
    about = "Import existed repositories into management"
)]
pub struct ImportCommand {
    #[arg(help = "Root directories contains for scanning")]
    roots: Option<Vec<PathBuf>>,

    #[arg(
        long = "depth",
        help = "Maximal depth of entries for each base directory"
    )]
    depth: Option<usize>,

    #[arg(short = 'v', long = "verbose", help = "Use verbose output")]
    verbose: bool,
}

impl ImportCommand {
    pub fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);

        let roots = self
            .roots
            .unwrap_or_else(|| workspace.config().include_dirs.clone());
        for root in roots {
            workspace.import_repositories(root, self.depth)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}
