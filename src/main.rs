extern crate rhq;
extern crate env_logger;
use rhq::cli::Command;

fn main() {
  env_logger::init().expect("failed to initialize env_logger.");

  let ref matches = rhq::app::get_matches::<Command>();
  let command: Command = matches.into();

  if let Err(message) = command.run() {
    println!("failed with: {}", message);
    std::process::exit(1);
  }
}
