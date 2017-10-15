#![warn(unused_extern_crates)]

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate rhq_core as rhq;
extern crate shlex;

use std::borrow::{Borrow, Cow};
use std::env;
use std::marker::PhantomData;
use std::path::Path;
use clap::{AppSettings, Arg, SubCommand};

use rhq::{Query, Remote, Repository, Workspace};
use rhq::url::build_url;
use rhq::util;
use rhq::vcs::{self, Vcs};
use rhq::Result;


fn main() {
    env_logger::init().expect("failed to initialize env_logger.");

    let app = build_cli();
    let matches = app.get_matches();
    if let Err(message) = Command::from_args(&matches).run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}


/// Toplevel application
pub enum Command<'a> {
    Add(AddCommand<'a>),
    Refresh(RefreshCommand<'a>),
    New(NewCommand<'a>),
    Clone(CloneCommand<'a>),
    List(ListCommand),
    Foreach(ForeachCommand<'a>),
    Completion(CompletionCommand<'a>),
}

impl<'a> Command<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.subcommand(AddCommand::make_app(SubCommand::with_name("add")))
            .subcommand(RefreshCommand::make_app(SubCommand::with_name("refresh")))
            .subcommand(NewCommand::make_app(SubCommand::with_name("new")))
            .subcommand(CloneCommand::make_app(SubCommand::with_name("clone")))
            .subcommand(ListCommand::make_app(SubCommand::with_name("list")))
            .subcommand(ForeachCommand::make_app(SubCommand::with_name("foreach")))
            .subcommand(CompletionCommand::make_app(
                SubCommand::with_name("completion"),
            ))
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> Command<'a> {
        match m.subcommand() {
            ("add", Some(m)) => Command::Add(AddCommand::from_args(m)),
            ("refresh", Some(m)) => Command::Refresh(RefreshCommand::from_args(m)),
            ("new", Some(m)) => Command::New(NewCommand::from_args(m)),
            ("clone", Some(m)) => Command::Clone(CloneCommand::from_args(m)),
            ("list", Some(m)) => Command::List(ListCommand::from_args(m)),
            ("foreach", Some(m)) => Command::Foreach(ForeachCommand::from_args(m)),
            ("completion", Some(m)) => Command::Completion(CompletionCommand::from_args(m)),
            _ => unreachable!(),
        }
    }

    pub fn run(self) -> Result<()> {
        match self {
            Command::Refresh(m) => m.run(),
            Command::Add(m) => m.run(),
            Command::New(m) => m.run(),
            Command::Clone(m) => m.run(),
            Command::List(m) => m.run(),
            Command::Foreach(m) => m.run(),
            Command::Completion(m) => m.run(),
        }
    }
}


/// subcommand `add`
pub struct AddCommand<'a> {
    path: Option<Vec<&'a Path>>,
    verbose: bool,
    import: bool,
    depth: Option<usize>,
}

impl<'a> AddCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Add existed repositories into management")
            .arg_from_usage("[path]...       'Location of local repositories'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
            .arg_from_usage("-i, --import    'Use import mode'")
            .arg_from_usage("--depth=[depth] 'Maximal depth of entries for each base directory'")
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> AddCommand<'a> {
        AddCommand {
            path: m.values_of("path").map(|s| s.map(Path::new).collect()),
            verbose: m.is_present("verbose"),
            import: m.is_present("import"),
            depth: m.value_of("depth").and_then(|s| s.parse().ok()),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(None)?;

        if self.import {
            if let Some(roots) = self.path {
                for root in roots {
                    workspace.scan_repositories(root, self.verbose, self.depth)?;
                }
            } else {
                workspace.scan_repositories_default(self.verbose, self.depth)?;
            }
        } else {
            let paths: Vec<Cow<Path>> = if let Some(ref path) = self.path {
                path.into_iter().map(|&path| path.into()).collect()
            } else {
                vec![env::current_dir()?.into()]
            };
            for path in paths {
                let vcs = match vcs::detect_from_path(&path) {
                    Some(vcs) => vcs,
                    None => {
                        println!("Ignored: {} is not a repository", path.display());
                        continue;
                    }
                };

                let remote = match vcs.get_remote_url(&path)? {
                    Some(url) => url,
                    None => continue,
                };

                let repo = Repository::new(&path, Remote::new(remote))?;

                workspace.add_repository(repo, false);
                if self.verbose {
                    println!("Added: {}", util::canonicalize_pretty(&path)?.display());
                }
            }
        }

        workspace.save_cache()?;
        Ok(())
    }
}


/// Subommand `refresh`
pub struct RefreshCommand<'a> {
    verbose: bool,
    sort: bool,
    marker: PhantomData<&'a usize>,
}

impl<'a> RefreshCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Scan repository list and drop if it is not existed or matches exclude pattern.")
            .arg_from_usage("-v, --verbose 'Use verbose output'")
            .arg_from_usage("-s, --sort    'Sort by path string'")
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> RefreshCommand<'a> {
        RefreshCommand {
            verbose: m.is_present("verbose"),
            sort: m.is_present("sort"),
            marker: PhantomData,
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(None)?;
        workspace.drop_invalid_repositories(self.verbose);
        if self.sort {
            workspace.sort_repositories();
        }
        workspace.save_cache()?;
        Ok(())
    }
}


