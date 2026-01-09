//! Error types for the slicer

use thiserror::Error;

/// Result type alias for slicer operations
pub type Result<T> = std::result::Result<T, SlicerError>;

/// Main error type for the slicer
#[derive(Debug, Error)]
pub enum SlicerError {
    /// Error reading or parsing STL file
    #[error("Failed to read STL file: {0}")]
    StlReadError(String),

    /// Invalid mesh geometry
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Slicing operation failed
    #[error("Slicing failed: {0}")]
    SlicingError(String),

    /// G-code generation failed
    #[error("G-code generation failed: {0}")]
    GCodeError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// TOML parsing error
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Generic error with context
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SlicerError {
    /// Create a new STL read error
    pub fn stl_read(msg: impl Into<String>) -> Self {
        Self::StlReadError(msg.into())
    }

    /// Create a new invalid mesh error
    pub fn invalid_mesh(msg: impl Into<String>) -> Self {
        Self::InvalidMesh(msg.into())
    }

    /// Create a new config error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Create a new slicing error
    pub fn slicing(msg: impl Into<String>) -> Self {
        Self::SlicingError(msg.into())
    }

    /// Create a new G-code error
    pub fn gcode(msg: impl Into<String>) -> Self {
        Self::GCodeError(msg.into())
    }
}
