use std::env;
use std::marker::PhantomData;
use clap::{self, SubCommand};
use shlex;

use app::{ClapApp, ClapRun};
use core::{Repository, Workspace};
use util;


/// Toplevel application
pub enum Command<'a> {
  New(NewCommand<'a>),
  Add(AddCommand<'a>),
  Clone(CloneCommand<'a>),
  Scan(ScanCommand<'a>),
  List(ListCommand<'a>),
  Foreach(ForeachCommand<'a>),
}

impl<'a> ClapApp for Command<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.subcommand(NewCommand::make_app(SubCommand::with_name("new")))
       .subcommand(AddCommand::make_app(SubCommand::with_name("add")))
       .subcommand(CloneCommand::make_app(SubCommand::with_name("clone")))
       .subcommand(ListCommand::make_app(SubCommand::with_name("list")))
       .subcommand(ForeachCommand::make_app(SubCommand::with_name("foreach")))
       .subcommand(ScanCommand::make_app(SubCommand::with_name("scan")))
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for Command<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> Command<'a> {
    match m.subcommand() {
      ("new", Some(m)) => Command::New(m.into()),
      ("add", Some(m)) => Command::Add(m.into()),
      ("clone", Some(m)) => Command::Clone(m.into()),
      ("list", Some(m)) => Command::List(m.into()),
      ("foreach", Some(m)) => Command::Foreach(m.into()),
      ("scan", Some(m)) => Command::Scan(m.into()),
      _ => unreachable!(),
    }
  }
}

impl<'a> Command<'a> {
  pub fn run(self) -> ::Result<()> {
    match self {
      Command::New(m) => m.run(),
      Command::Add(m) => m.run(),
      Command::Clone(m) => m.run(),
      Command::List(m) => m.run(),
      Command::Foreach(m) => m.run(),
      Command::Scan(m) => m.run(),
    }
  }
}


/// Subcommand `new`
pub struct NewCommand<'a> {
  query: &'a str,
  root: Option<&'a str>,
  dry_run: bool,
  ssh: bool,
}

impl<'a> ClapApp for NewCommand<'a> {
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

impl<'a> ClapRun for NewCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(self.root)?;

    let query = self.query.parse()?;
    let root = workspace.root_dir().ok_or("Unknown root directory")?;
    let repo = Repository::from_query(root, query, self.ssh)?;

    if self.dry_run {
      println!("+ git init \"{}\"", repo.path_string());
    } else {
      repo.do_init()?;
      workspace.add_repository(repo);
      workspace.save_cache()?;
    }

    Ok(())
  }
}


pub struct AddCommand<'a> {
  path: Option<&'a str>,
}

impl<'a> ClapApp for AddCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Add existed repository into managed")
       .arg_from_usage("[path]  'Path of local repository'")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for AddCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> AddCommand<'a> {
    AddCommand { path: m.value_of("path") }
  }
}

impl<'a> ClapRun for AddCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(None)?;

    let mut path = if let Some(path) = self.path {
      util::make_path_buf(path)?
    } else {
      env::current_dir()?
    };
    if !path.is_absolute() {
      path = env::current_dir()?.join(path);
    }

    let repo = Repository::from_path(path)?;
    if !repo.is_vcs() {
      Err("Given path is not a repository")?;
    }
    workspace.add_repository(repo);
    workspace.save_cache()?;

    Ok(())
  }
}


/// Subcommand `clone`
pub struct CloneCommand<'a> {
  query: Option<&'a str>,
  arg: Option<&'a str>,
  root: Option<&'a str>,
  dry_run: bool,
  ssh: bool,
}

impl<'a> ClapApp for CloneCommand<'a> {
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

impl<'a> ClapRun for CloneCommand<'a> {
  fn run(self) -> ::Result<()> {
    let args = self.arg
                   .and_then(|s| shlex::split(s))
                   .unwrap_or_default();
    let queries = if let Some(query) = self.query {
      vec![query.parse()?]
    } else {
      use std::io::BufRead;
      let stdin = ::std::io::stdin();
      let mut queries = Vec::new();
      for query in stdin.lock().lines().filter_map(|l| l.ok()) {
        queries.push(query.parse()?);
      }
      queries
    };

    let mut workspace = Workspace::new(self.root)?;
    let root = workspace.root_dir().ok_or("Unknown root directory")?;

    for query in queries {
      let repo = Repository::from_query(&root, query, self.ssh)?;
      if self.dry_run {
        let url = repo.url_string().ok_or("Unknown URL")?;
        println!("+ git clone \"{}\" \"{}\" {}",
                 url,
                 repo.path_string(),
                 util::join_str(&args));
      } else {
        repo.do_clone(&args)?;
        workspace.add_repository(repo);
      }
    }

    if !self.dry_run {
      workspace.save_cache()?;
    }

    Ok(())
  }
}


/// Subcommand `scan`
pub struct ScanCommand<'a> {
  verbose: bool,
  prune: bool,
  depth: Option<usize>,
  marker: PhantomData<&'a usize>,
}

impl<'a> ClapApp for ScanCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Scan directories to create cache of repositories list")
       .arg_from_usage("-v, --verbose    'Use verbose output'")
       .arg_from_usage("-p, --prune      'Ignore repositories located at outside of base directories'")
       .arg_from_usage("--depth=[depth]  'Maximal depth of entries for each base directory")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for ScanCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> ScanCommand<'a> {
    ScanCommand {
      verbose: m.is_present("verbose"),
      prune: m.is_present("prune"),
      depth: m.value_of("depth").and_then(|s| s.parse().ok()),
      marker: PhantomData,
    }
  }
}

impl<'a> ClapRun for ScanCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(None)?;
    workspace.scan_repositories(self.verbose, self.prune, self.depth)?;
    workspace.save_cache()?;
    Ok(())
  }
}


/// Subcommand `list`
pub struct ListCommand<'a> {
  marker: PhantomData<&'a usize>,
}

impl<'a> ClapApp for ListCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("List local repositories managed by rhq")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for ListCommand<'a> {
  fn from(_: &'b clap::ArgMatches<'a>) -> ListCommand<'a> {
    ListCommand { marker: PhantomData }
  }
}

impl<'a> ClapRun for ListCommand<'a> {
  fn run(self) -> ::Result<()> {
    let workspace = Workspace::new(None)?;
    let repos = workspace.repositories()
                         .ok_or("The cache has not initialized yet")?;
    for repo in repos {
      println!("{}", repo.path_string());
    }
    Ok(())
  }
}


/// Subcommand `foreach`
pub struct ForeachCommand<'a> {
  command: &'a str,
  args: Option<clap::Values<'a>>,
  dry_run: bool,
}

impl<'a> ClapApp for ForeachCommand<'a> {
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

impl<'a> ClapRun for ForeachCommand<'a> {
  fn run(self) -> ::Result<()> {
    let args: Vec<_> = self.args.map(|s| s.collect()).unwrap_or_default();
    let workspace = Workspace::new(None)?;
    let repos = workspace.repositories()
                         .ok_or("The cache has not initialized yet")?;
    for repo in repos {
      if self.dry_run {
        println!("+ {} {}", self.command, util::join_str(&args));
      } else {
        repo.run_command(self.command, &args)?;
      }
    }
    Ok(())
  }
}
