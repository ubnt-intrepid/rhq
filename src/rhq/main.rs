#![warn(unused_extern_crates)]

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate rhq_core as rhq;
extern crate shlex;

use std::borrow::Cow;
use std::env;
use std::path::Path;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use rhq::{Query, Remote, Repository, Workspace};
use rhq::url::build_url;
use rhq::util;
use rhq::vcs::{self, Vcs};
use rhq::Result;

const POSSIBLE_VCS: &[&str] = &["git", "hg", "darcs", "pijul"];


fn main() {
    env_logger::init().expect("failed to initialize env_logger.");
    if let Err(message) = run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}


macro_rules! def_app {
    ($( $name:expr => $t:ident, )*) => {
        fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            app_from_crate!()
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                $( .subcommand($t::app(SubCommand::with_name($name))) )*
        }

        pub fn run() -> Result<()> {
            let matches = app().get_matches();
            match matches.subcommand() {
                $( ($name, Some(m)) => $t::from_matches(m).run(), )*
                _ => unreachable!(),
            }
        }
    }
}

def_app! {
    "add"        => AddCommand,
    "clone"      => CloneCommand,
    "completion" => CompletionCommand,
    "foreach"    => ForeachCommand,
    "list"       => ListCommand,
    "new"        => NewCommand,
    "refresh"    => RefreshCommand,
}


/// subcommand `add`
struct AddCommand<'a> {
    path: Option<Vec<&'a Path>>,
    verbose: bool,
    import: bool,
    depth: Option<usize>,
}

impl<'a> AddCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add existed repositories into management")
            .arg_from_usage("[path]...       'Location of local repositories'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
            .arg_from_usage("-i, --import    'Use import mode'")
            .arg_from_usage("--depth=[depth] 'Maximal depth of entries for each base directory'")
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> AddCommand<'a> {
        AddCommand {
            path: m.values_of("path").map(|s| s.map(Path::new).collect()),
            verbose: m.is_present("verbose"),
            import: m.is_present("import"),
            depth: m.value_of("depth").and_then(|s| s.parse().ok()),
        }
    }

    fn run_import(&self) -> Result<()> {
        let mut workspace = Workspace::new(None)?;
        if let Some(ref roots) = self.path {
            for root in roots {
                workspace.scan_repositories(root, self.verbose, self.depth)?;
            }
        } else {
            workspace.scan_repositories_default(self.verbose, self.depth)?;
        }
        workspace.save_cache()?;
        Ok(())
    }

    fn run(self) -> Result<()> {
        if self.import {
            return self.run_import();
        }

        let paths: Vec<Cow<Path>> = match self.path {
            Some(ref path) => path.into_iter().map(|&path| path.into()).collect(),
            None => vec![env::current_dir()?.into()],
        };

        let mut workspace = Workspace::new(None)?;
        for path in paths {
            let vcs = match vcs::detect_from_path(&path) {
                Some(vcs) => vcs,
                None => {
                    println!("Ignored: {} is not a repository", path.display());
                    continue;
                }
            };
            let remote = vcs.get_remote_url(&path)
                .ok()
                .map_or(None, |s| s.map(Remote::new));
            let repo = Repository::new(&path, remote)?;

            workspace.add_repository(repo, false);
            if self.verbose {
                println!("Added: {}", util::canonicalize_pretty(&path)?.display());
            }
        }
        workspace.save_cache()?;

        Ok(())
    }
}


/// Subommand `refresh`
struct RefreshCommand {
    verbose: bool,
    sort: bool,
}

