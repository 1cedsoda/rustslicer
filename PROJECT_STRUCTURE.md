# RustSlicer Project Structure

This document describes the organization and architecture of the RustSlicer project.

## Directory Structure

```
rustslicer/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root with module exports
│   ├── cli.rs               # Command-line interface with clap
│   ├── error.rs             # Custom error types
│   ├── config/
│   │   └── mod.rs           # Configuration system (TOML)
│   ├── geometry/
│   │   └── mod.rs           # 3D geometry, STL parsing, mesh handling
│   ├── slicer/
│   │   └── mod.rs           # Core slicing algorithm
│   ├── gcode/
│   │   └── mod.rs           # G-code generation
│   └── commands/
│       ├── mod.rs           # Command module exports
│       ├── slice.rs         # Slice command implementation
│       ├── validate.rs      # Validate command
│       ├── config.rs        # Config generation command
│       └── info.rs          # Info display command
├── tests/
│   ├── geometry_tests.rs    # Geometry module tests
│   └── config_tests.rs      # Configuration tests
├── examples/
│   └── configs/
│       ├── default.toml     # Default configuration
│       ├── high_quality.toml # High quality preset
│       └── fast_draft.toml  # Fast draft preset
├── Cargo.toml               # Project manifest
├── LICENSE                  # MIT License
├── README.md                # Main documentation
└── PROJECT_STRUCTURE.md     # This file
```

## Module Overview

### `main.rs`
Entry point for the CLI application. Initializes logging and parses command-line arguments.

### `cli.rs`
Defines the command-line interface using `clap`. Includes:
- Command definitions (slice, validate, config, info)
- Argument parsing
- Command routing

### `error.rs`
Custom error types using `thiserror`:
- `SlicerError`: Main error enum
- Various error variants for different failure modes

### `config/`
Configuration management:
- TOML-based configuration files
- Default values
- CLI parameter merging
- Save/load functionality

### `geometry/`
3D geometry handling:
- `Mesh`: Triangle mesh representation
- `Triangle`: Individual triangle with vertices and normal
- `BoundingBox`: Axis-aligned bounding box
- `LineSegment`: 2D line segment from plane intersection
- STL file parsing using `stl_io`
- Plane-triangle intersection algorithm

### `slicer/`
Core slicing engine:
- `Slicer`: Main slicing coordinator
- `Layer`: Represents a single layer at a Z-height
- `Contour`: Closed or open contour from line segments
- Parallel processing using `rayon`
- Contour building algorithm

### `gcode/`
G-code generation:
- `GCodeGenerator`: Converts layers to G-code
- Header/footer generation
- Layer-by-layer output
- Move and extrusion commands

### `commands/`
CLI command implementations:
- **slice**: Main slicing workflow
- **validate**: STL validation
- **config**: Configuration file generation
- **info**: Model information display

## Data Flow

1. **Input**: User provides STL file and parameters
2. **Parse**: Load and validate STL file into `Mesh`
3. **Slice**: Generate `Layer`s by intersecting triangles with planes
4. **Contour**: Build contours from line segments in each layer
5. **Generate**: Convert layers to G-code commands
6. **Output**: Write G-code file

## Key Algorithms

### Plane-Triangle Intersection
For each triangle and each layer height:
1. Check if triangle's Z-range intersects the layer
2. Find intersection points with triangle edges
3. Return line segment if exactly 2 intersections found

### Contour Building
1. Collect all line segments for a layer
2. Connect segments end-to-end
3. Detect closed contours
4. Classify as outer or inner contours

### G-code Generation
1. Write initialization commands (heating, homing)
2. For each layer:
   - Move to layer Z-height
   - For each contour:
     - Travel to start point
     - Extrude along contour path
3. Write finalization commands (cool down, home)

## Dependencies

- **clap**: Command-line argument parsing
- **serde/toml**: Configuration serialization
- **nalgebra**: 3D math and geometry
- **stl_io**: STL file parsing
- **rayon**: Parallel processing
- **indicatif**: Progress bars
- **anyhow/thiserror**: Error handling
- **log/env_logger**: Logging

## Testing

Tests are located in the `tests/` directory:
- Unit tests for geometry operations
- Configuration serialization tests
- Integration tests (can be added)

Run tests with:
```bash
cargo test
```

## Future Enhancements

- [ ] Infill pattern generation (rectilinear, honeycomb, gyroid)
- [ ] Support structure generation
- [ ] Multi-material support
- [ ] Adaptive layer heights
- [ ] 3MF file format support
- [ ] Web-based preview interface
- [ ] Printer profiles
- [ ] Advanced path optimization
