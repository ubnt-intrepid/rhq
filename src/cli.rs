mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

use crate::Workspace;
use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[non_exhaustive]
pub struct Args {
    #[command(subcommand)]
    pub op: Ops,

    #[arg(short = 'v', long = "verbose", help = "Use verbose output")]
    pub verbose: bool,
}

#[derive(Debug, clap::Subcommand)]
#[non_exhaustive]
pub enum Ops {
    Add(add::AddCommand),
    Clone(clone::CloneCommand),
    Completion(completion::CompletionCommand),
    Import(import::ImportCommand),
    List(list::ListCommand),
    New(new::NewCommand),
    Refresh(refresh::RefreshCommand),
}

impl Args {
    pub fn run(self, workspace: &mut Workspace) -> Result<()> {
        match self.op {
            Ops::Add(op) => op.run(workspace),
            Ops::Clone(op) => op.run(workspace),
            Ops::Completion(op) => op.run(workspace),
            Ops::Import(op) => op.run(workspace),
            Ops::List(op) => op.run(workspace),
            Ops::New(op) => op.run(workspace),
            Ops::Refresh(op) => op.run(workspace),
        }
    }
}
