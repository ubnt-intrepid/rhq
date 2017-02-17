extern crate rhq;
extern crate clap;
extern crate env_logger;

use clap::{App, AppSettings, Arg, SubCommand};

fn build_cli() -> App<'static, 'static> {
  App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("list")
      .about("List local repositories in the root directory"))
    .subcommand(SubCommand::with_name("config").about("Show current configuration"))
    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repositories into the root directory")
      .arg(Arg::with_name("query")
        .help("URL or query of remote repository")
        .required(false))
      .arg(Arg::with_name("arg")
        .help("supplemental arguments for Git command")
        .takes_value(true)
        .long("arg")
        .short("a")
        .required(false)))
    .subcommand(SubCommand::with_name("completions")
      .about("Generate completion scripts for your shell")
      .setting(AppSettings::ArgRequiredElseHelp)
      .arg(Arg::with_name("shell")
        .possible_values(&["bash", "zsh", "fish", "powershell"])))
}

fn run() -> rhq::Result<()> {
  let cli = rhq::Client::new()?;

  let matches = build_cli().get_matches();
  match matches.subcommand() {
    ("clone", Some(m)) => cli.command_clone(m.value_of("query"), m.value_of("arg")),
    ("list", _) => cli.command_list(),
    ("config", _) => cli.command_config(),
    ("completions", Some(m)) => {
      let shell: clap::Shell = m.value_of("shell").and_then(|s| s.parse().ok()).expect("failed to parse target shell");
      build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
      Ok(())
    }
    _ => unreachable!(),
  }
}

fn main() {
  env_logger::init().expect("failed to initialize env_logger.");
  if let Err(message) = run() {
    println!("failed with: {}", message);
    std::process::exit(1);
  }
}
