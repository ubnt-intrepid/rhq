use std::borrow::Borrow;
use std::io::BufRead;
use std::path::PathBuf;

use clap;
use shlex;
use shellexpand;

use config;
use query::Query;
use repository;
use vcs;
use util;


pub enum Command<'a> {
  New(NewCommand<'a>),
  Clone(CloneCommand<'a>),
  List(ListCommand<'a>),
  Foreach(ForeachCommand<'a>),
}

impl<'a> util::ClapApp for Command<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.subcommand(NewCommand::make_app(clap::SubCommand::with_name("new")))
      .subcommand(CloneCommand::make_app(clap::SubCommand::with_name("clone")))
      .subcommand(ListCommand::make_app(clap::SubCommand::with_name("list")))
      .subcommand(ForeachCommand::make_app(clap::SubCommand::with_name("foreach")))
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for Command<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> Command<'a> {
    match m.subcommand() {
      ("new", Some(m)) => Command::New(m.into()),
      ("clone", Some(m)) => Command::Clone(m.into()),
      ("list", Some(m)) => Command::List(m.into()),
      ("foreach", Some(m)) => Command::Foreach(m.into()),
      _ => unreachable!(),
    }
  }
}

impl<'a> Command<'a> {
  pub fn run(self) -> ::Result<()> {
    match self {
      Command::New(m) => m.run(),
      Command::Clone(m) => m.run(),
      Command::List(m) => m.run(),
      Command::Foreach(m) => m.run(),
    }
  }
}


pub struct NewCommand<'a> {
  query: &'a str,
  root: Option<&'a str>,
  dry_run: bool,
  ssh: bool,
}

impl<'a> util::ClapApp for NewCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Create a new Git repository with intuitive directory structure")
      .arg_from_usage("<query>         'URL or query of remote repository'")
      .arg_from_usage("--root=[root]   'Target root directory of repository")
      .arg_from_usage("-n, --dry-run   'Do not actually create a new repository'")
      .arg_from_usage("-s, --ssh       'Use SSH protocol'")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for NewCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> NewCommand<'a> {
    NewCommand {
      query: m.value_of("query").unwrap(),
      root: m.value_of("root"),
      dry_run: m.is_present("dry-run"),
      ssh: m.is_present("ssh"),
    }
  }
}

impl<'a> NewCommand<'a> {
  fn run(self) -> ::Result<()> {
    // resolve root directory.
    let config = config::read_all_config()?;
    let root = self.root
      .and_then(|s| shellexpand::full(s).ok())
      .map(|s| PathBuf::from(s.borrow() as &str))
      .unwrap_or_else(|| config.root.clone());

    let query: Query = self.query.parse()?;
    let local_path = root.join(query.to_local_path()?);
    if local_path.is_dir() {
      println!("The directory {} has already existed.",
               local_path.display());
      return Ok(());
    }

    if self.dry_run {
      println!("launch 'git init {}'", local_path.display());
      Ok(())
    } else {
      vcs::init_repo(&local_path)?;
      vcs::set_remote(&local_path, &query.to_url(self.ssh)?)?;
      Ok(())
    }
  }
}


pub struct CloneCommand<'a> {
  query: Option<&'a str>,
  arg: Option<&'a str>,
  root: Option<&'a str>,
  dry_run: bool,
  ssh: bool,
}

impl<'a> util::ClapApp for CloneCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Clone remote repositories into the root directory")
      .arg_from_usage("[query]         'URL or query of remote repository'")
      .arg_from_usage("--root=[root]   'Target root directory of cloned repository'")
      .arg_from_usage("--arg=[arg]     'Supplemental arguments for Git command'")
      .arg_from_usage("-n, --dry-run   'Do not actually execute Git command'")
      .arg_from_usage("-s, --ssh       'Use SSH protocol'")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for CloneCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> CloneCommand<'a> {
    CloneCommand {
      query: m.value_of("query"),
      arg: m.value_of("arg"),
      root: m.value_of("root"),
      dry_run: m.is_present("dry-run"),
      ssh: m.is_present("ssh"),
    }
  }
}

impl<'a> CloneCommand<'a> {
  fn run(self) -> ::Result<()> {
    let args = self.arg.and_then(|s| shlex::split(s)).unwrap_or_default();

    let config = config::read_all_config()?;
    let root = self.root.map(|s| PathBuf::from(s));
    let root = root.as_ref().unwrap_or(&config.root);

    if let Some(query) = self.query {
      let query: Query = query.parse()?;
      self.do_clone(query, root, &args)?;
    } else {
      let stdin = ::std::io::stdin();
      let queries = stdin.lock().lines().filter_map(|l| l.ok());
      for query in queries {
        let query: Query = query.parse()?;
        self.do_clone(query, root, &args)?;
      }
    }

    Ok(())
  }

  fn do_clone<P, S>(&self, query: Query, root: P, args: &[S]) -> ::Result<()>
    where P: AsRef<::std::path::Path>,
          S: AsRef<::std::ffi::OsStr> + ::std::fmt::Display
  {
    let path = query.to_local_path()?;
    let path = root.as_ref().join(path);
    let url = query.to_url(self.ssh)?;
    if vcs::detect_from_path(&path).is_some() {
      println!("The repository has already cloned.");
      return Ok(());
    }
    if self.dry_run {
      println!("[debug] git clone '{}' '{}' {}",
               url.as_str(),
               path.display(),
               args.iter().fold(String::new(), |s, a| format!("{} {}", s, a)));
    } else {
      vcs::git::clone(&url, &path, args)?;
    }
    Ok(())
  }
}


pub struct ListCommand<'a> {
  marker: ::std::marker::PhantomData<&'a usize>,
}

impl<'a> util::ClapApp for ListCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("List local repositories managed by rhq")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for ListCommand<'a> {
  fn from(_: &'b clap::ArgMatches<'a>) -> ListCommand<'a> {
    ListCommand { marker: ::std::marker::PhantomData }
  }
}

impl<'a> ListCommand<'a> {
  fn run(self) -> ::Result<()> {
    let config = config::read_all_config()?;
    for root in config.roots() {
      for ref repo in repository::collect_from(root) {
        println!("{}", repo.path_string());
      }
    }

    Ok(())
  }
}


pub struct ForeachCommand<'a> {
  command: &'a str,
  args: Option<clap::Values<'a>>,
  dry_run: bool,
}

impl<'a> util::ClapApp for ForeachCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Execute command into each repositories")
      .arg_from_usage("<command>       'Command name'")
      .arg_from_usage("[args]...       'Arguments of command'")
      .arg_from_usage("-n, --dry-run   'Do not actually execute command'")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for ForeachCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> ForeachCommand<'a> {
    ForeachCommand {
      command: m.value_of("command").unwrap(),
      args: m.values_of("args"),
      dry_run: m.is_present("dry-run"),
    }
  }
}

impl<'a> ForeachCommand<'a> {
  fn run(self) -> ::Result<()> {
    let args: Vec<_> = self.args.map(|s| s.collect()).unwrap_or_default();
    let config = config::read_all_config()?;
    for root in config.roots() {
      for ref repo in repository::collect_from(root) {
        repo.run_command(self.command, &args, self.dry_run)
          .map_err(|_| "failed to execute command".to_owned())?;
      }
    }
    Ok(())
  }
}
