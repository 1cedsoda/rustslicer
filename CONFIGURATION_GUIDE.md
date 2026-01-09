# RustSlicer Configuration Guide

## Overview

RustSlicer uses TOML configuration files to define print profiles. The enhanced configuration structure provides dedicated sections for different aspects of 3D printing, making it more organized and intuitive.

## Configuration File Structure

### New Enhanced Structure (v2.0)

The enhanced configuration format organizes settings into logical sections:

```toml
[metadata]        # Profile information
[input]           # Input file and transformations
[output]          # Output file settings
[machine]         # Printer hardware specifications
[quality]         # Layer heights and print quality
[speed]           # Speed settings for different operations
[infill]          # Infill and support settings
[filament]        # Material-specific settings
[gcode]           # G-code generation settings
```

## Section Details

### [metadata]
Profile identification and version information.

```toml
[metadata]
profile_name = "Enhanced PLA Profile"
version = "2.0"
author = "Your Name"
```

**Fields:**
- `profile_name` (string): Name of the print profile
- `version` (string): Profile version
- `author` (string, optional): Profile author

### [input]
Input file configuration and model transformations.

```toml
[input]
stl_path = "examples/models/calibration_cube.stl"
x_rotation = 0.0    # Rotation around X axis in degrees
y_rotation = 0.0    # Rotation around Y axis in degrees
z_rotation = 0.0    # Rotation around Z axis in degrees
scale = 1.0         # Scaling factor (1.0 = original size)
```

**Fields:**
- `stl_path` (path): Path to the STL file to slice
- `x_rotation` (float, optional): Rotation around X axis in degrees (default: 0.0)
- `y_rotation` (float, optional): Rotation around Y axis in degrees (default: 0.0)
- `z_rotation` (float, optional): Rotation around Z axis in degrees (default: 0.0)
- `scale` (float, optional): Scaling factor (default: 1.0)

### [output]
Output file configuration.

```toml
[output]
gcode_path = "output/calibration_cube.gcode"
thumbnail = false   # Generate thumbnail preview
comments = true     # Include comments in G-code
```

**Fields:**
- `gcode_path` (path): Output G-code file path
- `thumbnail` (bool, optional): Generate thumbnail preview (default: false)
- `comments` (bool, optional): Include descriptive comments (default: true)

### [machine]
Printer hardware specifications.

```toml
[machine]
printer_type = "cartesian"
build_volume = [220.0, 220.0, 250.0]  # X, Y, Z in mm
nozzle_diameter = 0.4
filament_diameter = 1.75
max_feedrate = [300.0, 300.0, 12.0, 80.0]  # X, Y, Z, E in mm/s
max_acceleration = [3000.0, 3000.0, 100.0, 5000.0]  # X, Y, Z, E in mm/s²
```

**Fields:**
- `printer_type` (string): Printer type (e.g., "cartesian", "corexy")
- `build_volume` ([float; 3]): Print volume [X, Y, Z] in mm
- `nozzle_diameter` (float): Nozzle diameter in mm
- `filament_diameter` (float): Filament diameter in mm
- `max_feedrate` ([float; 4]): Maximum feedrate [X, Y, Z, E] in mm/s
- `max_acceleration` ([float; 4]): Maximum acceleration [X, Y, Z, E] in mm/s²

### [quality]
Layer heights and print quality settings.

```toml
[quality]
layer_height = 0.2              # Standard layer height in mm
first_layer_height = 0.3        # Thicker first layer for adhesion
line_width = 0.4                # Extrusion width in mm
perimeters = 3                  # Number of perimeter walls
top_solid_layers = 4            # Solid layers on top
bottom_solid_layers = 3         # Solid layers on bottom
```

**Fields:**
- `layer_height` (float): Standard layer height in mm
- `first_layer_height` (float): First layer height (usually thicker)
- `line_width` (float): Extrusion line width in mm
- `perimeters` (int): Number of perimeter walls
- `top_solid_layers` (int): Number of solid top layers
- `bottom_solid_layers` (int): Number of solid bottom layers

### [speed]
Speed settings for different print operations.

```toml
[speed]
external_perimeter_speed = 40.0    # Outer wall speed (mm/s)
perimeter_speed = 60.0             # Inner wall speed (mm/s)
infill_speed = 80.0                # Infill printing speed (mm/s)
solid_infill_speed = 60.0          # Top/bottom solid fill speed (mm/s)
travel_speed = 150.0               # Non-printing moves (mm/s)
first_layer_speed = 20.0           # First layer speed for adhesion (mm/s)
```

**Fields:**
- `external_perimeter_speed` (float): Speed for outer perimeters in mm/s
- `perimeter_speed` (float): Speed for inner perimeters in mm/s
- `infill_speed` (float): Speed for infill in mm/s
- `solid_infill_speed` (float): Speed for solid infill in mm/s
- `travel_speed` (float): Speed for travel moves in mm/s
- `first_layer_speed` (float): Speed for first layer in mm/s

### [infill]
Infill and support structure configuration.

```toml
[infill]
infill_density = 0.20           # 20% infill density
infill_pattern = "gyroid"       # Pattern: rectilinear, honeycomb, gyroid, concentric
support_material = false        # Enable support structures
support_density = 0.15          # Support structure density (15%)
```

