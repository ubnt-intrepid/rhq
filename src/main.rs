extern crate rhq;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    if let Err(message) = rhq::run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
