//! # RustSlicer - 3D Printing Slicer Library
//!
//! A high-performance, modular 3D slicer for converting STL files to G-code.
//!
//! ## Architecture
//!
//! The slicer follows a pipeline architecture:
//! 1. **Mesh Loading**: Parse STL and build internal mesh representation
//! 2. **Slicing**: Intersect mesh with horizontal planes to create layers with contours
//! 3. **Island Detection**: Identify separate regions and holes in each layer
//! 4. **Path Generation**: Generate perimeter and infill toolpaths (future)
//! 5. **G-code Generation**: Convert paths to G-code commands
//!
//! ## Example Usage
//!
//! ```ignore
//! use rustslicer::prelude::*;
//!
//! // Load configuration
//! let config = PrintProfile::from_file("profile.toml")?;
//!
//! // Load STL mesh
//! let mesh = Mesh::from_stl("model.stl")?;
//!
//! // Slice the mesh
//! let slicer = SliceEngine::new(mesh, config.clone());
//! let layers = slicer.slice()?;
//!
//! // Generate G-code
//! let gcode = GCodeGenerator::new(config).generate(layers)?;
//! std::fs::write("output.gcode", gcode)?;
//! ```

pub mod commands;
pub mod config;
pub mod error;
pub mod gcode;
pub mod geometry;
pub mod slicer;

/// Convenience re-exports for common types
pub mod prelude {
    pub use crate::config::PrintProfile;
    pub use crate::error::{Result, SlicerError};
    pub use crate::gcode::GCodeGenerator;
    pub use crate::geometry::{Mesh, Polygon, LineSegment2D, BoundingBox};
    pub use crate::slicer::{SliceEngine, Layer, Island};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_loads() {
        // Basic sanity test
        assert!(true);
    }
}
