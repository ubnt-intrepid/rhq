pub mod cache;
pub mod command;
pub mod config;

use self::command::{get_matches, Command};

pub fn run() -> ::Result<()> {
  let matches = get_matches::<Command>();
  Command::from(&matches).run()
}
