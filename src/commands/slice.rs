use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use crate::geometry::Mesh;
use crate::slicer::Slicer;
use crate::gcode::GCodeGenerator;
use crate::config::SlicerConfig;
use std::time::Instant;

pub fn execute(
    input: &str,
    output: Option<&str>,
    layer_height: f64,
    infill: u8,
    speed: f64,
    nozzle_temp: u16,
    bed_temp: u16,
    config_path: Option<&str>,
) -> Result<()> {
    let start_time = Instant::now();

    println!("🦀 RustSlicer v0.1.0");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📁 Input file: {}", input);

    // Load configuration
    let mut config = if let Some(config_path) = config_path {
        println!("⚙️  Loading configuration from: {}", config_path);
        SlicerConfig::load_from_file(config_path)?
    } else {
        SlicerConfig::default()
    };

    // Merge CLI parameters
    config.merge_with_cli(layer_height, infill, speed, nozzle_temp, bed_temp);

    println!("📐 Layer height: {} mm", config.layer_height);
    println!("🔲 Infill: {}%", config.infill_percentage);
    println!("⚡ Print speed: {} mm/s", config.print_speed);
    println!("🌡️  Nozzle temp: {}°C", config.nozzle_temperature);
    println!("🌡️  Bed temp: {}°C", config.bed_temperature);
    println!();

    // Load STL file
    println!("📥 Loading STL file...");
    let mesh = Mesh::from_stl_file(input)?;
    println!("✓ Loaded {} triangles", mesh.triangles.len());

    let dims = mesh.bounds.dimensions();
    println!("📏 Model dimensions: {:.2} x {:.2} x {:.2} mm", dims.x, dims.y, dims.z);
    println!();

    // Validate mesh
    println!("🔍 Validating mesh...");
    mesh.validate()?;
    println!("✓ Mesh is valid");
    println!();

    // Slice the model
    let slicer = Slicer::new(mesh, config.layer_height)?;
    println!("🔪 Slicing model...");
    let layers = slicer.slice()?;
    println!("✓ Generated {} layers", layers.len());
    println!();

    // Generate G-code
    let output_path = output.unwrap_or_else(|| {
        let input_stem = std::path::Path::new(input)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        Box::leak(Box::new(format!("{}.gcode", input_stem))) as &str
    });

    println!("📝 Generating G-code...");
    let generator = GCodeGenerator::new(config);
    generator.generate(&layers, output_path)?;
    println!("✓ G-code written to: {}", output_path);
    println!();

    let duration = start_time.elapsed();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Slicing complete in {:.2}s", duration.as_secs_f64());

    Ok(())
}
