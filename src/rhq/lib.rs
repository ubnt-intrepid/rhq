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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate shellexpand;
extern crate shlex;
extern crate toml;
extern crate url;
extern crate walkdir;

error_chain!{
  foreign_links {
    Io(::std::io::Error);
    TomlSer(::toml::ser::Error);
    TomlDe(::toml::de::Error);
    Json(::serde_json::Error);
    ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
    UrlParse(::url::ParseError);
  }
}

pub mod app;
pub mod core;
pub mod util;
pub mod vcs;
