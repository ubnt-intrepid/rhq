use anyhow::Context;
use clap::Parser as _;
use rhq::{args::Args, Workspace};

fn main() -> anyhow::Result<()> {
    better_panic::install();
    pretty_env_logger::init();

    let args = Args::parse();
    log::debug!("operation={:?}", args);

    let mut workspace = Workspace::new()
        .context("failed to initialize the rhq context")?
        .verbose_output(args.verbose);

    args.run(&mut workspace)?;

    Ok(())
}