/// Subcommand `new`
pub struct NewCommand<'a> {
    path: &'a str,
    vcs: Option<Vcs>,
    posthook: Option<&'a str>,
}

impl<'a> NewCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Create a new repository and add it into management")
            .arg_from_usage("<path>                'Path of target repository, or URL-like pattern'")
            .arg(
                Arg::from_usage("--vcs=[vcs]      'Used Version Control System'")
                    .possible_values(Vcs::possible_values()),
            )
            .arg_from_usage("--posthook=[posthook] 'Post hook after initialization'")
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> NewCommand<'a> {
        NewCommand {
            path: m.value_of("path").unwrap(),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()),
            posthook: m.value_of("posthook"),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(None)?;

        let posthook = self.posthook.and_then(|s| shlex::split(s));
        let vcs = self.vcs.unwrap_or(Vcs::Git);
        let path: Cow<Path> = if let Ok(query) = self.path.parse::<Query>() {
            let host = query.host().unwrap_or("github.com");
            let path = query.path();
            workspace
                .root_dir()
                .ok_or("Unknown root directory")?
                .join(host)
                .join(path.borrow() as &str)
                .into()
        } else {
            Path::new(self.path).into()
        };

        // init
        if vcs::detect_from_path(&path).is_some() {
            println!("The repository {} has already existed.", path.display());
            return Ok(());
        }
        print!("Creating an empty repository at \"{}\"", path.display());
        print!(" (VCS: {:?})", vcs);
        println!();
        vcs.do_init(&path)?;
        let repo = Repository::new(path, None)?;

        // hook
        if let Some(posthook) = posthook {
            if posthook.len() >= 1 {
                repo.run_command(&posthook[0], &posthook[1..])?;
            }
        }

        workspace.add_repository(repo, false);
        workspace.save_cache()?;

        Ok(())
    }
}


/// Subcommand `clone`
pub struct CloneCommand<'a> {
    query: Query,
    dest: Option<&'a Path>,
    root: Option<&'a Path>,
    ssh: bool,
    arg: Option<&'a str>,
    vcs: Option<Vcs>,
}

impl<'a> CloneCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Clone remote repositories, and then add it under management")
            .arg_from_usage("<query>         'an URL or a string to determine the URL of remote repository'")
            .arg_from_usage("[dest]          'Destination directory of cloned repository'")
            .arg_from_usage("--root=[root]   'Path to determine the destination directory of cloned repository'")
            .arg_from_usage("-s, --ssh       'Use SSH protocol'")
            .arg_from_usage("--arg=[arg]     'Supplemental arguments for VCS command'")
            .arg(Arg::from_usage("--vcs=[vcs] 'Used Version Control System'").possible_values(Vcs::possible_values()))
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> CloneCommand<'a> {
        CloneCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            dest: m.value_of("dest").map(Path::new),
            root: m.value_of("root").map(Path::new),
            ssh: m.is_present("ssh"),
            arg: m.value_of("arg"),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()),
        }
    }
}

impl<'a> CloneCommand<'a> {
    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(self.root)?;

