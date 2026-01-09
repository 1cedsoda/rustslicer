use anyhow::Result;
use crate::geometry::Mesh;

pub fn execute(input: &str) -> Result<()> {
    println!("🔍 Validating STL file: {}", input);
    println!();

    let mesh = Mesh::from_stl_file(input)?;
    mesh.validate()?;

    println!("✅ STL file is valid");
    println!("   Triangles: {}", mesh.triangles.len());
    
    let dims = mesh.bounds.dimensions();
    println!("   Dimensions: {:.2} x {:.2} x {:.2} mm", dims.x, dims.y, dims.z);

    Ok(())
}
