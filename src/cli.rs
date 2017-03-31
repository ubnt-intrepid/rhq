use std::env;
use std::marker::PhantomData;
use std::path::Path;
use clap::{self, Arg, SubCommand};
use shlex;

use app::{ClapApp, ClapRun};
use core::{Query, Repository, Workspace};
use util;
use vcs::{self, Vcs};


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


pub struct AddCommand<'a> {
  path: Option<&'a Path>,
}

impl<'a> ClapApp for AddCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Add existed repository under management")
       .arg_from_usage("[path]  'Path of local repository'")
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for AddCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> AddCommand<'a> {
    AddCommand { path: m.value_of("path").map(Path::new) }
  }
}

impl<'a> ClapRun for AddCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(None)?;

    let path = self.path
                   .map(|path| if path.is_absolute() {
                          Ok(path.to_owned())
                        } else {
                          env::current_dir().map(|cwd| cwd.join(path))
                        })
                   .unwrap_or_else(|| env::current_dir())?;

    let repo = Repository::from_path(path)?;
    if !repo.is_vcs() {
      Err("Given path is not a repository")?;
    }
    workspace.add_repository(repo);
    workspace.save_cache()?;

    Ok(())
  }
}


/// Subcommand `new`
pub struct NewCommand<'a> {
  query: Query,
  root: Option<&'a Path>,
  dry_run: bool,
  vcs: Option<Vcs>,
}

impl<'a> ClapApp for NewCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Create a new Git repository with intuitive directory structure")
       .arg_from_usage("<query>          'URL or query of remote repository'")
       .arg_from_usage("--root=[root]    'Target root directory of repository")
       .arg_from_usage("-n, --dry-run    'Do not actually create a new repository'")
       .arg(Arg::from_usage("--vcs=[vcs] 'Used Version Control System'").possible_values(Vcs::possible_values()))
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for NewCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> NewCommand<'a> {
    NewCommand {
      query: m.value_of("query")
              .and_then(|s| s.parse().ok())
              .unwrap(),
      root: m.value_of("root").map(Path::new),
      dry_run: m.is_present("dry-run"),
      vcs: m.value_of("vcs").and_then(|s| s.parse().ok()),
    }
  }
}

impl<'a> ClapRun for NewCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(self.root)?;

    let path = {
      let root = workspace.root_dir().ok_or("Unknown root directory")?;
      let path = self.query.to_local_path()?;
      root.join(path)
    };
    if vcs::detect_from_path(&path).is_some() {
      println!("The repository {} has already existed.", path.display());
      return Ok(());
    }

    let vcs = self.vcs.unwrap_or(Vcs::Git);

    print!("Creating an empty repository at \"{}\"", path.display());
    print!(" (VCS: {:?})", vcs);
    println!();

    if !self.dry_run {
      vcs.do_init(&path)?;
      let repo = Repository::from_path(path)?;
      workspace.add_repository(repo);
      workspace.save_cache()?;
    }

    Ok(())
  }
}


/// Subcommand `clone`
pub struct CloneCommand<'a> {
  query: Query,
  root: Option<&'a Path>,
  arg: Option<&'a str>,
  dry_run: bool,
  ssh: bool,
  vcs: Option<Vcs>,
}

impl<'a> ClapApp for CloneCommand<'a> {
  fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
    app.about("Clone remote repositories into the root directory")
       .arg_from_usage("<query>         'URL or query of remote repository'")
       .arg_from_usage("--root=[root]   'Target root directory of cloned repository'")
       .arg_from_usage("--arg=[arg]     'Supplemental arguments for Git command'")
       .arg_from_usage("-n, --dry-run   'Do not actually execute Git command'")
       .arg_from_usage("-s, --ssh       'Use SSH protocol'")
       .arg(Arg::from_usage("--vcs=[vcs] 'Used Version Control System'").possible_values(Vcs::possible_values()))
  }
}

impl<'a, 'b: 'a> From<&'b clap::ArgMatches<'a>> for CloneCommand<'a> {
  fn from(m: &'b clap::ArgMatches<'a>) -> CloneCommand<'a> {
    CloneCommand {
      query: m.value_of("query")
              .and_then(|s| s.parse().ok())
              .unwrap(),
      root: m.value_of("root").map(Path::new),
      arg: m.value_of("arg"),
      dry_run: m.is_present("dry-run"),
      ssh: m.is_present("ssh"),
      vcs: m.value_of("vcs").and_then(|s| s.parse().ok()),
    }
  }
}

impl<'a> ClapRun for CloneCommand<'a> {
  fn run(self) -> ::Result<()> {
    let mut workspace = Workspace::new(self.root)?;

    let path = {
      let root = workspace.root_dir().ok_or("Unknown root directory")?;
      let path = self.query.to_local_path()?;
      root.join(path)
    };
    if vcs::detect_from_path(&path).is_some() {
      println!("The repository {} has already existed.", path.display());
      return Ok(());
    }

    let url = self.query.to_url(self.ssh)?;

    let args = self.arg
                   .and_then(|s| shlex::split(s))
                   .unwrap_or_default();

    let vcs = self.vcs.unwrap_or(Vcs::Git);

    println!("Clone: \"{}\" \"{}\" {}, {:?}",
             url,
             path.display(),
             util::join_str(&args),
             vcs);

    if !self.dry_run {
      vcs.do_clone(&path, &url, &args)?;

      let mut repo = Repository::from_path(path)?;
      repo.set_url(url);
      workspace.add_repository(repo);
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
       .arg_from_usage("--depth=[depth]  'Maximal depth of entries for each base directory'")
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
