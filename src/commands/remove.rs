//! Remove command implementation

use crate::core::CyrusCore;
use super::RemoveCommand;
use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;
use std::fs;

pub async fn execute(cmd: RemoveCommand, core: &CyrusCore) -> Result<()> {
    // Parse language and version
    let (language, version) = parse_language_version(&cmd.language_version)?;
    
    // Check if installed
    if !core.is_language_installed(&language, &version) {
        println!("{} {} {} is not installed.", 
                 "❌".red(), 
                 language.yellow(), 
                 version.yellow());
        return Ok(());
    }
    
    // Confirm removal
    let confirm = Confirm::new()
        .with_prompt(format!("Remove {} {}?", language, version))
        .interact()?;
    
    if !confirm {
        println!("{}", "Removal cancelled.".yellow());
        return Ok(());
    }
    
    // Remove the installation
    let install_path = core.language_path(&language, &version);
    fs::remove_dir_all(&install_path)
        .context("Failed to remove language installation")?;
    
    println!("{} {} {} removed successfully!", 
             "✅".green(), 
             language.yellow(), 
             version.yellow());
    
    Ok(())
}

fn parse_language_version(input: &str) -> Result<(String, String)> {
    if let Some(captures) = regex::Regex::new(r"^([a-zA-Z]+)(.+)$")
        .unwrap()
        .captures(input) {
        let language = captures.get(1).unwrap().as_str().to_lowercase();
        let version = captures.get(2).unwrap().as_str();
        Ok((language, version.to_string()))
    } else {
        anyhow::bail!("Invalid language version format: {}", input);
    }
}
