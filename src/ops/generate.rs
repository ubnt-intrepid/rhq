use clap::{App, ArgMatches};
use failure::Fallible;

use config::Config;

#[derive(Debug)]
pub struct GenerateCommand {
    _priv: (),
}

impl GenerateCommand {
    pub fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Generates the configuration file")
    }

    pub fn from_matches(_m: &ArgMatches) -> GenerateCommand {
        GenerateCommand { _priv: () }
    }

    pub fn run(self) -> Fallible<()> {
        Config::generate().map_err(Into::into)
    }
}
