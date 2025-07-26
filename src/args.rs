mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[non_exhaustive]
pub struct Args {
    #[command(subcommand)]
    pub op: Ops,

    #[arg(short = 'n', long = "dry-run")]
    pub dry_run: bool,
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
    pub fn run(self) -> Result<()> {
        match self.op {
            Ops::Add(op) => op.run(),
            Ops::Clone(op) => op.run(),
            Ops::Completion(op) => op.run(),
            Ops::Import(op) => op.run(),
            Ops::List(op) => op.run(),
            Ops::New(op) => op.run(),
            Ops::Refresh(op) => op.run(),
        }
    }
}
