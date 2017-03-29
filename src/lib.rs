//!
//! rhq is an alternative of `ghq`
//!

extern crate regex;
extern crate shellexpand;
extern crate serde;
extern crate shlex;
extern crate toml;
extern crate url;
extern crate walkdir;
#[macro_use]
extern crate clap;
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
    ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
    UrlParse(::url::ParseError);
  }
}

pub mod app;
pub mod core;
pub mod cli;
pub mod util;
pub mod vcs;
