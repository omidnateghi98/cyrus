//! Run command implementation

use crate::core::{CyrusCore, Project};
use crate::languages;
use super::RunCommand;
use anyhow::{Context, Result};
use colored::*;
use std::env;

pub async fn execute(cmd: RunCommand, core: &CyrusCore) -> Result<()> {
    // Find project root
    let project_root = Project::find_project_root()
        .context("No cyrus.toml found. Run 'cyrus init' first.")?;
    
    env::set_current_dir(&project_root)?;
    
    // Load project configuration
    let project = Project::load_from_file(project_root.join("cyrus.toml"))?;
    
    // Get language handler
    let handler = languages::get_language_handler(&project.language)
        .context("Unsupported language in project")?;
    
    // Check if language is installed
    if !core.is_language_installed(&project.language, &project.version) {
        println!("{} {} {} is not installed. Run 'cyrus install {}{}'", 
                 "❌".red(),
                 project.language.yellow(),
                 project.version.yellow(),
                 project.language,
                 project.version);
        return Ok(());
    }
    
    println!("{} Running: {} {}", 
             "🚀".blue(), 
             cmd.command.yellow(), 
             cmd.args.join(" ").cyan());
    
    // Execute command with project environment
    handler.run_command(&cmd.command, &cmd.args).await
        .context("Failed to execute command")?;
    
    Ok(())
}
