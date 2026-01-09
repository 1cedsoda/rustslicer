use std::path::Path;
use crate::error::Result;

pub fn run(input: &Path, _output_dir: &Path, _layers: Option<&str>, _config: &Path, _verbose: bool) -> Result<()> {
    println!("Previewing: {}", input.display());
    println!("Preview not yet implemented");
    Ok(())
}
