#![warn(unused_extern_crates)]

fn main() {
    env_logger::init();
    if let Err(message) = rhq::ops::run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
