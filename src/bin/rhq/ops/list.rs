use anyhow::Result;
use clap::{arg, builder::PossibleValuesParser, ArgMatches, Command};
use rhq::Workspace;
use std::str::FromStr;

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
    pub fn app(app: Command) -> Command {
        app.about("List local repositories managed by rhq").arg(
            arg!(--format [format] "List format")
                .value_parser(PossibleValuesParser::new(&["name", "fullpath"]))
                .default_value("fullpath"),
        )
    }

    pub fn from_matches(m: &ArgMatches) -> ListCommand {
        ListCommand {
            format: m
                .get_one::<String>("format")
                .and_then(|s| s.parse().ok())
                .unwrap(),
        }
    }

    pub fn run(self) -> Result<()> {
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
