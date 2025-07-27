use anyhow::Context as _;
use clap::Parser as _;
use rhq::{cli::Args, Cache, Config, Workspace};

fn main() -> anyhow::Result<()> {
    better_panic::install();
    pretty_env_logger::init();

    let args = Args::parse();
    log::debug!("operation={:?}", args);

    let mut config = Config::new(None) //
        .context("failed to load the configuration file")?;
    let mut cache = Cache::new(&config.cache_dir()) //
        .context("failed to load the repository cache")?;
    let mut workspace = Workspace::new(&mut cache, &mut config);

    args.run(&mut workspace)?;

    Ok(())
}
