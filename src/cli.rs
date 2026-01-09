use clap::{Parser, Subcommand};
use anyhow::Result;
use crate::commands;

#[derive(Parser)]
#[command(name = "rustslicer")]
#[command(author = "Philipp")]
#[command(version = "0.1.0")]
#[command(about = "A high-performance 3D slicer CLI tool written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Slice an STL file to G-code
    Slice {
        /// Input STL file path
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output G-code file path
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<String>,

        /// Layer height in mm
        #[arg(short = 'l', long, default_value = "0.2")]
        layer_height: f64,

        /// Infill percentage (0-100)
        #[arg(short = 'i', long, default_value = "20")]
        infill: u8,

        /// Print speed in mm/s
        #[arg(short = 's', long, default_value = "60")]
        speed: f64,

        /// Nozzle temperature in Celsius
        #[arg(long, default_value = "210")]
        nozzle_temp: u16,

        /// Bed temperature in Celsius
        #[arg(long, default_value = "60")]
        bed_temp: u16,

        /// Configuration file path
        #[arg(short = 'c', long)]
        config: Option<String>,
    },

    /// Validate an STL file
    Validate {
        /// Input STL file path
        #[arg(value_name = "INPUT")]
        input: String,
    },

    /// Generate a configuration file template
    Config {
        /// Output configuration file path
        #[arg(short, long, default_value = "slicer_config.toml")]
        output: String,
    },

    /// Display information about an STL file
    Info {
        /// Input STL file path
        #[arg(value_name = "INPUT")]
        input: String,
    },
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Slice {
                input,
                output,
                layer_height,
                infill,
                speed,
                nozzle_temp,
                bed_temp,
                config,
            } => commands::slice::execute(
                input,
                output.as_deref(),
                *layer_height,
                *infill,
                *speed,
                *nozzle_temp,
                *bed_temp,
                config.as_deref(),
            ),
            Commands::Validate { input } => commands::validate::execute(input),
            Commands::Config { output } => commands::config::execute(output),
            Commands::Info { input } => commands::info::execute(input),
        }
    }
}
