use clap;
use clap::{App, AppSettings, Arg, ArgMatches};
use failure::Fallible;
use std::path::Path;

#[derive(Debug)]
pub struct CompletionCommand<'a> {
    shell: clap::Shell,
    out_file: Option<&'a Path>,
}

impl<'a> CompletionCommand<'a> {
    pub fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
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

    pub fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> CompletionCommand<'a> {
        CompletionCommand {
            shell: m.value_of("shell").and_then(|s| s.parse().ok()).unwrap(),
            out_file: m.value_of("out-file").map(Path::new),
        }
    }

    pub fn run(self) -> Fallible<()> {
        if let Some(path) = self.out_file {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            super::app().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut file);
        } else {
            super::app().gen_completions_to(
                env!("CARGO_PKG_NAME"),
                self.shell,
                &mut ::std::io::stdout(),
            );
        }
        Ok(())
    }
}
