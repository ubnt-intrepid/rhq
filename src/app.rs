use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::io::BufRead;
use std::path::PathBuf;

use clap::{self, Arg, SubCommand};
use shlex;
use shellexpand;

use config::{self, Config};
use errors::Result;
use query::Query;
use repository::{self, Repository};
use vcs;

pub struct App {
  config: Config,
}

impl App {
  /// Creates a new instance of rhq application.
  pub fn new() -> Result<App> {
    let config = config::read_all_config()?;
    Ok(App { config: config })
  }

  pub fn create_repository(&self, query: &str, root: Option<&str>, dry_run: bool) -> Result<()> {
    let query: Query = query.parse()?;
    let root = root.and_then(|s| shellexpand::full(s).ok())
      .map(|s| PathBuf::from(s.borrow() as &str))
      .unwrap_or(self.config.root.clone());

    let local_path = root.join(query.to_local_path()?);
    if local_path.is_dir() {
      println!("The directory {} has already existed.",
               local_path.display());
      return Ok(());
    }

    if dry_run {
      println!("launch 'git init {}'", local_path.display());
      Ok(())
    } else {
      vcs::init_repo(&local_path)?;
      vcs::set_remote(&local_path, query.to_url()?)?;
      Ok(())
    }
  }

  pub fn clone_from_queries<Q, R, S>(&self,
                                     queries: R,
                                     args: Vec<S>,
                                     dry_run: bool,
                                     root: Option<&str>)
                                     -> Result<()>
    where Q: AsRef<str>,
          R: Iterator<Item = Q>,
          S: AsRef<OsStr> + Display
  {
    let root = root.map(|s| PathBuf::from(s));
    let root = root.as_ref().unwrap_or(&self.config.root);
    for query in queries {
      let query = query.as_ref().parse()?;
      vcs::clone_from_query(query, root, &args, dry_run)?;
    }
    Ok(())
  }

  pub fn iter_repos<F>(&self, func: F) -> Result<()>
    where F: Fn(&Repository) -> Result<()>
  {
    for root in self.config.roots() {
      for ref repo in repository::collect_from(root) {
        func(repo)?;
      }
    }
    Ok(())
  }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn build_cli() -> clap::App<'static, 'static> {
  cli_template()
    .subcommand(SubCommand::with_name("new")
      .about("Create a new Git repository with intuitive directory structure")
      .arg(Arg::from_usage("<query>         'URL or query of remote repository'"))
      .arg(Arg::from_usage("--root=[root]   'Target root directory of repository"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually create a new repository'")))

    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repositories into the root directory")
      .arg(Arg::from_usage("[query]         'URL or query of remote repository'"))
      .arg(Arg::from_usage("--root=[root]   'Target root directory of cloned repository'"))
      .arg(Arg::from_usage("--arg=[arg]     'Supplemental arguments for Git command'"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually execute Git command'")))

    .subcommand(SubCommand::with_name("list")
      .about("List local repositories managed by rhq"))

    .subcommand(SubCommand::with_name("foreach")
      .about("Execute command into each repositories")
      .arg(Arg::from_usage("<command>       'Command name'"))
      .arg(Arg::from_usage("[args]...       'Arguments of command'"))
      .arg(Arg::from_usage("-n, --dry-run   'Do not actually execute command'")))
}

pub fn run() -> Result<()> {
  let matches = get_matches(build_cli())?;

  let app = App::new()?;
  match matches.subcommand() {
    ("new", Some(m)) => {
      let query = m.value_of("query").unwrap();
      let root = m.value_of("root");
      let dry_run = m.is_present("dry-run");

      app.create_repository(query, root, dry_run)
    }
    ("clone", Some(m)) => {
      let args = m.value_of("arg").and_then(|s| shlex::split(s)).unwrap_or_default();
      let root = m.value_of("root");
      let dry_run = m.is_present("dry-run");

      if let Some(query) = m.value_of("query") {
        app.clone_from_queries(vec![query].into_iter(), args, dry_run, root)?;
      } else {
        let stdin = ::std::io::stdin();
        app.clone_from_queries(stdin.lock().lines().filter_map(|l| l.ok()),
                              args,
                              dry_run,
                              root)?;
      }

      Ok(())
    }
    ("list", _) => {
      app.iter_repos(|ref repo| {
        println!("{}", repo.path_string());
        Ok(())
      })
    }
    ("foreach", Some(m)) => {
      let command = m.value_of("command").unwrap();
      let args: Vec<_> = m.values_of("args").map(|s| s.collect()).unwrap_or_default();
      let dry_run = m.is_present("dry-run");
      app.iter_repos(|repo| if repo.run_command(command, &args, dry_run)? {
        Ok(())
      } else {
        Err("failed to execute command".to_owned().into())
      })
    }
    _ => unreachable!(),
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

fn get_matches<'a, 'b>(mut cli: clap::App<'a, 'b>) -> ::std::io::Result<clap::ArgMatches<'a>> {
  let matches = cli.clone().get_matches();
  if let ("completion", Some(m)) = matches.subcommand() {
    let shell = m.value_of("shell")
      .and_then(|s| s.parse().ok())
      .expect("failed to parse target shell");

    if let Some(path) = m.value_of("out-file") {
      let mut file =
        ::std::fs::OpenOptions::new().write(true).create(true).append(false).open(path)?;
      cli.gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut file);
    } else {
      cli.gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut ::std::io::stdout());
    }
    ::std::process::exit(0);
  }
  Ok(matches)
}