        let dest: Cow<Path> = if let Some(dest) = self.dest {
            dest.into()
        } else {
            let host = self.query.host().unwrap_or("github.com");
            let path = self.query.path();
            workspace
                .root_dir()
                .ok_or("Unknown root directory")?
                .join(host)
                .join(path.borrow() as &str)
                .into()
        };
        if vcs::detect_from_path(&dest).is_some() {
            println!("The repository {} has already existed.", dest.display());
            return Ok(());
        }

        let url = build_url(&self.query, self.ssh)?;

        let args = self.arg.and_then(|s| shlex::split(s)).unwrap_or_default();

        let vcs = self.vcs.unwrap_or(Vcs::Git);

        println!(
            "Clone from {} into {} by using {:?} (with arguments: {})",
            url,
            dest.display(),
            vcs,
            util::join_str(&args),
        );
        vcs.do_clone(&dest, &url, &args)?;
        let repo = Repository::new(dest, Remote::new(url))?;

        workspace.add_repository(repo, false);

        workspace.save_cache()?;

        Ok(())
    }
}


#[derive(Debug)]
enum ListFormat {
    Name,
    FullPath,
}

impl ::std::str::FromStr for ListFormat {
    type Err = ();
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "name" => Ok(ListFormat::Name),
            "fullpath" => Ok(ListFormat::FullPath),
            _ => Err(()),
        }
    }
}


/// Subcommand `list`
pub struct ListCommand {
    format: ListFormat,
}

impl ListCommand {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("List local repositories managed by rhq")
            .arg(clap::Arg::from_usage("--format=[format] 'List format'").possible_values(&["name", "fullpath"]))
    }

    fn from_args(m: &clap::ArgMatches) -> ListCommand {
        ListCommand {
            format: m.value_of("format")
                .and_then(|s| s.parse().ok())
                .unwrap_or(ListFormat::FullPath),
        }
    }

    fn run(self) -> Result<()> {
        let workspace = Workspace::new(None)?;
        let repos = workspace
            .repositories()
            .ok_or("The cache has not initialized yet")?;
        for repo in repos {
            match self.format {
                ListFormat::Name => println!("{}", repo.name()),
                ListFormat::FullPath => println!("{}", repo.path_string()),
            }
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

impl<'a> ForeachCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Execute command into each repositories")
            .arg_from_usage("<command>       'Command name'")
            .arg_from_usage("[args]...       'Arguments of command'")
            .arg_from_usage("-n, --dry-run   'Do not actually execute command'")
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> ForeachCommand<'a> {
        ForeachCommand {
            command: m.value_of("command").unwrap(),
            args: m.values_of("args"),
            dry_run: m.is_present("dry-run"),
        }
    }

    fn run(self) -> Result<()> {
        let args: Vec<_> = self.args.map(|s| s.collect()).unwrap_or_default();
        let workspace = Workspace::new(None)?;
        let repos = workspace
            .repositories()
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


pub struct CompletionCommand<'a> {
    shell: clap::Shell,
    out_file: Option<&'a Path>,
}

impl<'a> CompletionCommand<'a> {
    fn make_app<'b, 'c: 'b>(app: clap::App<'b, 'c>) -> clap::App<'b, 'c> {
        app.about("Generate completion scripts for your shell")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                clap::Arg::with_name("shell")
                    .help("target shell")
                    .possible_values(&["bash", "zsh", "fish", "powershell"])
                    .required(true),
            )
            .arg(clap::Arg::from_usage("[out-file]  'path to output script'"))
    }

    fn from_args<'b: 'a>(m: &'b clap::ArgMatches<'a>) -> CompletionCommand<'a> {
        CompletionCommand {
            shell: m.value_of("shell").and_then(|s| s.parse().ok()).unwrap(),
            out_file: m.value_of("out-file").map(Path::new),
        }
    }

    fn run(self) -> Result<()> {
        let mut app = build_cli();
        if let Some(path) = self.out_file {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            app.gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut file);
        } else {
            app.gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut ::std::io::stdout());
        }
        ::std::process::exit(0)
    }
}


fn build_cli<'a, 'b: 'a>() -> clap::App<'a, 'b> {
    let app = app_from_crate!()
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp);
    Command::make_app(app)
}
