use anyhow::Result;
use crate::config::SlicerConfig;

pub fn execute(output: &str) -> Result<()> {
    println!("⚙️  Generating configuration template: {}", output);
    
    let config = SlicerConfig::default();
    config.save_to_file(output)?;

    println!("✅ Configuration file created successfully");
    println!("   You can now edit this file and use it with --config flag");

    Ok(())
}
