//! List command implementation

use crate::core::CyrusCore;
use crate::languages;
use super::ListCommand;
use anyhow::Result;
use colored::*;
use std::fs;

pub async fn execute(cmd: ListCommand, core: &CyrusCore) -> Result<()> {
    println!("{}", "📋 Installed Languages:".cyan().bold());
    
    if !core.languages_dir.exists() {
        println!("{}", "No languages installed yet.".yellow());
        return Ok(());
    }
    
    let mut found_any = false;
    
    // Iterate through language directories
    for entry in fs::read_dir(&core.languages_dir)? {
        let entry = entry?;
        let language_name = entry.file_name().to_string_lossy().to_string();
        
        if entry.file_type()?.is_dir() {
            println!("\n{} {}:", "🔧".blue(), language_name.yellow().bold());
            
            // List versions for this language
            for version_entry in fs::read_dir(entry.path())? {
                let version_entry = version_entry?;
                if version_entry.file_type()?.is_dir() {
                    let version = version_entry.file_name().to_string_lossy();
                    println!("  {} {}", "📦".green(), version.cyan());
                    found_any = true;
                }
            }
        }
    }
    
    if !found_any {
        println!("{}", "No languages installed yet.".yellow());
    }
    
    Ok(())
}
