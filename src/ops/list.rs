use clap::{App, Arg, ArgMatches};
use failure::Fallible;
use std::str::FromStr;

use crate::workspace::Workspace;

#[derive(Debug)]
enum ListFormat {
    Name,
    FullPath,
}

impl FromStr for ListFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(ListFormat::Name),
            "fullpath" => Ok(ListFormat::FullPath),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ListCommand {
    format: ListFormat,
}

impl ListCommand {
    pub fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("List local repositories managed by rhq").arg(
            Arg::from_usage("--format=[format] 'List format'")
                .possible_values(&["name", "fullpath"])
                .default_value("fullpath"),
        )
    }

    pub fn from_matches(m: &ArgMatches) -> ListCommand {
        ListCommand {
            format: m.value_of("format").and_then(|s| s.parse().ok()).unwrap(),
        }
    }

    pub fn run(self) -> Fallible<()> {
        let workspace = Workspace::new()?;
        workspace.for_each_repo(|repo| {
            match self.format {
                ListFormat::Name => println!("{}", repo.name()),
                ListFormat::FullPath => println!("{}", repo.path_string()),
            }
            Ok(())
        })
    }
}
