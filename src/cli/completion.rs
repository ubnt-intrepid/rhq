use crate::{cli::Args, Workspace};
use anyhow::Result;
use clap::CommandFactory as _;
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(
    name = "completion",
    aliases = ["cmpl"],
    about = "Generate completion scripts for your shell"
)]
pub struct CompletionCommand {
    #[arg(help = "Target shell")]
    shell: Shell,

    #[arg(help = "Destination path to generated script")]
    out_file: Option<PathBuf>,
}

impl CompletionCommand {
    pub fn run(self, _: &mut Workspace) -> Result<()> {
        if let Some(path) = self.out_file {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            clap_complete::generate(
                self.shell,
                &mut Args::command(),
                env!("CARGO_PKG_NAME"),
                &mut file,
            );
        } else {
            clap_complete::generate(
                self.shell,
                &mut Args::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
        Ok(())
    }
}
