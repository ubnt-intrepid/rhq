//! Defines command line interface application.
//!
//! ... which contains supports for management:
//! * to read/write configuration file
//! * to read/write cache file
//! * user interface

mod cache;
mod cli;
mod config;

pub use self::cache::{Cache, CacheContent};
pub use self::config::{Config, InitialStr};
pub use self::cli::{ClapApp, ClapRun};

pub use self::cli::get_matches;
