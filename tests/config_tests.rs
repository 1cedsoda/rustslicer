use rustslicer::config::SlicerConfig;
use tempfile::NamedTempFile;

#[test]
fn test_default_config() {
    let config = SlicerConfig::default();
    assert_eq!(config.layer_height, 0.2);
    assert_eq!(config.infill_percentage, 20);
    assert_eq!(config.print_speed, 60.0);
}

#[test]
fn test_config_save_load() {
    let config = SlicerConfig::default();
    
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();
    
    config.save_to_file(path).unwrap();
    let loaded_config = SlicerConfig::load_from_file(path).unwrap();
    
    assert_eq!(loaded_config.layer_height, config.layer_height);
    assert_eq!(loaded_config.infill_percentage, config.infill_percentage);
}

#[test]
fn test_config_merge() {
    let mut config = SlicerConfig::default();
    config.merge_with_cli(0.1, 50, 80.0, 220, 70);
    
    assert_eq!(config.layer_height, 0.1);
    assert_eq!(config.infill_percentage, 50);
    assert_eq!(config.print_speed, 80.0);
    assert_eq!(config.nozzle_temperature, 220);
    assert_eq!(config.bed_temperature, 70);
}
