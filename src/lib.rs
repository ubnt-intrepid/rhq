//!
//! rhq is an alternative of `ghq`
//!

#![forbid(clippy::todo, clippy::unimplemented)]
#![cfg_attr(test, deny(warnings))]

mod cache;
mod config;
mod printer;
mod remote;
mod repository;
mod scp;
mod workspace;

pub mod args;
pub mod query;
pub mod util;
pub mod vcs;

pub use crate::{
    cache::Cache, //
    config::Config,
    query::Query,
    remote::Remote,
    repository::Repository,
    vcs::Vcs,
    workspace::Workspace,
};
