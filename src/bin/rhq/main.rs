mod ops;

fn main() {
    better_panic::install();
    pretty_env_logger::init();

    if let Err(message) = ops::run() {
        println!("failed with: {}", message);
        std::process::exit(1);
    }
}
