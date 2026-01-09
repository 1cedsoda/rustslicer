# RustSlicer 🦀

A high-performance 3D printing slicer written in Rust, designed for speed, reliability, and extensibility.

## Features

- **Fast Slicing:** Parallel processing for quick slice generation
- **Enhanced Configuration:** Organized TOML-based configuration with dedicated sections
- **STL Support:** Read and process binary and ASCII STL files
- **G-code Generation:** Generate printer-ready G-code with custom start/end scripts
- **Multiple Infill Patterns:** Rectilinear, honeycomb, gyroid, and concentric
- **Validation:** Built-in STL mesh validation and repair suggestions
- **Analysis:** Detailed mesh analysis and print time estimation

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rustslicer.git
cd rustslicer

# Build in release mode for optimal performance
cargo build --release

# The binary will be at target/release/rustslicer
```

### Basic Usage

```bash
# Slice an STL file with a configuration profile
./target/release/rustslicer slice examples/models/calibration_cube.stl \
  -c examples/configs/enhanced_profile.toml

# Slice with verbose output
./target/release/rustslicer slice model.stl -c profile.toml -v

# Override configuration settings
./target/release/rustslicer slice model.stl -c profile.toml \
  --layer-height 0.15 \
  --infill-density 0.30

# Validate an STL file
./target/release/rustslicer validate model.stl

# Analyze a model
./target/release/rustslicer analyze model.stl -c profile.toml
```

## Enhanced Configuration Structure (v2.0)

RustSlicer uses an intuitive TOML configuration format organized into logical sections:

```toml
[metadata]        # Profile information
[input]           # Input file and model transformations
[output]          # Output file settings
[machine]         # Printer hardware specifications
[quality]         # Layer heights and print detail
[speed]           # Speed settings for different operations
[infill]          # Infill and support configuration
[filament]        # Material-specific settings
[gcode]           # G-code generation settings
```

### Example Configuration

```toml
[metadata]
profile_name = "Enhanced PLA Profile"
version = "2.0"
author = "Philipp"

[input]
stl_path = "examples/models/calibration_cube.stl"
x_rotation = 0.0
y_rotation = 0.0
z_rotation = 0.0

[machine]
printer_type = "cartesian"
build_volume = [220.0, 220.0, 250.0]
nozzle_diameter = 0.4
filament_diameter = 1.75

[quality]
layer_height = 0.2
first_layer_height = 0.3
perimeters = 3

[speed]
perimeter_speed = 60.0
infill_speed = 80.0
travel_speed = 150.0

[infill]
infill_density = 0.20
infill_pattern = "gyroid"
support_material = false

[filament]
filament_type = "PLA"
temperature = 210
bed_temperature = 60
fan_speed = 100
```

See [CONFIGURATION_GUIDE.md](CONFIGURATION_GUIDE.md) for complete documentation.

## Example Output

```
RustSlicer - 3D Slicer
════════════════════════════════════════════════════════════

→ Loading configuration...
  ✓ Profile: Enhanced PLA Profile

→ Loading STL file...
  ✓ Triangles: 12
  ✓ Vertices: 8
  ✓ Dimensions: 20.0 × 20.0 × 20.0 mm

→ Slicing mesh...
  ✓ Generated 100 layers
  ✓ Non-empty layers: 100

→ Generating G-code...
  ✓ Wrote 40KB to output.gcode

════════════════════════════════════════════════════════════
  Slicing complete!
════════════════════════════════════════════════════════════
```

## Documentation

- **[Configuration Guide](CONFIGURATION_GUIDE.md)** - Complete configuration reference
- **[Getting Started](GETTING_STARTED.md)** - Detailed setup and usage guide
- **[Project Structure](PROJECT_STRUCTURE.md)** - Code organization and architecture
- **[Slicing Implementation](SLICING_IMPLEMENTATION.md)** - Technical details of the slicing algorithm
- **[Testing Guide](TESTING.md)** - How to run and write tests

## Project Structure

```
rustslicer/
├── src/
│   ├── cli/           # Command-line interface
│   ├── commands/      # Command implementations (slice, validate, analyze)
│   ├── config/        # Configuration management
│   ├── geometry/      # 3D geometry primitives and operations
│   ├── gcode/         # G-code generation
│   ├── io/            # File I/O (STL parsing)
│   ├── slicer/        # Core slicing engine
│   └── lib.rs         # Library root
├── examples/
│   ├── configs/       # Example configuration profiles
│   └── models/        # Sample STL files
├── tests/             # Integration tests
└── benches/           # Performance benchmarks
```

## Features in Detail

### Slicing Engine
- Parallel layer generation using Rayon
- Precise plane-triangle intersection
- Contour detection and island separation
- Hole detection within contours

### G-code Generation
- Customizable start and end G-code
- Temperature and bed control
- Support for multiple G-code flavors
- Commented output for readability

### Validation
- Manifold edge checking
- Normal vector validation
- Watertight mesh verification
- Degenerate triangle detection

### Analysis
- Bounding box calculation
- Volume estimation
- Print time estimation
- Layer-by-layer statistics

## Material Profiles

The repository includes pre-configured profiles for common materials:

- **PLA:** Standard profile for PLA filament (200-220°C)
- **PETG:** Higher temperature profile for PETG (230-250°C)
- **Enhanced PLA:** Optimized profile with new configuration structure

## Performance

RustSlicer is designed for performance:

- **Parallel Processing:** Multi-threaded layer generation
- **Efficient Algorithms:** Optimized geometry operations
- **Memory Efficiency:** Minimal memory allocations
- **Release Builds:** Aggressive optimizations enabled

Benchmark a 20mm calibration cube:
- **Slicing:** ~10ms for 100 layers
- **G-code Generation:** ~5ms for 1300+ lines

## Requirements

- **Rust:** 1.70 or higher
- **Operating System:** Linux, macOS, or Windows
- **Memory:** Minimal (depends on model size)

## Dependencies

- `clap` - Command-line argument parsing
- `nalgebra` - Linear algebra and 3D math
- `rayon` - Data parallelism
- `serde` / `toml` - Configuration serialization
- `colored` - Terminal output formatting

## Development

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Run with logging
RUST_LOG=debug cargo run -- slice model.stl -c profile.toml

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

This project is open source. See LICENSE file for details.

## Roadmap

- [ ] Advanced retraction settings
- [ ] Variable layer height
- [ ] Adaptive infill
- [ ] Support structure generation
- [ ] Preview generation (thumbnails)
- [ ] Web-based interface
- [ ] More infill patterns
- [ ] Multi-material support

## Acknowledgments

Built with Rust 🦀 for the 3D printing community.

---

**Note:** This slicer is under active development. Features and APIs may change.
