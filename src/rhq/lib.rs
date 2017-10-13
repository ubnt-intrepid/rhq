//!
//! rhq is an alternative of `ghq`
//!

#![warn(unused_extern_crates)]

extern crate chrono;
extern crate glob;
extern crate regex;
extern crate shellexpand;
extern crate serde;
extern crate serde_json;
extern crate shlex;
extern crate toml;
extern crate url;
extern crate walkdir;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

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
