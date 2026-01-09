//! CLI argument parsing and command definitions

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustslicer")]
#[command(version, about = "High-performance 3D slicer for converting STL to G-code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Slice an STL file to G-code
    Slice {
        /// Input STL file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output G-code file (default: input filename with .gcode extension)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Print profile configuration
        #[arg(short, long, value_name = "FILE", default_value = "examples/configs/default_profile.toml")]
        config: PathBuf,

        /// Layer height in mm (overrides config)
        #[arg(long)]
        layer_height: Option<f64>,

        /// Infill density 0.0-1.0 (overrides config)
        #[arg(long)]
        infill_density: Option<f64>,

        /// Enable support generation
        #[arg(long)]
        supports: bool,

        /// Center model on build plate
        #[arg(long, default_value = "true")]
        center: bool,
    },

    /// Validate STL file for errors
    Validate {
        /// Input STL file
        input: PathBuf,

        /// Attempt to fix common issues
        #[arg(short, long)]
        fix: bool,
    },

    /// Analyze print time and material usage
    Analyze {
        /// Input STL file or G-code
        input: PathBuf,

        /// Print profile (if analyzing STL)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Generate layer preview images
    Preview {
        /// Input STL file
        input: PathBuf,

        /// Output directory for preview images
        #[arg(short, long, default_value = "preview")]
        output_dir: PathBuf,

        /// Comma-separated layer indices to preview (e.g., "0,10,50")
        #[arg(short, long)]
        layers: Option<String>,

        /// Print profile
        #[arg(short, long, default_value = "examples/configs/default_profile.toml")]
        config: PathBuf,
    },

    /// List available print profiles
    Profiles {
        /// Show detailed information
        #[arg(short, long)]
        details: bool,
    },
}
