use anyhow::Result;
use clap::{builder::PossibleValuesParser, ArgMatches, Command};
use std::path::PathBuf;

const POSSIBLE_SHELLS: &[&str] = &["bash", "zsh", "fish", "powershell"];

#[derive(Debug)]
pub struct CompletionCommand {
    shell: clap_complete::Shell,
    out_file: Option<PathBuf>,
}

impl CompletionCommand {
    pub fn command() -> Command {
        Command::new("completion")
            .about("Generate completion scripts for your shell")
            .subcommand_required(true)
            .args(&[
                clap::arg!(<shell> "Target shell")
                    .value_parser(PossibleValuesParser::new(POSSIBLE_SHELLS)),
                clap::arg!([out_file] "Destination path to generated script"),
            ])
            .aliases(&["cmpl"])
    }

    pub fn from_matches(m: &ArgMatches) -> CompletionCommand {
        CompletionCommand {
            shell: m
                .get_one::<String>("shell")
                .and_then(|s| s.parse().ok())
                .unwrap(),
            out_file: m.get_one::<String>("out_file").map(PathBuf::from),
        }
    }

    pub fn run(self) -> Result<()> {
        if let Some(path) = self.out_file {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            clap_complete::generate(
                self.shell,
                &mut super::command(),
                env!("CARGO_PKG_NAME"),
                &mut file,
            );
        } else {
            clap_complete::generate(
                self.shell,
                &mut super::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
        Ok(())
    }
}
