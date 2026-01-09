# RustSlicer - Getting Started

## ✅ Project Successfully Initialized!

Your Rust 3D slicer CLI tool is ready for development.

## Project Location
```
~/rustslicer/
```

## Quick Start

### 1. Build the Project
```bash
cd ~/rustslicer
cargo build
```

### 2. Run Tests
```bash
cargo test
```

**Result**: ✅ 3/3 tests passing

### 3. Check CLI Help
```bash
cargo run -- --help
```

### 4. Try the Slice Command (Example)
```bash
# This will show the command structure
cargo run -- slice --help
```

## What's Been Created

### ✅ Complete Module Structure
- **error/** - Error handling (`SlicerError`, `Result<T>`)
- **config/** - Configuration management (TOML profiles)
- **geometry/** - 3D mesh & STL parsing
- **slicer/** - Layer slicing engine
- **gcode/** - G-code generation
- **commands/** - CLI command implementations

### ✅ Example Configuration Files
- `examples/configs/default_profile.toml` - PLA profile
- `examples/configs/petg_profile.toml` - PETG profile

### ✅ Testing Infrastructure
- Integration tests in `tests/`
- Benchmark setup in `benches/`
- All tests passing ✅

### ✅ Documentation
- README.md - User documentation
- PROJECT_STRUCTURE.md - Complete architecture overview
- Inline code documentation

## Key Files to Know

```
src/
├── main.rs          # CLI entry point - Start here for CLI flow
├── lib.rs           # Library root - Public API exports
├── cli.rs           # Command-line argument definitions
│
├── error/mod.rs     # Error types
├── config/mod.rs    # Configuration structures
├── geometry/mod.rs  # STL parsing & mesh handling
├── slicer/mod.rs    # Slicing algorithms (to be implemented)
├── gcode/mod.rs     # G-code generation (to be implemented)
│
└── commands/
    └── slice.rs     # Main slice command implementation
```

## Available CLI Commands

### Slice (Primary Command)
```bash
rustslicer slice <input.stl> [OPTIONS]

Options:
  -o, --output <FILE>          Output G-code file
  -c, --config <FILE>          Configuration profile
  --layer-height <HEIGHT>      Override layer height
  --infill-density <DENSITY>   Override infill density (0.0-1.0)
  --supports                   Enable support generation
```

### Other Commands (Placeholders)
```bash
rustslicer validate <input.stl>   # Validate STL file
rustslicer analyze <input.stl>    # Analyze print
rustslicer preview <input.stl>    # Generate previews
rustslicer profiles               # List profiles
```

## Configuration System

### Example Profile Structure
```toml
[metadata]
profile_name = "My Profile"

[machine]
nozzle_diameter = 0.4
filament_diameter = 1.75
build_volume = [220.0, 220.0, 250.0]

[print_settings]
layer_height = 0.2
infill_density = 0.20  # 20%
infill_pattern = "gyroid"
perimeters = 3

[material]
nozzle_temperature = 210
bed_temperature = 60

[gcode]
gcode_flavor = "marlin"
start_gcode = "G28 ; Home\n"
end_gcode = "M84 ; Motors off\n"
```

## Development Workflow

### Build & Test Cycle
```bash
# Make changes
vim src/slicer/mod.rs

# Check compilation
cargo check

# Run tests
cargo test

# Build optimized
cargo build --release
```

### Running with Logging
```bash
RUST_LOG=info cargo run -- slice model.stl
RUST_LOG=debug cargo run -- slice model.stl -v
```

### Generate Documentation
```bash
cargo doc --open
```

## Next Implementation Steps

### Phase 1: Core Slicing Algorithm
**Location**: `src/slicer/mod.rs`

Implement:
1. Triangle-plane intersection
2. Line segment connection (contour building)
3. Polygon classification (outline vs hole)
4. Island detection

**Key Functions to Add**:
```rust
fn intersect_triangle_plane(tri: &Triangle, z: f64) -> Option<LineSegment>
fn build_contours(segments: Vec<LineSegment>) -> Vec<Polygon>
fn detect_islands(contours: Vec<Polygon>) -> Vec<Island>
```

### Phase 2: Path Generation
**Location**: `src/pathgen/` (to be created)

Implement:
1. Perimeter generation (polygon offsetting)
2. Infill pattern generation
3. Path ordering optimization

### Phase 3: G-code Output
**Location**: `src/gcode/mod.rs`

Implement:
1. Extrusion calculation
2. Move command generation (G0/G1)
3. Retraction logic
4. Speed management

## Code Style Guidelines

1. **Error Handling**: Always use `Result` types
2. **Documentation**: Document public APIs with `///`
3. **Testing**: Write tests alongside code
4. **Naming**: Use clear, descriptive names
5. **Modularity**: Keep functions focused and small

## Useful Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check

# Build documentation
cargo doc --no-deps

# Run specific test
cargo test test_config_load_and_validate

# Run benchmarks
cargo bench

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

## Debugging Tips

### Enable Logging
```bash
export RUST_LOG=debug
cargo run -- slice model.stl
```

### Use Rust Analyzer (VS Code)
Install the rust-analyzer extension for:
- Autocomplete
- Inline errors
- Go to definition
- Refactoring tools

### Print Debugging
```rust
dbg!(&mesh.vertices.len());
println!("Layer {}: {} triangles", idx, count);
```

## Testing Strategy

### Unit Tests
Add to each module:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests
Add to `tests/integration_test.rs`:
```rust
#[test]
fn test_full_pipeline() {
    let config = PrintProfile::default_pla();
    let mesh = Mesh::from_stl("tests/fixtures/cube.stl").unwrap();
    // ...
}
```

## Project Status

### ✅ Completed
- [x] Project initialization
- [x] Module structure
- [x] CLI framework (clap)
- [x] Configuration system (TOML)
- [x] Error handling
- [x] STL file parsing
- [x] Basic layer counting
- [x] G-code header/footer generation
- [x] Test infrastructure
- [x] Documentation

### 🚧 In Progress / TODO
- [ ] Plane-triangle intersection
- [ ] Contour extraction
- [ ] Perimeter generation
- [ ] Infill patterns
- [ ] Extrusion calculation
- [ ] Path optimization
- [ ] Support structures

## Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Relevant Crates Docs
- [clap](https://docs.rs/clap/)
- [serde](https://serde.rs/)
- [nalgebra](https://www.nalgebra.org/)
- [rayon](https://docs.rs/rayon/)

### 3D Printing Resources
- G-code Reference: [RepRap Wiki](https://reprap.org/wiki/G-code)
- Slicing Algorithms: CuraEngine, PrusaSlicer source code
- STL Format: [Wikipedia](https://en.wikipedia.org/wiki/STL_(file_format))

## Get Help

### Compilation Errors
```bash
cargo build 2>&1 | less
```

### Test Failures
```bash
cargo test -- --nocapture
```

### Check Dependencies
```bash
cargo tree
```

## Success Criteria

Your foundation is complete when:
- ✅ Project compiles without errors
- ✅ All tests pass
- ✅ CLI help works
- ✅ Config files parse correctly
- ✅ STL files can be loaded

**All criteria met!** Ready for Phase 1 implementation.

---

**Project**: RustSlicer  
**Status**: Foundation Complete ✅  
**Location**: `~/rustslicer/`  
**Next**: Implement core slicing algorithm  
**Date**: January 9, 2026
