use crate::error::Result;

pub fn run(_details: bool) -> Result<()> {
    println!("Available profiles:");
    println!("  - default_pla");
    println!("\nProfile listing not yet fully implemented");
    Ok(())
}
