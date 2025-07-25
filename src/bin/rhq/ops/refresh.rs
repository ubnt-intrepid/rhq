use anyhow::Result;
use clap::{App, ArgMatches};
use rhq::Workspace;

#[derive(Debug)]
pub struct RefreshCommand {
    verbose: bool,
    sort: bool,
}

impl RefreshCommand {
    pub fn app<'help>(app: App<'help>) -> App<'help> {
        app.about("Scan repository list and drop if it is not existed or matches exclude pattern.")
            .arg_from_usage("-v, --verbose 'Use verbose output'")
            .arg_from_usage("-s, --sort    'Sort by path string'")
    }

    pub fn from_matches(m: &ArgMatches) -> RefreshCommand {
        RefreshCommand {
            verbose: m.is_present("verbose"),
            sort: m.is_present("sort"),
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
