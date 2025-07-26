use crate::Workspace;
use anyhow::Result;
use std::{env, path::PathBuf};

#[derive(Debug, clap::Parser)]
#[command(name = "add", about = "Add existed repositories into management")]
pub struct AddCommand {
    #[arg(help = "Location of local repositories")]
    paths: Option<Vec<PathBuf>>,

    #[arg(short = 'v', long = "verbose", help = "Use verbose output")]
    verbose: bool,
}

impl AddCommand {
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
