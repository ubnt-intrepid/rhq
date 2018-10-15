#![warn(unused_extern_crates)]

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate rhq;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::env;
use std::path::{Path, PathBuf};

use rhq::{Query, Remote, Result, Vcs, Workspace};

const POSSIBLE_VCS: &[&str] = &["git", "hg", "darcs", "pijul"];

fn main() {
    env_logger::init();
    if let Err(message) = run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}

macro_rules! def_app {
    ($( $name:expr => [$t:ident: $aliases:expr], )*) => {
        fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            app_from_crate!()
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                $( .subcommand($t::app(SubCommand::with_name($name)).aliases($aliases)) )*
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
    "add"        => [AddCommand: &[]],
    "clone"      => [CloneCommand: &["cl"]],
    "completion" => [CompletionCommand: &["cmpl"]],
    "import"     => [ImportCommand: &["imp"]],
    "list"       => [ListCommand: &["ls"]],
    "new"        => [NewCommand: &[]],
    "refresh"    => [RefreshCommand: &[]],
}

struct AddCommand {
    paths: Option<Vec<PathBuf>>,
    verbose: bool,
}

impl AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add existed repositories into management")
            .arg_from_usage("[paths]...      'Location of local repositories'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
    }

    fn from_matches(m: &ArgMatches) -> AddCommand {
        AddCommand {
            paths: m.values_of("paths").map(|s| s.map(PathBuf::from).collect()),
            verbose: m.is_present("verbose"),
        }
    }

    fn run(self) -> Result<()> {
        let paths = self
            .paths
            .unwrap_or_else(|| vec![env::current_dir().expect("env::current_dir()")]);

        let mut workspace = Workspace::new()?.verbose_output(self.verbose);
        for path in paths {
            workspace.add_repository_if_exists(&path)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}

struct ImportCommand {
    roots: Option<Vec<PathBuf>>,
    depth: Option<usize>,
    verbose: bool,
}

impl ImportCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Import existed repositories into management")
            .arg_from_usage("[roots]...      'Root directories contains for scanning'")
            .arg_from_usage("--depth=[depth] 'Maximal depth of entries for each base directory'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
    }

    fn from_matches(m: &ArgMatches) -> ImportCommand {
        ImportCommand {
            roots: m.values_of("roots").map(|s| s.map(PathBuf::from).collect()),
            depth: m.value_of("depth").and_then(|s| s.parse().ok()),
            verbose: m.is_present("verbose"),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);

        let roots = self
            .roots
            .unwrap_or_else(|| workspace.config().include_dirs.clone());
        for root in roots {
            workspace.import_repositories(root, self.depth)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}

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
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);
        workspace.drop_invalid_repositories();
        if self.sort {
            workspace.sort_repositories();
        }
        workspace.save_cache()?;
        Ok(())
    }
}

struct NewCommand<'a> {
    query: Query,
    root: Option<&'a Path>,
    vcs: Vcs,
    ssh: bool,
}

impl<'a> NewCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Create a new repository and add it into management")
            .arg_from_usage("<query>           'Path of target repository, or URL-like pattern'")
            .arg_from_usage(
                "--root=[root]    'Path to determine the destination of new repository'",
            ).arg(
                Arg::from_usage("--vcs=[vcs] 'Used Version Control System'")
                    .possible_values(POSSIBLE_VCS)
                    .default_value("git"),
            ).arg_from_usage("-s, --ssh        'Use SSH protocol instead of HTTP(s)'")
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> NewCommand<'a> {
        NewCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            root: m.value_of("root").map(Path::new),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()).unwrap(),
            ssh: m.is_present("ssh"),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;
        if let Some(root) = self.root {
            workspace.set_root_dir(root);
        }

        workspace.create_repository(&self.query, self.vcs, self.ssh)?;

        workspace.save_cache()?;
        Ok(())
    }
}

struct CloneCommand<'a> {
    query: Query,
    dest: Option<PathBuf>,
    root: Option<&'a Path>,
    ssh: bool,
    vcs: Vcs,
}

impl<'a> CloneCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Clone remote repositories, and then add it under management")
            .arg_from_usage("<query>          'an URL or a string to determine the URL of remote repository'")
            .arg_from_usage("[dest]           'Destination directory of cloned repository'")
            .arg_from_usage("--root=[root]    'Path to determine the destination directory of cloned repository'")
            .arg_from_usage("-s, --ssh        'Use SSH protocol instead of HTTP(s)'")
            .arg(
                Arg::from_usage("--vcs=[vcs] 'Used Version Control System'")
                    .possible_values(POSSIBLE_VCS)
                    .default_value("git"),
            )
    }

    fn from_matches<'b: 'a>(m: &'b ArgMatches<'a>) -> CloneCommand<'a> {
        CloneCommand {
            query: m.value_of("query").and_then(|s| s.parse().ok()).unwrap(),
            dest: m.value_of("dest").map(PathBuf::from),
            root: m.value_of("root").map(Path::new),
            ssh: m.is_present("ssh"),
            vcs: m.value_of("vcs").and_then(|s| s.parse().ok()).unwrap(),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;
        if let Some(root) = self.root {
            workspace.set_root_dir(root);
        }

        let remote = Remote::from_query(&self.query, self.ssh, workspace.default_host())?;
        let dest = match self.dest {
            Some(dest) => dest,
            None => workspace.resolve_query(&self.query)?,
        };
        workspace.clone_repository(remote, &dest, self.vcs)?;

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

struct ListCommand {
    format: ListFormat,
}

impl ListCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("List local repositories managed by rhq").arg(
            Arg::from_usage("--format=[format] 'List format'")
                .possible_values(&["name", "fullpath"])
                .default_value("fullpath"),
        )
    }

    fn from_matches(m: &ArgMatches) -> ListCommand {
        ListCommand {
            format: m.value_of("format").and_then(|s| s.parse().ok()).unwrap(),
        }
    }

    fn run(self) -> Result<()> {
        let workspace = Workspace::new()?;
        workspace.for_each_repo(|repo| {
            match self.format {
                ListFormat::Name => println!("{}", repo.name()),
                ListFormat::FullPath => println!("{}", repo.path_string()),
            }
            Ok(())
        })
    }
}

struct CompletionCommand<'a> {
    shell: clap::Shell,
    out_file: Option<&'a Path>,
}

impl<'a> CompletionCommand<'a> {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Generate completion scripts for your shell")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::from_usage("<shell> 'Target shell'").possible_values(&[
                "bash",
                "zsh",
                "fish",
                "powershell",
            ])).arg_from_usage("[out-file] 'Destination path to generated script'")
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
