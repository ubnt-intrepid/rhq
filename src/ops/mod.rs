use clap::{App, AppSettings, SubCommand};
use failure::Fallible;

mod add;
mod clone;
mod completion;
mod generate;
mod import;
mod list;
mod new;
mod refresh;

macro_rules! def_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn app<'a, 'b: 'a>() -> App<'a, 'b> {
            app_from_crate!()
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                $( .subcommand(<$t>::app(SubCommand::with_name($name)).aliases($aliases)) )*
        }

        pub fn run() -> Fallible<()> {
            let matches = app().get_matches();
            match matches.subcommand() {
                $( ($name, Some(m)) => <$t>::from_matches(m).run(), )*
                _ => unreachable!(),
            }
        }
    }
}

def_app! {
    "add"        => [self::add::AddCommand: &[]],
    "clone"      => [self::clone::CloneCommand: &["cl"]],
    "completion" => [self::completion::CompletionCommand: &["cmpl"]],
    "generate"   => [self::generate::GenerateCommand: &[]],
    "import"     => [self::import::ImportCommand: &["imp"]],
    "list"       => [self::list::ListCommand: &["ls"]],
    "new"        => [self::new::NewCommand: &[]],
    "refresh"    => [self::refresh::RefreshCommand: &[]],
}
