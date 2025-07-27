use crate::Workspace;
use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[command(
    name = "refresh",
    about = "Scan repository list and drop if it is not existed or matches exclude pattern."
)]
pub struct RefreshCommand {
    #[arg(short = 's', long = "sort", help = "Sort by path string")]
    sort: bool,
}

impl RefreshCommand {
    pub fn run(self, workspace: &mut Workspace) -> Result<()> {
        workspace.drop_invalid_repositories();
        if self.sort {
            workspace.sort_repositories();
        }
        workspace.save_cache()?;
        Ok(())
    }
}
