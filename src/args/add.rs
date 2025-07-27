use crate::Workspace;
use anyhow::Result;
use std::{env, path::PathBuf};

#[derive(Debug, clap::Parser)]
#[command(name = "add", about = "Add existed repositories into management")]
pub struct AddCommand {
    #[arg(help = "Location of local repositories")]
    paths: Option<Vec<PathBuf>>,
}

impl AddCommand {
    pub fn run(self, workspace: &mut Workspace) -> Result<()> {
        let paths = self
            .paths
            .unwrap_or_else(|| vec![env::current_dir().expect("env::current_dir()")]);

        for path in paths {
            workspace.add_repository_if_exists(&path)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}
