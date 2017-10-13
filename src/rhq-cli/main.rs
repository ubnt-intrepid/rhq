#![warn(unused_extern_crates)]

#[macro_use]
extern crate clap;
extern crate rhq;
extern crate shlex;
extern crate env_logger;

mod cli;
use cli::Command;

fn main() {
    env_logger::init().expect("failed to initialize env_logger.");

    let matches = &cli::get_matches::<Command>();
    let command: Command = matches.into();

    if let Err(message) = command.run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
