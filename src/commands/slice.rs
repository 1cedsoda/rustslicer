//! Slice command implementation

use std::path::Path;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::PrintProfile;
use crate::error::Result;
use crate::geometry::Mesh;
use crate::slicer::SliceEngine;
use crate::gcode::GCodeGenerator;

pub fn run(
    input: &Path,
    output: Option<&Path>,
    config_path: &Path,
    layer_height: Option<f64>,
    infill_density: Option<f64>,
    _supports: bool,
    _center: bool,
    verbose: bool,
) -> Result<()> {
    println!("{}", "═".repeat(60).cyan());
    println!("{}", "  RustSlicer - 3D Slicer".bold().cyan());
    println!("{}", "═".repeat(60).cyan());
    println!();

    // Load configuration
    println!("{} {}", "→".cyan().bold(), "Loading configuration...".bold());
    let mut config = match PrintProfile::from_file(config_path) {
        Ok(c) => {
            println!(
                "  {} Profile: {}",
                "✓".green(),
                c.metadata.profile_name.bold()
            );
            c
        }
        Err(e) => {
            eprintln!("  {} Failed to load config: {}", "✗".red(), e);
            return Err(e);
        }
    };

    // Override with CLI parameters
    if let Some(height) = layer_height {
        if let Some(ref mut quality) = config.quality {
            quality.layer_height = height;
        } else if let Some(ref mut ps) = config.print_settings {
            ps.layer_height = height;
        }
        println!(
            "  {} Layer height: {:.3}mm",
            "↻".yellow(),
            height
        );
    }
    if let Some(density) = infill_density {
        if let Some(ref mut infill) = config.infill {
            infill.infill_density = density;
        } else if let Some(ref mut ps) = config.print_settings {
            ps.infill_density = density;
        }
        println!(
            "  {} Infill density: {}%",
            "↻".yellow(),
            (density * 100.0) as u8
        );
    }
    println!();

    // Load STL
    println!("{} {}", "→".cyan().bold(), "Loading STL file...".bold());
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Parsing STL...");

    let mesh = match Mesh::from_stl(input) {
        Ok(m) => {
            pb.finish_and_clear();
            m
        }
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("  {} Failed to load STL: {}", "✗".red(), e);
            return Err(e);
        }
    };

    let dims = mesh.bounds.dimensions();
    println!("  {} Triangles: {}", "✓".green(), mesh.triangle_count());
    println!("  {} Vertices: {}", "✓".green(), mesh.vertex_count());
    println!(
        "  {} Dimensions: {:.1} × {:.1} × {:.1} mm",
        "✓".green(),
        dims.x,
        dims.y,
        dims.z
    );
    println!(
        "  {} Volume bounds: ({:.1}, {:.1}, {:.1}) to ({:.1}, {:.1}, {:.1})",
        "✓".green(),
        mesh.bounds.min.x,
        mesh.bounds.min.y,
        mesh.bounds.min.z,
        mesh.bounds.max.x,
        mesh.bounds.max.y,
        mesh.bounds.max.z,
    );
    println!();

    // Calculate expected layers
    let total_height = dims.z;
    let first_layer_height = config.quality.as_ref().map(|q| q.first_layer_height).or_else(|| config.print_settings.as_ref().map(|ps| ps.first_layer_height)).unwrap_or(0.3);
    let layer_height = config.get_layer_height();
    let estimated_layers = if total_height <= first_layer_height {
        1
    } else {
        1 + ((total_height - first_layer_height) / layer_height).ceil() as usize
    };

    println!("{} {}", "→".cyan().bold(), "Slicing mesh...".bold());
    println!(
        "  {} Layer height: {:.3}mm (first layer: {:.3}mm)",
        "•".cyan(),
        layer_height,
        first_layer_height
    );
    println!(
        "  {} Estimated layers: {}",
        "•".cyan(),
        estimated_layers
    );
    println!();

    // Create progress bar
    let pb = ProgressBar::new(estimated_layers as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{bar:40.cyan/blue}] {pos}/{len} layers {msg}")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );

    // Slice the mesh
    let slicer = SliceEngine::new(mesh, config.clone());
    let layers = match slicer.slice() {
        Ok(l) => {
            pb.finish_and_clear();
            l
        }
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("  {} Slicing failed: {}", "✗".red(), e);
            return Err(e);
        }
    };

    // Report slicing results
    let total_islands: usize = layers.iter().map(|l| l.islands.len()).sum();
    let total_contours: usize = layers.iter().map(|l| l.contour_count()).sum();
    let non_empty_layers = layers.iter().filter(|l| !l.is_empty()).count();

    println!("  {} Generated {} layers", "✓".green(), layers.len());
    println!("  {} Non-empty layers: {}", "✓".green(), non_empty_layers);
    println!("  {} Total islands: {}", "✓".green(), total_islands);
    println!("  {} Total contours: {}", "✓".green(), total_contours);

    if verbose {
        println!("\n  {} Layer details:", "ℹ".blue());
        for (i, layer) in layers.iter().enumerate().take(5) {
            println!(
                "    Layer {}: z={:.3}mm, {} islands, {} contours",
                i,
                layer.z_height,
                layer.islands.len(),
                layer.contour_count()
            );
        }
        if layers.len() > 5 {
            println!("    ... ({} more layers)", layers.len() - 5);
        }
    }
    println!();

    // Generate G-code
    println!("{} {}", "→".cyan().bold(), "Generating G-code...".bold());
    let mut generator = GCodeGenerator::new(config);
    // Determine output file
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.to_path_buf();
        path.set_extension("gcode");
        Box::leak(Box::new(path))
    });

    // Generate and write G-code
    println!("{} {}", "→".cyan().bold(), "Generating G-code...".bold());
    match generator.generate(&layers, output_path) {
        Ok(_) => {
            let metadata = std::fs::metadata(output_path).unwrap();
            let size_kb = metadata.len() / 1024;
            println!("  {} Wrote {} to {}", "✓".green(), 
                     format!("{}KB", size_kb).cyan(), 
                     output_path.display());
        }
        Err(e) => {
            eprintln!("  {} G-code generation failed: {}", "✗".red(), e);
            return Err(e);
        }
    }


    println!();
    println!("{}", "═".repeat(60).cyan());
    println!("{}", "  Slicing complete!".green().bold());
    println!("{}", "═".repeat(60).cyan());

    Ok(())
}
