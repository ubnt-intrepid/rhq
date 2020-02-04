use clap::{App, ArgMatches};
use failure::Fallible;
use std::path::PathBuf;

use crate::workspace::Workspace;

#[derive(Debug)]
pub struct ImportCommand {
    roots: Option<Vec<PathBuf>>,
    depth: Option<usize>,
    verbose: bool,
}

impl ImportCommand {
    pub fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Import existed repositories into management")
            .arg_from_usage("[roots]...      'Root directories contains for scanning'")
            .arg_from_usage("--depth=[depth] 'Maximal depth of entries for each base directory'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
    }

    pub fn from_matches(m: &ArgMatches) -> ImportCommand {
        ImportCommand {
            roots: m.values_of("roots").map(|s| s.map(PathBuf::from).collect()),
            depth: m.value_of("depth").and_then(|s| s.parse().ok()),
            verbose: m.is_present("verbose"),
        }
    }

    pub fn run(self) -> Fallible<()> {
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