impl RefreshCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Scan repository list and drop if it is not existed or matches exclude pattern.")
            .arg_from_usage("-v, --verbose 'Use verbose output'")
            .arg_from_usage("-s, --sort    'Sort by path string'")
    }

    fn from_matches(m: &ArgMatches) -> RefreshCommand {
        RefreshCommand {
            verbose: m.is_present("verbose"),
            sort: m.is_present("sort"),
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
struct NewCommand<'a> {
    path: &'a str,
    vcs: Vcs,
    hook: Option<Vec<String>>,
}

impl<'a> NewCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Create a new repository and add it into management")
            .arg_from_usage("<path>           'Path of target repository, or URL-like pattern'")
            .arg(Arg::from_usage("--vcs=[vcs] 'Used Version Control System'").possible_values(POSSIBLE_VCS))
            .arg_from_usage("--hook=[hook]    'Post hook after initialization'")
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> NewCommand<'a> {
        NewCommand {
            path: m.value_of("path").unwrap(),
            vcs: m.value_of("vcs")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Vcs::Git),
            hook: m.value_of("hook").and_then(|s| shlex::split(s)),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(None)?;

        let path: Cow<Path> = match self.path.parse::<Query>() {
            Ok(query) => workspace.resolve_query(&query)?.into(),
            Err(_) => Path::new(self.path).into(),
        };

        // init
        if vcs::detect_from_path(&path).is_some() {
            println!("The repository {} has already existed.", path.display());
            return Ok(());
        }
        print!("Creating an empty repository at \"{}\"", path.display());
        print!(" (VCS: {:?})", self.vcs);
        println!();
        self.vcs.do_init(&path)?;
        let repo = Repository::new(path, None)?;

        // hook
        if let Some(hook) = self.hook {
            if hook.len() >= 1 {
                repo.run_command(&hook[0], &hook[1..])?;
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
    args: Vec<&'a str>,
    vcs: Vcs,
}

impl<'a> CloneCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Clone remote repositories, and then add it under management")
            .arg_from_usage("<query>          'an URL or a string to determine the URL of remote repository'")
            .arg_from_usage("[dest]           'Destination directory of cloned repository'")
            .arg_from_usage("[args]...        'Supplemental arguments for VCS command'")
            .arg_from_usage("--root=[root]    'Path to determine the destination directory of cloned repository'")
            .arg_from_usage("-s, --ssh        'Use SSH protocol'")
            .arg(Arg::from_usage("--vcs=[vcs] 'Used Version Control System'").possible_values(POSSIBLE_VCS))
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> CloneCommand<'a> {
        CloneCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            dest: m.value_of("dest").map(Path::new),
            root: m.value_of("root").map(Path::new),
            ssh: m.is_present("ssh"),
            args: m.values_of("args").map(|s| s.collect()).unwrap_or_default(),
            vcs: m.value_of("vcs")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Vcs::Git),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new(self.root)?;

        let dest: Cow<Path> = match self.dest {
            Some(dest) => dest.into(),
            None => workspace.resolve_query(&self.query)?.into(),
        };
        let url = build_url(&self.query, self.ssh)?;

        if vcs::detect_from_path(&dest).is_some() {
            println!("The repository {} has already existed.", dest.display());
            return Ok(());
        }

        println!(
            "Clone from {} into {} by using {:?} (with arguments: {})",
            url,
            dest.display(),
            self.vcs,
            util::join_str(&self.args[..]),
        );

        self.vcs.do_clone(&dest, &url, &self.args[..])?;
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
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("List local repositories managed by rhq")
            .arg(clap::Arg::from_usage("--format=[format] 'List format'").possible_values(&["name", "fullpath"]))
    }

    fn from_matches(m: &ArgMatches) -> ListCommand {
        ListCommand {
            format: m.value_of("format")
                .and_then(|s| s.parse().ok())
                .unwrap_or(ListFormat::FullPath),
        }
    }

    fn run(self) -> Result<()> {
        let workspace = Workspace::new(None)?;
        workspace.for_each_repo(|repo| {
            match self.format {
                ListFormat::Name => println!("{}", repo.name()),
                ListFormat::FullPath => println!("{}", repo.path_string()),
            }
            Ok(())
        })
    }
}


/// Subcommand `foreach`
pub struct ForeachCommand<'a> {
    command: &'a str,
    args: Vec<&'a str>,
    dry_run: bool,
}

impl<'a> ForeachCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> clap::App<'a, 'b> {
        app.about("Execute command into each repositories")
            .arg_from_usage("<command>       'Command name'")
            .arg_from_usage("[args]...       'Arguments of command'")
            .arg_from_usage("-n, --dry-run   'Do not actually execute command'")
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> ForeachCommand<'a> {
        ForeachCommand {
            command: m.value_of("command").unwrap(),
            args: m.values_of("args").map(|s| s.collect()).unwrap_or_default(),
            dry_run: m.is_present("dry-run"),
        }
    }

    fn run(self) -> Result<()> {
        let workspace = Workspace::new(None)?;
        workspace.for_each_repo(|repo| {
            if self.dry_run {
                println!("+ {} {}", self.command, util::join_str(&self.args[..]));
            } else {
                repo.run_command(self.command, &self.args[..])?;
            }
            Ok(())
        })
    }
}


pub struct CompletionCommand<'a> {
    shell: clap::Shell,
    out_file: Option<&'a Path>,
}

impl<'a> CompletionCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
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

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> CompletionCommand<'a> {
        CompletionCommand {
            shell: m.value_of("shell").and_then(|s| s.parse().ok()).unwrap(),
            out_file: m.value_of("out-file").map(Path::new),
        }
    }

    fn run(self) -> Result<()> {
        if let Some(path) = self.out_file {
            let mut file = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(path)
                .unwrap();
            app().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut file);
        } else {
            app().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut ::std::io::stdout());
        }
        Ok(())
    }
}
