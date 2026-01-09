use std::path::Path;
use crate::error::Result;

pub fn run(input: &Path, _config: Option<&Path>, _verbose: bool) -> Result<()> {
    println!("Analyzing: {}", input.display());
    println!("Analysis not yet implemented");
    Ok(())
}
