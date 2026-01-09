use anyhow::Result;
use rustslicer::cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    cli.run()
}
