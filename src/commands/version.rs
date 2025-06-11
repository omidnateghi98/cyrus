//! Version command implementation

use crate::core::CyrusCore;
use super::VersionCommand;
use anyhow::Result;
use colored::*;

pub async fn execute(_cmd: VersionCommand, _core: &CyrusCore) -> Result<()> {
    println!("{}", "Cyrus - All-in-One Language Management Tool".cyan().bold());
    println!("Version: {}", "0.1.0".yellow());
    println!("Author: {}", "Omid Nateghi".green());
    println!("Engine: {}", "Omid Coder".blue());
    println!("Built with: {}", "Rust 🦀".red());
    
    Ok(())
}
