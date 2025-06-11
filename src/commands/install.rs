//! Install command implementation

use crate::core::CyrusCore;
use crate::languages;
use super::InstallCommand;
use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Select, Confirm};

pub async fn execute(cmd: InstallCommand, core: &CyrusCore) -> Result<()> {
    println!("{}", "🚀 Installing language...".cyan().bold());
    
    // Parse language and version
    let (language, version) = parse_language_version(&cmd.language_version)?;
    
    // Check if already installed
    if core.is_language_installed(&language, &version) {
        println!("{} {} {} is already installed!", 
                "✅".green(), 
                language.yellow(), 
                version.yellow());
        return Ok(());
    }
    
    // Get language handler
    let handler = languages::get_language_handler(&language)
        .context("Unsupported language")?;
    
    let config = handler.get_config();
    
    // Select package manager
    let package_manager = if let Some(pm) = cmd.package_manager {
        pm
    } else if cmd.default {
        config.default_package_manager.clone()
    } else {
        let selection = Select::new()
            .with_prompt("Select package manager")
            .items(&config.package_managers)
            .default(0)
            .interact()?;
        config.package_managers[selection].clone()
    };
    
    // Confirm installation
    if !cmd.default {
        let install = Confirm::new()
            .with_prompt(format!("Install {} {} with {}?", language, version, package_manager))
            .interact()?;
        
        if !install {
            println!("{}", "Installation cancelled.".yellow());
            return Ok(());
        }
    }
    
    // Install the language
    let install_path = core.language_path(&language, &version);
    
    println!("{} Installing {} {} to {:?}...", 
             "📦".blue(), 
             language.yellow(), 
             version.yellow(), 
             install_path);
    
    handler.install(&version, &install_path).await
        .context("Failed to install language")?;
    
    println!("{} {} {} installed successfully!", 
             "✅".green(), 
             language.yellow(), 
             version.yellow());
    
    Ok(())
}

fn parse_language_version(input: &str) -> Result<(String, String)> {
    // Handle formats like "python3.11", "node18", "go1.21"
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
