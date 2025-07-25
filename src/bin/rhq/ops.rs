mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

use anyhow::Result;
use clap::Command;

use crate::ops::{
    add::AddCommand, clone::CloneCommand, completion::CompletionCommand, import::ImportCommand,
    list::ListCommand, new::NewCommand, refresh::RefreshCommand,
};

fn command() -> Command {
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

pub fn run() -> Result<()> {
    let matches = command().get_matches();
    match matches.subcommand() {
        Some(("add", m)) => AddCommand::from_matches(m).run(),
        Some(("clone", m)) => CloneCommand::from_matches(m).run(),
        Some(("completion", m)) => CompletionCommand::from_matches(m).run(),
        Some(("import", m)) => ImportCommand::from_matches(m).run(),
        Some(("list", m)) => ListCommand::from_matches(m).run(),
        Some(("new", m)) => NewCommand::from_matches(m).run(),
        Some(("refresh", m)) => RefreshCommand::from_matches(m).run(),
        Some(..) | None => unreachable!(),
    }
}
