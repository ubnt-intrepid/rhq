//!
//! rhq is an alternative of `ghq`
//!

#![forbid(clippy::todo, clippy::unimplemented)]
#![cfg_attr(test, deny(warnings))]

pub mod cli;

mod cache;
mod config;
mod printer;
mod query;
mod remote;
mod repository;
mod scp;
mod util;
mod vcs;
mod workspace;

pub use crate::{
    cache::Cache, //
    config::Config,
    query::Query,
    remote::Remote,
    repository::Repository,
    vcs::Vcs,
    workspace::Workspace,
};
