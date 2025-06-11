//! Config command implementation

use crate::core::{CyrusCore, Project};
use super::ConfigCommand;
use anyhow::Result;
use colored::*;

pub async fn execute(cmd: ConfigCommand, core: &CyrusCore) -> Result<()> {
    if cmd.global {
        println!("{}", "🌐 Global Configuration:".cyan().bold());
        println!("Cyrus Directory: {}", core.cyrus_dir.display().to_string().blue());
        println!("Config Directory: {}", core.config_dir.display().to_string().blue());
        println!("Languages Directory: {}", core.languages_dir.display().to_string().blue());
    } else {
        // Show project configuration
        if let Some(project_root) = Project::find_project_root() {
            let project = Project::load_from_file(project_root.join("cyrus.toml"))?;
            
            println!("{}", "📁 Project Configuration:".cyan().bold());
            println!("Name: {}", project.name.yellow());
            println!("Language: {}", project.language.yellow());
            println!("Version: {}", project.version.yellow());
            println!("Package Manager: {}", project.package_manager.yellow());
            
            if !project.dependencies.is_empty() {
                println!("Dependencies: {}", project.dependencies.join(", ").cyan());
            }
            
            if !project.scripts.is_empty() {
                println!("Scripts:");
                for (name, command) in &project.scripts {
                    println!("  {}: {}", name.green(), command.cyan());
                }
            }
        } else {
            println!("{}", "No project configuration found. Run 'cyrus init' first.".yellow());
        }
    }
    
    Ok(())
}
