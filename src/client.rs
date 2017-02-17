use std::borrow::Borrow;
use std::io::{self, BufRead};
use std::path::Path;

use clap::{self, Arg, SubCommand};
use shlex;

use config::Config;
use errors::Result;
use local;
use remote;
use process::make_command;

pub struct Client {
  config: Config,
}

impl Client {
  /// Creates a new instance of rhq client.
  pub fn new() -> Result<Client> {
    let config = Config::load()?;
    Ok(Client { config: config })
  }

  /// Performs to clone repository from query.
  ///
  /// If `query` is omitted, use standard input to take queries.
  pub fn command_clone(&self, query: Option<&str>, arg: Option<&str>, dry_run: bool) -> Result<()> {
    let opt_arg: Option<&str> = self.config.clone_arg.as_ref().map(|s| s as &str);
    let args = arg.or(opt_arg).and_then(|a| shlex::split(a)).unwrap_or_default();

    if let Some(query) = query {
      return clone_repository(self.config.default_root(), query, &args, dry_run);
    }

    let stdin = io::stdin();
    for ref query in stdin.lock().lines().filter_map(|l| l.ok()) {
      clone_repository(self.config.default_root(), query, &args, dry_run)?;
    }
    Ok(())
  }

  /// List all of local repositories's path managed from rhq.
  ///
  /// On Windows, the path separaters are replated to '/'.
  pub fn command_list(&self) -> Result<()> {
    for root in &self.config.roots {
      for repo in local::collect_repositories(root) {
        #[cfg(windows)]
        println!("{}", repo.path().to_string_lossy().replace("\\", "/"));
        #[cfg(not(windows))]
        println!("{}", repo.path().display());
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

fn clone_repository(root: &Path, query: &str, args: &[String], dry_run: bool) -> Result<()> {
  let url = remote::build_url(query)?;

  let path = local::make_path_from_url(&url, root)?;
  for repo in local::collect_repositories(root) {
    if path == repo.path() {
      println!("The repository has already cloned.");
      return Ok(());
    }
  }

  if dry_run {
    println!("clone from {:?} into {:?} (args = {:?})", url, path, args);
  } else {
    make_command("git").arg("clone")
      .args(&[url.as_str(), path.to_string_lossy().borrow()])
      .args(&args)
      .status()?;
  }

  Ok(())
}

fn cli_template<'a, 'b>() -> clap::App<'a, 'b> {
  app_from_crate!()
    .setting(clap::AppSettings::VersionlessSubcommands)
    .setting(clap::AppSettings::SubcommandRequiredElseHelp)
    .subcommand(clap::SubCommand::with_name("completions")
      .about("Generate completion scripts for your shell")
      .setting(clap::AppSettings::ArgRequiredElseHelp)
      .arg(clap::Arg::with_name("shell").possible_values(&["bash", "zsh", "fish", "powershell"])))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn build_cli() -> clap::App<'static, 'static> {
  cli_template()
    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repositories into the root directory")
      .arg(Arg::from_usage("[query]         'URL or query of remote repository'"))
      .arg(Arg::from_usage("-a, --arg=[arg] 'Supplemental arguments for Git command'"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually execute Git command'")))

    .subcommand(SubCommand::with_name("list")
      .about("List local repositories managed by rhq"))

    .subcommand(SubCommand::with_name("config")
      .about("Show current configuration"))
}

pub fn run() -> Result<()> {
  let matches = build_cli().get_matches();
  if let ("completions", Some(m)) = matches.subcommand() {
    let shell = m.value_of("shell")
      .and_then(|s| s.parse().ok())
      .expect("failed to parse target shell");
    build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut ::std::io::stdout());
    return Ok(());
  }

  let cli = Client::new()?;
  match matches.subcommand() {
    ("clone", Some(m)) => {
      cli.command_clone(m.value_of("query"),
                        m.value_of("arg"),
                        m.is_present("dry-run"))
    }
    ("list", _) => cli.command_list(),
    ("config", _) => cli.command_config(),
    _ => unreachable!(),
  }
}
