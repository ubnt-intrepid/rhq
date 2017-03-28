//! Defines command line interface application.
//!
//! ... which contains supports for management:
//! * to read/write configuration file
//! * to read/write cache file
//! * user interface

pub mod cache;
pub mod command;
pub mod config;

pub use self::cache::Cache;
pub use self::config::Config;

pub fn run() -> ::Result<()> {
  use self::command::{get_matches, Command};

  let matches = get_matches::<Command>();
  let command = Command::from(&matches);

  let cache = Cache::load()?;
  let config = config::read_config()?;
  command.run(cache, config)
}
