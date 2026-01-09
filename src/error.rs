use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlicerError {
    #[error("Failed to read STL file: {0}")]
    StlReadError(String),

    #[error("Invalid STL geometry: {0}")]
    InvalidGeometry(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Slicing error: {0}")]
    SlicingError(String),

    #[error("G-code generation error: {0}")]
    GCodeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, SlicerError>;
