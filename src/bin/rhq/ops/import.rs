use anyhow::Result;
use clap::{arg, ArgMatches, Command};
use rhq::Workspace;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ImportCommand {
    roots: Option<Vec<PathBuf>>,
    depth: Option<usize>,
    verbose: bool,
}

impl ImportCommand {
    pub fn app(app: Command) -> Command {
        app.about("Import existed repositories into management")
            .args(&[
                arg!([roots] ...        "Root directories contains for scanning"),
                arg!(--depth [depth]    "Maximal depth of entries for each base directory"),
                arg!(-v --verbose       "Use verbose output"),
            ])
    }

    pub fn from_matches(m: &ArgMatches) -> ImportCommand {
        ImportCommand {
            roots: m
                .get_many::<String>("roots")
                .map(|s| s.map(PathBuf::from).collect()),
            depth: m.get_one::<String>("depth").and_then(|s| s.parse().ok()),
            verbose: m.contains_id("verbose"),
        }
    }

    pub fn run(self) -> Result<()> {
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
