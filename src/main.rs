extern crate rhq;
extern crate env_logger;

fn main() {
  env_logger::init().expect("failed to initialize env_logger.");
  if let Err(message) = rhq::app::run() {
    println!("failed with: {}", message);
    std::process::exit(1);
  }
}
