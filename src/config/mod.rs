use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::{SlicerError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlicerConfig {
    #[serde(default = "default_layer_height")]
    pub layer_height: f64,

    #[serde(default = "default_infill")]
    pub infill_percentage: u8,

    #[serde(default = "default_print_speed")]
    pub print_speed: f64,

    #[serde(default = "default_travel_speed")]
    pub travel_speed: f64,

    #[serde(default = "default_nozzle_temp")]
    pub nozzle_temperature: u16,

    #[serde(default = "default_bed_temp")]
    pub bed_temperature: u16,

    #[serde(default = "default_nozzle_diameter")]
    pub nozzle_diameter: f64,

    #[serde(default = "default_filament_diameter")]
    pub filament_diameter: f64,

    #[serde(default = "default_retraction_distance")]
    pub retraction_distance: f64,

    #[serde(default = "default_retraction_speed")]
    pub retraction_speed: f64,

    #[serde(default = "default_wall_thickness")]
    pub wall_thickness: f64,

    #[serde(default = "default_top_bottom_thickness")]
    pub top_bottom_thickness: f64,
}

fn default_layer_height() -> f64 { 0.2 }
fn default_infill() -> u8 { 20 }
fn default_print_speed() -> f64 { 60.0 }
fn default_travel_speed() -> f64 { 120.0 }
fn default_nozzle_temp() -> u16 { 210 }
fn default_bed_temp() -> u16 { 60 }
fn default_nozzle_diameter() -> f64 { 0.4 }
fn default_filament_diameter() -> f64 { 1.75 }
fn default_retraction_distance() -> f64 { 5.0 }
fn default_retraction_speed() -> f64 { 40.0 }
fn default_wall_thickness() -> f64 { 0.8 }
fn default_top_bottom_thickness() -> f64 { 0.8 }

impl Default for SlicerConfig {
    fn default() -> Self {
        Self {
            layer_height: default_layer_height(),
            infill_percentage: default_infill(),
            print_speed: default_print_speed(),
            travel_speed: default_travel_speed(),
            nozzle_temperature: default_nozzle_temp(),
            bed_temperature: default_bed_temp(),
            nozzle_diameter: default_nozzle_diameter(),
            filament_diameter: default_filament_diameter(),
            retraction_distance: default_retraction_distance(),
            retraction_speed: default_retraction_speed(),
            wall_thickness: default_wall_thickness(),
            top_bottom_thickness: default_top_bottom_thickness(),
        }
    }
}

impl SlicerConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .map_err(|e| SlicerError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&contents)
            .map_err(|e| SlicerError::ConfigError(format!("Failed to parse config: {}", e)))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| SlicerError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path, contents)
            .map_err(|e| SlicerError::ConfigError(format!("Failed to write config file: {}", e)))
    }

    pub fn merge_with_cli(&mut self, layer_height: f64, infill: u8, speed: f64, nozzle_temp: u16, bed_temp: u16) {
        self.layer_height = layer_height;
        self.infill_percentage = infill;
        self.print_speed = speed;
        self.nozzle_temperature = nozzle_temp;
        self.bed_temperature = bed_temp;
    }
}
