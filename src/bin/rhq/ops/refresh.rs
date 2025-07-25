use anyhow::Result;
use clap::{arg, ArgMatches, Command};
use rhq::Workspace;

#[derive(Debug)]
pub struct RefreshCommand {
    verbose: bool,
    sort: bool,
}

impl RefreshCommand {
    pub fn app(app: Command) -> Command {
        app.about("Scan repository list and drop if it is not existed or matches exclude pattern.")
            .args(&[
                arg!(-v --verbose   "Use verbose output"),
                arg!(-s --sort      "Sort by path string"),
            ])
    }

    pub fn from_matches(m: &ArgMatches) -> RefreshCommand {
        RefreshCommand {
            verbose: m.contains_id("verbose"),
            sort: m.contains_id("sort"),
        }
    }

    pub fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);
        workspace.drop_invalid_repositories();
        if self.sort {
            workspace.sort_repositories();
        }
        workspace.save_cache()?;
        Ok(())
    }
}
