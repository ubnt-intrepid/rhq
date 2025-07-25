use anyhow::Result;
use clap::Command;

mod add;
mod clone;
mod completion;
mod import;
mod list;
mod new;
mod refresh;

macro_rules! def_app {
    ($( $name:expr => [$t:ty: $aliases:expr], )*) => {
        fn app<'help>() -> Command {
            clap::command!()
                .subcommand_required(true)
                $( .subcommand(<$t>::app(Command::new($name)).aliases($aliases as &[&str])) )*
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
