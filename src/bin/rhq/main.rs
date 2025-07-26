use crate::ops::Ops;

mod ops;

fn main() {
    better_panic::install();
    pretty_env_logger::init();

    let matches = Ops::command().get_matches();
    let op = Ops::from_matches(&matches);
    if let Err(message) = op.run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
