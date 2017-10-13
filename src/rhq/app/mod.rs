//! Defines command line interface application.
//!
//! ... which contains supports for management:
//! * to read/write configuration file
//! * to read/write cache file
//! * user interface

mod cache;
mod config;

pub use self::cache::{Cache, CacheContent};
pub use self::config::{Config, InitialStr};