**Fields:**
- `infill_density` (float): Infill density (0.0 to 1.0)
- `infill_pattern` (string): Pattern type ("rectilinear", "honeycomb", "gyroid", "concentric")
- `support_material` (bool): Enable support structures
- `support_density` (float): Support material density (0.0 to 1.0)

### [filament]
Material-specific settings.

```toml
[filament]
filament_type = "PLA"
temperature = 210                      # Nozzle temperature (°C)
bed_temperature = 60                   # Bed temperature (°C)
first_layer_temperature = 215          # First layer nozzle temp (°C)
first_layer_bed_temperature = 65       # First layer bed temp (°C)
filament_diameter = 1.75               # Filament diameter (mm)
flow_rate = 1.0                        # Flow multiplier (1.0 = 100%)
fan_speed = 100                        # Cooling fan speed (0-100%)
cooling_min_layer_time = 10.0          # Minimum time per layer (seconds)
```

**Fields:**
- `filament_type` (string): Material type (e.g., "PLA", "PETG", "ABS")
- `temperature` (int): Nozzle temperature in °C
- `bed_temperature` (int): Bed temperature in °C
- `first_layer_temperature` (int, optional): First layer nozzle temperature
- `first_layer_bed_temperature` (int, optional): First layer bed temperature
- `filament_diameter` (float): Filament diameter in mm
- `flow_rate` (float): Extrusion flow multiplier
- `fan_speed` (int): Cooling fan speed (0-100%)
- `cooling_min_layer_time` (float): Minimum time per layer in seconds

### [gcode]
G-code generation settings.

```toml
[gcode]
gcode_flavor = "marlin"
use_relative_e = false

start_gcode = """
G28 ; Home all axes
G1 Z15.0 F6000 ; Move platform down
G92 E0 ; Reset extruder
G1 F200 E3 ; Extrude a short distance
G92 E0 ; Reset extruder again
M117 Printing... ; Display message
"""

end_gcode = """
M104 S0 ; Turn off hotend
M140 S0 ; Turn off bed
G91 ; Relative positioning
G1 E-2 F2700 ; Retract filament
G1 Z10 ; Raise Z
G90 ; Absolute positioning
G28 X Y ; Home X and Y
M84 ; Disable motors
M117 Print complete! ; Display message
"""
```

**Fields:**
- `gcode_flavor` (string): G-code flavor ("marlin", "reprap", etc.)
- `use_relative_e` (bool): Use relative extrusion
- `start_gcode` (string): Custom start G-code
- `end_gcode` (string): Custom end G-code

## Usage Examples

### Basic Usage

```bash
# Slice with enhanced configuration
rustslicer slice model.stl -c enhanced_profile.toml

# Slice with verbose output
rustslicer slice model.stl -c enhanced_profile.toml -v

# Override configuration settings via CLI
rustslicer slice model.stl -c enhanced_profile.toml --layer-height 0.15
rustslicer slice model.stl -c enhanced_profile.toml --infill-density 0.30
```

### Creating Custom Profiles

1. Start with an example profile:
   ```bash
   cp examples/configs/enhanced_profile.toml my_profile.toml
   ```

2. Edit settings for your printer and material:
   - Update `[machine]` section for your printer specifications
   - Adjust `[filament]` section for your material
   - Tune `[quality]` and `[speed]` sections for desired results

3. Test the profile:
   ```bash
   rustslicer slice test_model.stl -c my_profile.toml -v
   ```

## Backward Compatibility

RustSlicer maintains backward compatibility with the legacy configuration format. Old configuration files using `[print_settings]`, `[material]`, and `[advanced]` sections will continue to work.

Legacy sections are automatically mapped to the new structure:
- `[print_settings]` → `[quality]`, `[speed]`, `[infill]`
- `[material]` → `[filament]`
- `[advanced]` → Internal settings

## Material Presets

### PLA Profile
- **Temperature:** 200-220°C
- **Bed:** 60-65°C
- **Speed:** 60-80 mm/s
- **Fan:** 100%

### PETG Profile
- **Temperature:** 230-250°C
- **Bed:** 70-80°C
- **Speed:** 40-60 mm/s
- **Fan:** 50-70%

### ABS Profile
- **Temperature:** 230-250°C
- **Bed:** 100-110°C
- **Speed:** 50-70 mm/s
- **Fan:** 0-30%

## Tips and Best Practices

1. **Layer Height:** Generally 50-80% of nozzle diameter
2. **First Layer:** Use 100-150% of normal layer height
3. **Infill Density:** 15-20% for decorative, 40-60% for functional
4. **Perimeters:** 2-3 walls for most prints, 4+ for strength
5. **Speed:** Slower speeds improve quality, especially for first layer
6. **Support Density:** 15-20% is usually sufficient

## Troubleshooting

### Poor First Layer Adhesion
- Increase `first_layer_bed_temperature`
- Decrease `first_layer_speed`
- Increase `first_layer_height`

### Stringing
- Adjust retraction settings (coming in advanced section)
- Decrease `temperature`
- Increase `travel_speed`

### Weak Parts
- Increase `infill_density`
- Add more `perimeters`
- Increase `line_width`

## See Also

- [Getting Started Guide](GETTING_STARTED.md)
- [Project Structure](PROJECT_STRUCTURE.md)
- [Slicing Implementation](SLICING_IMPLEMENTATION.md)
