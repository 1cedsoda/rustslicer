//! RustSlicer - A high-performance 3D slicer written in Rust
//!
//! This library provides functionality for converting 3D models (STL files)
//! into G-code for 3D printing.

pub mod cli;
pub mod error;
pub mod config;
pub mod geometry;
pub mod slicer;
pub mod gcode;
pub mod commands;

pub use error::{SlicerError, Result};
pub use config::SlicerConfig;
