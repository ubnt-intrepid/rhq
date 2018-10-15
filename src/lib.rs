//!
//! rhq is an alternative of `ghq`
//!

#![warn(unused_extern_crates)]

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate glob;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate shellexpand;
extern crate toml;
extern crate url;
extern crate walkdir;

mod cache;
mod config;
mod errors;
mod printer;
mod remote;
mod repository;
mod scp;
mod workspace;

pub mod query;
pub mod util;
pub mod vcs;

pub use self::errors::{Error, ErrorKind, Result};
pub use self::query::Query;
pub use self::remote::Remote;
pub use self::repository::Repository;
pub use self::vcs::Vcs;
pub use self::workspace::Workspace;
