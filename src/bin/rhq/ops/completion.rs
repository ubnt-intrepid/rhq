use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use std::path::Path;

#[derive(Debug)]
pub struct CompletionCommand<'a> {
    shell: clap_complete::Shell,
    out_file: Option<&'a Path>,
}

impl<'a> CompletionCommand<'a> {
    pub fn app<'help>(app: App<'help>) -> App<'help> {
        app.about("Generate completion scripts for your shell")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::from_usage("<shell> 'Target shell'").possible_values(&[
                "bash",
                "zsh",
                "fish",
                "powershell",
            ]))
            .arg_from_usage("[out-file] 'Destination path to generated script'")
    }

    pub fn from_matches<'b: 'a>(m: &'b ArgMatches) -> CompletionCommand<'b> {
        CompletionCommand {
            shell: m.value_of("shell").and_then(|s| s.parse().ok()).unwrap(),
            out_file: m.value_of("out-file").map(Path::new),
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
                &mut super::app(),
                env!("CARGO_PKG_NAME"),
                &mut file,
            );
        } else {
            clap_complete::generate(
                self.shell,
                &mut super::app(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
        Ok(())
    }
}
