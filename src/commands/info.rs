use anyhow::Result;
use crate::geometry::Mesh;

pub fn execute(input: &str) -> Result<()> {
    println!("ℹ️  STL File Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📁 File: {}", input);
    println!();

    let mesh = Mesh::from_stl_file(input)?;

    println!("🔢 Triangle count: {}", mesh.triangles.len());
    println!();

    let dims = mesh.bounds.dimensions();
    println!("📏 Dimensions:");
    println!("   Width (X):  {:.2} mm", dims.x);
    println!("   Depth (Y):  {:.2} mm", dims.y);
    println!("   Height (Z): {:.2} mm", dims.z);
    println!();

    println!("📐 Bounding box:");
    println!("   Min: ({:.2}, {:.2}, {:.2})", 
        mesh.bounds.min.x, mesh.bounds.min.y, mesh.bounds.min.z);
    println!("   Max: ({:.2}, {:.2}, {:.2})",
        mesh.bounds.max.x, mesh.bounds.max.y, mesh.bounds.max.z);
    println!();

    let volume_estimate = dims.x * dims.y * dims.z;
    println!("📦 Bounding volume: {:.2} mm³ ({:.2} cm³)", 
        volume_estimate, volume_estimate / 1000.0);

    Ok(())
}
