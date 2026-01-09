//! RustSlicer - 3D Printing Slicer CLI
//!
//! A high-performance 3D slicer that converts STL files to G-code.

use clap::Parser;
use colored::*;
use std::process;

mod cli;

use cli::{Cli, Commands};

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Run the appropriate command
    let result = match cli.command {
        Commands::Slice {
            input,
            output,
            config,
            layer_height,
            infill_density,
            supports,
            center,
        } => {
            println!("{}", "RustSlicer - 3D Slicer".bold().cyan());
            println!("{}", "=".repeat(40).cyan());
            
            rustslicer::commands::slice::run(
                &input,
                output.as_deref(),
                &config,
                layer_height,
                infill_density,
                supports,
                center,
                cli.verbose,
            )
        }
        Commands::Validate { input, fix } => {
            rustslicer::commands::validate::run(&input, fix, cli.verbose)
        }
        Commands::Analyze { input, config } => {
            rustslicer::commands::analyze::run(&input, config.as_deref(), cli.verbose)
        }
        Commands::Preview {
            input,
            output_dir,
            layers,
            config,
        } => rustslicer::commands::preview::run(&input, &output_dir, layers.as_deref(), &config, cli.verbose),
        Commands::Profiles { details } => {
            rustslicer::commands::profiles::run(details)
        }
    };

    // Handle errors
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}
