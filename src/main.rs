use exitfailure::ExitFailure;
use rhq::config::Config;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

/// A Git repository manager.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "rhq",
    raw(setting = "AppSettings::ArgRequiredElseHelp"),
    raw(setting = "AppSettings::SubcommandRequiredElseHelp"),
    raw(setting = "AppSettings::VersionlessSubcommands")
)]
enum Command {
    /// Clones a remote repository and put it under management
    #[structopt(name = "clone")]
    Clone(CloneCommand),

    /// Appends existing local repositories under management
    #[structopt(name = "import")]
    Import(ImportCommand),

    /// Lists the all of managed repositories
    #[structopt(name = "list")]
    List(ListCommand),

    /// Dumps the current configuration to stdout
    #[structopt(name = "config")]
    Config(ConfigCommand),
}

#[derive(Debug, StructOpt)]
struct CloneCommand {
    /// Possible query string to specify the location of remote repository
    query: String,

    /// Enforce to use SSH protocol instead of HTTP/HTTPS
    #[structopt(short = "s", long = "ssh")]
    use_ssh: bool,
}

#[derive(Debug, StructOpt)]
struct ImportCommand {
    /// The location of target repository
    #[structopt(default_value = ".")]
    path: PathBuf,

    /// Traverse the specified directory and appends the *all* of found repositories
    #[structopt(short = "r", long = "recursive")]
    recursive: bool,
}

#[derive(Debug, StructOpt)]
struct ListCommand {
    #[structopt(short = "f", long = "format")]
    format: Option<String>,
}

#[derive(Debug, StructOpt)]
struct ConfigCommand {}

fn main() -> Result<(), ExitFailure> {
    match Command::from_args() {
        Command::Config(_) => {
            let mut config = Config::from_env()?;
            config.fill_default_fields();
            println!("{}", config);
        }
        command => println!("unimplemented command: {:?}", command),
    }
    Ok(())
}
