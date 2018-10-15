#![warn(unused_extern_crates)]

extern crate env_logger;
extern crate rhq;

fn main() {
    env_logger::init();
    if let Err(message) = rhq::ops::run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
