# Changelog

## [2.0.0] - 2026-01-09

### Added - Enhanced Configuration Structure

#### New Configuration Sections
- **[input]** - Dedicated section for input file and model transformations
  - `stl_path`: Path to STL file
  - `x_rotation`, `y_rotation`, `z_rotation`: Model rotation in degrees
  - `scale`: Model scaling factor
  
- **[output]** - Output file configuration
  - `gcode_path`: Output G-code file path
  - `thumbnail`: Generate thumbnail preview (future feature)
  - `comments`: Include descriptive comments in G-code

- **[quality]** - Layer heights and print quality settings
  - `layer_height`: Standard layer height
  - `first_layer_height`: First layer height for adhesion
  - `line_width`: Extrusion line width
  - `perimeters`: Number of perimeter walls
  - `top_solid_layers`, `bottom_solid_layers`: Solid layer counts

- **[speed]** - Dedicated speed settings for different operations
  - `external_perimeter_speed`: Speed for outer walls
  - `perimeter_speed`: Speed for inner walls
  - `infill_speed`: Speed for infill
  - `solid_infill_speed`: Speed for solid infill
  - `travel_speed`: Speed for non-printing moves
  - `first_layer_speed`: Speed for first layer

- **[infill]** - Infill and support structure configuration
  - `infill_density`: Infill density (0.0-1.0)
  - `infill_pattern`: Pattern type (rectilinear, honeycomb, gyroid, concentric)
  - `support_material`: Enable/disable supports
  - `support_density`: Support material density

- **[filament]** - Material-specific settings (replaces [material])
  - `filament_type`: Material type (PLA, PETG, ABS, etc.)
  - `temperature`: Nozzle temperature
  - `bed_temperature`: Bed temperature
  - `first_layer_temperature`: First layer nozzle temperature (optional)
  - `first_layer_bed_temperature`: First layer bed temperature (optional)
  - `filament_diameter`: Filament diameter
  - `flow_rate`: Extrusion flow multiplier
  - `fan_speed`: Cooling fan speed (0-100%)
  - `cooling_min_layer_time`: Minimum time per layer

#### Sample Models
- Added 20mm calibration cube STL file
- Python script to generate simple geometric STL files
- Located in `examples/models/`

#### Documentation
- **CONFIGURATION_GUIDE.md** - Comprehensive configuration reference
  - Detailed explanation of all configuration sections
  - Field descriptions with types and defaults
  - Usage examples and best practices
  - Material presets for common filaments
  - Troubleshooting tips

- **Updated README.md**
  - Quick start guide with new configuration format
  - Enhanced examples
  - Performance benchmarks
  - Project roadmap

#### Code Changes
- Restructured `src/config/mod.rs` to support new configuration sections
- Added helper methods: `get_layer_height()`, `get_infill_density()`
- Updated slicer engine to use new configuration structure
- Updated G-code generator for new filament settings
- Maintained backward compatibility with legacy configuration format

### Enhanced
- **Configuration flexibility** - More granular control over print settings
- **Code organization** - Clearer separation of concerns
- **Documentation** - Comprehensive guides for all features
- **User experience** - More intuitive configuration structure

### Changed
- Configuration structure now uses dedicated sections instead of monolithic blocks
- G-code generator refactored to write directly to file (more efficient)
- Print settings are now distributed across logical sections

### Backward Compatibility
- Legacy configuration files continue to work
- Old `[print_settings]`, `[material]`, and `[advanced]` sections automatically mapped
- Gradual migration path for existing users

### Fixed
- Configuration handling to properly use optional fields
- G-code temperature settings from new filament section
- Layer height and infill density accessors

## [1.0.0] - Previous Version

### Features
- Basic STL file parsing (binary and ASCII)
- Layer-by-layer slicing
- G-code generation
- Mesh validation
- Model analysis
- Command-line interface
- Multiple infill patterns
- Parallel processing
