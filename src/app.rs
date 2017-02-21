use std::ffi::OsStr;
use std::fmt::Display;
use std::io::{self, BufRead};

use clap::{self, Arg, SubCommand};
use shlex;

use config::{self, Config};
use errors::Result;
use repository;
use remote;

pub struct App {
  config: Config,
}

impl App {
  /// Creates a new instance of rhq application.
  pub fn new() -> Result<App> {
    let config = config::load_from_home()?;
    Ok(App { config: config })
  }

  pub fn command_clone<S>(&self, query: &str, args: Vec<S>, dry_run: bool) -> Result<()>
    where S: AsRef<OsStr> + Display
  {
    let root = self.config.default_root();
    let query = query.parse()?;
    remote::do_clone(query, root, &args, dry_run)
  }

  pub fn command_import<S>(&self, args: Vec<S>, dry_run: bool) -> Result<()>
    where S: AsRef<OsStr> + Display
  {
    let root = self.config.default_root();
    let stdin = io::stdin();
    for ref query in stdin.lock().lines().filter_map(|l| l.ok()) {
      let query = query.parse()?;
      remote::do_clone(query, root, &args, dry_run)?;
    }
    Ok(())
  }

  pub fn command_list(&self) -> Result<()> {
    for root in &self.config.roots {
      for repo in repository::collect_from(root) {
        println!("{}", repo.path_string());
      }
    }
    Ok(())
  }

  /// Returns the reference of configuration.
  pub fn command_config(&self) -> Result<()> {
    println!("{}", self.config);
    Ok(())
  }
}

fn cli_template<'a, 'b>() -> clap::App<'a, 'b> {
  app_from_crate!()
    .setting(clap::AppSettings::VersionlessSubcommands)
    .setting(clap::AppSettings::SubcommandRequiredElseHelp)
    .subcommand(clap::SubCommand::with_name("completion")
      .about("Generate completion scripts for your shell")
      .setting(clap::AppSettings::ArgRequiredElseHelp)
      .arg(clap::Arg::with_name("shell")
        .help("target shell")
        .possible_values(&["bash", "zsh", "fish", "powershell"])
        .required(true))
      .arg(Arg::from_usage("[out-file]  'path to output script'")))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn build_cli() -> clap::App<'static, 'static> {
  cli_template()
    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repositories into the root directory")
      .arg(Arg::from_usage("<query>         'URL or query of remote repository'"))
      .arg(Arg::from_usage("--arg=[arg]     'Supplemental arguments for Git command'"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually execute Git command'")))

    .subcommand(SubCommand::with_name("import")
      .about("Import remote repositories into the root directory")
      .arg(Arg::from_usage("--arg=[arg]     'Supplemental arguments for Git command'"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually execute Git command'")))

    .subcommand(SubCommand::with_name("list")
      .about("List local repositories managed by rhq"))

    .subcommand(SubCommand::with_name("config")
      .about("Show current configuration"))
}

pub fn run() -> Result<()> {
  let matches = build_cli().get_matches();
  if let ("completion", Some(m)) = matches.subcommand() {
    let shell = m.value_of("shell")
      .and_then(|s| s.parse().ok())
      .expect("failed to parse target shell");

    let mut cli = build_cli();
    if let Some(path) = m.value_of("out-file") {
      let mut file =
        ::std::fs::OpenOptions::new().write(true).create(true).append(false).open(path)?;
      cli.gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut file);
    } else {
      cli.gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut ::std::io::stdout());
    }
    return Ok(());
  }

  let app = App::new()?;
  match matches.subcommand() {
    ("clone", Some(m)) => {
      let query = m.value_of("query").unwrap();
      let args = m.value_of("arg").and_then(|s| shlex::split(s)).unwrap_or_default();
      let dry_run = m.is_present("dry-run");
      app.command_clone(query, args, dry_run)
    }
    ("import", Some(m)) => {
      let args = m.value_of("arg").and_then(|s| shlex::split(s)).unwrap_or_default();
      let dry_run = m.is_present("dry-run");
      app.command_import(args, dry_run)
    }
    ("list", _) => app.command_list(),
    ("config", _) => app.command_config(),
    _ => unreachable!(),
  }
}
