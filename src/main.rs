use clap::Parser as _;
use rhq::args::Args;

fn main() {
    better_panic::install();
    pretty_env_logger::init();

    let args = Args::parse();
    log::debug!("operation={:?}", args);

    if let Err(message) = args.run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
