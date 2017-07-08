//! Defines command line interface.

use clap;

pub trait ClapApp {
    fn make_app<'a, 'b: 'a>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b>;
}

pub trait ClapRun {
    fn run(self) -> ::Result<()>;
}

pub fn get_matches<'a, T: ClapApp>() -> clap::ArgMatches<'a> {
    let app = {
        let app = app_from_crate!()
            .setting(clap::AppSettings::VersionlessSubcommands)
            .setting(clap::AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                clap::SubCommand::with_name("completion")
                    .about("Generate completion scripts for your shell")
                    .setting(clap::AppSettings::ArgRequiredElseHelp)
                    .arg(
                        clap::Arg::with_name("shell")
                            .help("target shell")
                            .possible_values(&["bash", "zsh", "fish", "powershell"])
                            .required(true),
                    )
                    .arg(clap::Arg::from_usage("[out-file]  'path to output script'")),
            );
        T::make_app(app)
    };

    let matches = app.clone().get_matches();
    if let ("completion", Some(m)) = matches.subcommand() {
        let shell = m.value_of("shell").and_then(|s| s.parse().ok()).expect(
            "failed to parse target shell",
        );

        if let Some(path) = m.value_of("out-file") {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            app.clone().gen_completions_to(
                env!("CARGO_PKG_NAME"),
                shell,
                &mut file,
            );
        } else {
            app.clone().gen_completions_to(
                env!("CARGO_PKG_NAME"),
                shell,
                &mut ::std::io::stdout(),
            );
        }
        ::std::process::exit(0);
    }
    matches
}
