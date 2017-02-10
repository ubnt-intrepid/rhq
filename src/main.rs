#[macro_use]
extern crate log;
extern crate env_logger;

extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, Default, Deserialize)]
struct Host {
    name: String,
    host: String,
    vcs: Vec<String>,
    depth: usize,
}

#[derive(Debug, Default, Deserialize)]
struct Config {
    root: String,
    host: String,
    user: String,
    protocol: String,
    hosts: Vec<Host>,
}

fn run() -> io::Result<()> {
    let mut content = String::new();
    File::open("config.toml")?.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content).ok().unwrap_or_default();
    warn!("read config: {:?}", config);

    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    if let Err(message) = run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
