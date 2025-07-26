mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

use anyhow::Result;
use clap::{ArgMatches, Command};

use crate::ops::{
    add::AddCommand, clone::CloneCommand, completion::CompletionCommand, import::ImportCommand,
    list::ListCommand, new::NewCommand, refresh::RefreshCommand,
};

#[derive(Debug)]
#[non_exhaustive]
pub enum Ops {
    Add(AddCommand),
    Clone(CloneCommand),
    Completion(CompletionCommand),
    Import(ImportCommand),
    List(ListCommand),
    New(NewCommand),
    Refresh(RefreshCommand),
}

impl Ops {
    pub fn command() -> Command {
        clap::command!() //
            .subcommand_required(true)
            .subcommands([
                AddCommand::command(),
                CloneCommand::command(),
                CompletionCommand::command(),
                ImportCommand::command(),
                ListCommand::command(),
                NewCommand::command(),
                RefreshCommand::command(),
            ])
    }

    pub fn from_matches(matches: &ArgMatches) -> Self {
        match matches.subcommand() {
            Some(("add", m)) => Self::Add(AddCommand::from_matches(m)),
            Some(("clone", m)) => Self::Clone(CloneCommand::from_matches(m)),
            Some(("completion", m)) => Self::Completion(CompletionCommand::from_matches(m)),
            Some(("import", m)) => Self::Import(ImportCommand::from_matches(m)),
            Some(("list", m)) => Self::List(ListCommand::from_matches(m)),
            Some(("new", m)) => Self::New(NewCommand::from_matches(m)),
            Some(("refresh", m)) => Self::Refresh(RefreshCommand::from_matches(m)),
            Some(..) | None => unreachable!(),
        }
    }

    pub fn run(self) -> Result<()> {
        match self {
            Self::Add(op) => op.run(),
            Self::Clone(op) => op.run(),
            Self::Completion(op) => op.run(),
            Self::Import(op) => op.run(),
            Self::List(op) => op.run(),
            Self::New(op) => op.run(),
            Self::Refresh(op) => op.run(),
        }
    }
}
