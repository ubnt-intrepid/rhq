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
#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate shellexpand;
extern crate shlex;
extern crate toml;
extern crate url as url_crate;
extern crate walkdir;

error_chain!{
  foreign_links {
    Io(::std::io::Error);
    TomlSer(::toml::ser::Error);
    TomlDe(::toml::de::Error);
    Json(::serde_json::Error);
    ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
    UrlParse(::url_crate::ParseError);
  }
}

mod cache;
mod config;
mod repository;
mod workspace;

pub mod url;
pub mod util;
pub mod vcs;

pub use self::url::Query;
pub use self::repository::{Remote, Repository};
pub use self::workspace::Workspace;
