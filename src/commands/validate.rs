use std::path::Path;
use crate::error::Result;

pub fn run(input: &Path, _fix: bool, _verbose: bool) -> Result<()> {
    println!("Validating: {}", input.display());
    println!("Validation not yet implemented");
    Ok(())
}
