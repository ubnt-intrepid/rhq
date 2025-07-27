use crate::Workspace;
use anyhow::Result;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
enum ListFormat {
    Name,
    FullPath,
}

impl FromStr for ListFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(ListFormat::Name),
            "fullpath" => Ok(ListFormat::FullPath),
            _ => Err(anyhow::anyhow!("invalid list format")),
        }
    }
}

impl fmt::Display for ListFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name => f.write_str("name"),
            Self::FullPath => f.write_str("fullpath"),
        }
    }
}

#[derive(Debug, clap::Parser)]
#[command(
    name = "list",
    aliases = ["ls"],
    about = "List local repositories managed by rhq"
)]
pub struct ListCommand {
    #[arg(long = "format", help = "List format", default_value_t = ListFormat::FullPath)]
    format: ListFormat,
}

impl ListCommand {
    pub fn run(self, workspace: &mut Workspace) -> Result<()> {
        for repo in workspace.repositories().into_iter().flatten() {
            match self.format {
                ListFormat::Name => println!("{}", repo.name()),
                ListFormat::FullPath => println!("{}", repo.path_string()),
            }
        }

        Ok(())
    }
}
