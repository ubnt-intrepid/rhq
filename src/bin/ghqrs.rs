extern crate rhq;
extern crate clap;
extern crate env_logger;

use clap::{App, AppSettings, Arg, SubCommand};

fn cli() -> App<'static, 'static> {
  App::new("ghqrs")
    .about("manages Git repositories")
    .version("0.0.0")
    .author("Yusuke Sasaki <yusuke.sasaki.nuem@gmail.com>")
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("list")
      .about("List local repositories in the root directory"))
    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repositories into the root directory")
      .arg(Arg::with_name("query")
        .help("URL or query of remote repository")
        .required(true))
      .arg(Arg::with_name("args")
        .help("supplemental arguments for Git command")
        .multiple(true)
        .required(false)
        .hidden(true)))
}

fn run() -> rhq::errors::Result<()> {
  let matches = cli().get_matches();
  match matches.subcommand() {
    ("clone", Some(m)) => {
      let query = m.value_of("query").unwrap();
      let args: Vec<_> = m.values_of("args").map(|a| a.collect()).unwrap_or_default();
      println!("{}, ({:?})", query, args);
      println!("{:?}", rhq::resolve_query(query));

      Ok(())
    }
    ("list", _) => rhq::list_repositories(),
    _ => Ok(()),
  }
}

fn main() {
  env_logger::init().unwrap();
  if let Err(message) = run() {
    println!("failed with: {}", message);
    std::process::exit(1);
  }
}
