use anyhow::Result;
use clap::{app_from_crate, App, AppSettings, SubCommand};

mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

macro_rules! def_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn app<'help>() -> App<'help> {
            app_from_crate!()
                .setting(AppSettings::SubcommandRequiredElseHelp)
                $( .subcommand(<$t>::app(SubCommand::with_name($name)).aliases($aliases)) )*
        }

        pub fn run() -> Result<()> {
            let matches = app().get_matches();
            match matches.subcommand() {
                $( Some(($name, m)) => <$t>::from_matches(m).run(), )*
                Some(..) | None => unreachable!(),
            }
        }
    }
}

def_app! {
    "add"        => [self::add::AddCommand: &[]],
    "clone"      => [self::clone::CloneCommand: &["cl"]],
    "completion" => [self::completion::CompletionCommand: &["cmpl"]],
    "import"     => [self::import::ImportCommand: &["imp"]],
    "list"       => [self::list::ListCommand: &["ls"]],
    "new"        => [self::new::NewCommand: &[]],
    "refresh"    => [self::refresh::RefreshCommand: &[]],
}
