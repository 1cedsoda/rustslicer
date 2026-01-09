//! Configuration management for print profiles

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SlicerError};

/// Complete print profile configuration with new enhanced structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintProfile {
    #[serde(default)]
    pub metadata: Metadata,
    #[serde(default)]
    pub input: Option<InputSettings>,
    #[serde(default)]
    pub output: Option<OutputSettings>,
    pub machine: MachineConfig,
    #[serde(default)]
    pub quality: Option<QualitySettings>,
    #[serde(default)]
    pub speed: Option<SpeedSettings>,
    #[serde(default)]
    pub infill: Option<InfillSettings>,
    #[serde(default)]
    pub filament: Option<FilamentSettings>,
    // Legacy settings - keep for backward compatibility
    #[serde(default)]
    pub print_settings: Option<PrintSettings>,
    #[serde(default)]
    pub material: Option<MaterialSettings>,
    #[serde(default)]
    pub advanced: Option<AdvancedSettings>,
    pub gcode: GCodeSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[serde(default = "default_profile_name")]
    pub profile_name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub author: String,
}

fn default_profile_name() -> String {
    "Default Profile".to_string()
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Input settings for the STL file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSettings {
    pub stl_path: PathBuf,
    #[serde(default)]
    pub x_rotation: f64,  // degrees
    #[serde(default)]
    pub y_rotation: f64,  // degrees
    #[serde(default)]
    pub z_rotation: f64,  // degrees
    #[serde(default)]
    pub scale: f64,       // scaling factor (1.0 = no scaling)
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            stl_path: PathBuf::from("model.stl"),
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            scale: 1.0,
        }
    }
}

/// Output settings for the G-code file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSettings {
    pub gcode_path: PathBuf,
    #[serde(default)]
    pub thumbnail: bool,
    #[serde(default)]
    pub comments: bool,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            gcode_path: PathBuf::from("output.gcode"),
            thumbnail: false,
            comments: true,
        }
    }
}

/// Quality-related settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySettings {
    pub layer_height: f64,
    #[serde(default = "default_first_layer_height")]
    pub first_layer_height: f64,
    #[serde(default = "default_line_width")]
    pub line_width: f64,
    #[serde(default = "default_perimeters")]
    pub perimeters: usize,
    #[serde(default = "default_top_layers")]
    pub top_solid_layers: usize,
    #[serde(default = "default_bottom_layers")]
    pub bottom_solid_layers: usize,
}

fn default_first_layer_height() -> f64 { 0.3 }
fn default_line_width() -> f64 { 0.4 }
fn default_perimeters() -> usize { 3 }
fn default_top_layers() -> usize { 4 }
fn default_bottom_layers() -> usize { 3 }

/// Speed settings for different print operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedSettings {
    #[serde(default = "default_external_perimeter_speed")]
    pub external_perimeter_speed: f64,
    #[serde(default = "default_perimeter_speed")]
    pub perimeter_speed: f64,
    #[serde(default = "default_infill_speed")]
    pub infill_speed: f64,
    #[serde(default = "default_solid_infill_speed")]
    pub solid_infill_speed: f64,
    #[serde(default = "default_travel_speed")]
    pub travel_speed: f64,
    #[serde(default = "default_first_layer_speed")]
    pub first_layer_speed: f64,
}

fn default_external_perimeter_speed() -> f64 { 40.0 }
fn default_perimeter_speed() -> f64 { 60.0 }
fn default_infill_speed() -> f64 { 80.0 }
fn default_solid_infill_speed() -> f64 { 60.0 }
fn default_travel_speed() -> f64 { 150.0 }
fn default_first_layer_speed() -> f64 { 20.0 }

/// Infill-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfillSettings {
    #[serde(default = "default_infill_density")]
    pub infill_density: f64,  // 0.0 to 1.0
    #[serde(default)]
    pub infill_pattern: InfillPattern,
    #[serde(default)]
    pub support_material: bool,
    #[serde(default = "default_support_density")]
    pub support_density: f64,
}

fn default_infill_density() -> f64 { 0.20 }
fn default_support_density() -> f64 { 0.15 }

/// Filament-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilamentSettings {
    #[serde(default = "default_filament_type")]
    pub filament_type: String,
    pub temperature: u16,
    pub bed_temperature: u16,
    #[serde(default)]
    pub first_layer_temperature: Option<u16>,
    #[serde(default)]
    pub first_layer_bed_temperature: Option<u16>,
    pub filament_diameter: f64,
    #[serde(default = "default_flow_rate")]
    pub flow_rate: f64,  // percentage (1.0 = 100%)
    #[serde(default = "default_fan_speed")]
    pub fan_speed: u8,   // 0-100%
    #[serde(default)]
    pub cooling_min_layer_time: f64,  // seconds
}

fn default_filament_type() -> String { "PLA".to_string() }
fn default_flow_rate() -> f64 { 1.0 }
fn default_fan_speed() -> u8 { 100 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineConfig {
    pub printer_type: String,
    pub build_volume: [f64; 3],
    pub nozzle_diameter: f64,
    #[serde(default = "default_filament_diameter")]
    pub filament_diameter: f64,
    pub max_feedrate: [f64; 4],
    pub max_acceleration: [f64; 4],
}

fn default_filament_diameter() -> f64 { 1.75 }

// Legacy structures for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintSettings {
    pub layer_height: f64,
    pub first_layer_height: f64,
    pub line_width: f64,
    pub perimeters: usize,
    pub external_perimeter_speed: f64,
    pub perimeter_speed: f64,
    pub top_solid_layers: usize,
    pub bottom_solid_layers: usize,
    pub infill_density: f64,
    pub infill_pattern: InfillPattern,
    pub infill_speed: f64,
    pub solid_infill_speed: f64,
    #[serde(default)]
    pub support_material: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InfillPattern {
    Rectilinear,
    Honeycomb,
    Gyroid,
    Concentric,
}

impl Default for InfillPattern {
    fn default() -> Self {
        InfillPattern::Gyroid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSettings {
    pub material_type: String,
    pub nozzle_temperature: u16,
    pub bed_temperature: u16,
    pub first_layer_nozzle_temp: u16,
    pub first_layer_bed_temp: u16,
    pub fan_speed: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub retraction_length: f64,
    pub retraction_speed: f64,
    #[serde(default)]
    pub retraction_z_lift: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GCodeSettings {
    #[serde(default = "default_gcode_flavor")]
    pub gcode_flavor: String,
    #[serde(default)]
    pub use_relative_e: bool,
    #[serde(default)]
    pub start_gcode: String,
    #[serde(default)]
    pub end_gcode: String,
}

fn default_gcode_flavor() -> String {
    "marlin".to_string()
}

impl PrintProfile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| SlicerError::config(format!("Failed to read config: {}", e)))?;
        let profile: PrintProfile = toml::from_str(&content)?;
        profile.validate()?;
        Ok(profile)
    }

    pub fn validate(&self) -> Result<()> {
        // Validate quality settings
        if let Some(ref quality) = self.quality {
            if quality.layer_height <= 0.0 {
                return Err(SlicerError::config("Layer height must be positive"));
            }
        }
        
        // Validate infill settings
        if let Some(ref infill) = self.infill {
            if infill.infill_density < 0.0 || infill.infill_density > 1.0 {
                return Err(SlicerError::config("Infill density must be 0.0-1.0"));
            }
        }
        
        // Validate legacy print_settings if present
        if let Some(ref ps) = self.print_settings {
            if ps.layer_height <= 0.0 {
                return Err(SlicerError::config("Layer height must be positive"));
            }
            if ps.infill_density < 0.0 || ps.infill_density > 1.0 {
                return Err(SlicerError::config("Infill density must be 0.0-1.0"));
            }
        }
        
        Ok(())
    }

    /// Get layer height from either new quality settings or legacy print_settings
    pub fn get_layer_height(&self) -> f64 {
        self.quality.as_ref()
            .map(|q| q.layer_height)
            .or_else(|| self.print_settings.as_ref().map(|ps| ps.layer_height))
            .unwrap_or(0.2)
    }

    /// Get infill density from either new infill settings or legacy print_settings
    pub fn get_infill_density(&self) -> f64 {
        self.infill.as_ref()
            .map(|i| i.infill_density)
            .or_else(|| self.print_settings.as_ref().map(|ps| ps.infill_density))
            .unwrap_or(0.2)
    }

    pub fn default_pla() -> Self {
        Self {
            metadata: Metadata {
                profile_name: "Default PLA".to_string(),
                version: "1.0".to_string(),
                author: "RustSlicer".to_string(),
            },
            input: None,
            output: None,
            machine: MachineConfig {
                printer_type: "cartesian".to_string(),
                build_volume: [220.0, 220.0, 250.0],
                nozzle_diameter: 0.4,
                filament_diameter: 1.75,
                max_feedrate: [300.0, 300.0, 12.0, 80.0],
                max_acceleration: [3000.0, 3000.0, 100.0, 5000.0],
            },
            quality: Some(QualitySettings {
                layer_height: 0.2,
                first_layer_height: 0.3,
                line_width: 0.4,
                perimeters: 3,
                top_solid_layers: 4,
                bottom_solid_layers: 3,
            }),
            speed: Some(SpeedSettings {
                external_perimeter_speed: 40.0,
                perimeter_speed: 60.0,
                infill_speed: 80.0,
                solid_infill_speed: 60.0,
                travel_speed: 150.0,
                first_layer_speed: 20.0,
            }),
            infill: Some(InfillSettings {
                infill_density: 0.20,
                infill_pattern: InfillPattern::Gyroid,
                support_material: false,
                support_density: 0.15,
            }),
            filament: Some(FilamentSettings {
                filament_type: "PLA".to_string(),
                temperature: 210,
                bed_temperature: 60,
                first_layer_temperature: Some(215),
                first_layer_bed_temperature: Some(65),
                filament_diameter: 1.75,
                flow_rate: 1.0,
                fan_speed: 100,
                cooling_min_layer_time: 10.0,
            }),
            print_settings: None,
            material: None,
            advanced: None,
            gcode: GCodeSettings {
                gcode_flavor: "marlin".to_string(),
                use_relative_e: false,
                start_gcode: "G28 ; Home\nG1 Z15.0 F6000\n".to_string(),
                end_gcode: "M104 S0\nM140 S0\nM84\n".to_string(),
            },
        }
    }
}
