//! Init command implementation

use crate::core::{CyrusCore, Project};
use crate::languages;
use super::InitCommand;
use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Select};
use std::env;

pub async fn execute(cmd: InitCommand, core: &CyrusCore) -> Result<()> {
    println!("{}", "🎯 Initializing new project...".cyan().bold());
    
    // Get project name
    let project_name = if let Some(name) = cmd.name {
        name
    } else {
        let current_dir = env::current_dir()?;
        let default_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string();
        
        Input::new()
            .with_prompt("Project name")
            .default(default_name)
            .interact_text()?
    };
    
    // Select language
    let supported_languages = vec!["python", "javascript", "golang"];
    let language = if let Some(lang) = cmd.language {
        lang
    } else {
        let selection = Select::new()
            .with_prompt("Select language")
            .items(&supported_languages)
            .default(0)
            .interact()?;
        supported_languages[selection].to_string()
    };
    
    // Get language handler
    let handler = languages::get_language_handler(&language)
        .context("Unsupported language")?;
    
    let config = handler.get_config();
    
    // Select version
    let version = if let Some(ver) = cmd.version {
        ver
    } else {
        let selection = Select::new()
            .with_prompt("Select version")
            .items(&config.versions)
            .default(0)
            .interact()?;
        config.versions[selection].clone()
    };
    
    // Select package manager
    let package_manager = if let Some(pm) = cmd.package_manager {
        pm
    } else {
        let selection = Select::new()
            .with_prompt("Select package manager")
            .items(&config.package_managers)
            .default(0)
            .interact()?;
        config.package_managers[selection].clone()
    };
    
    // Create project configuration
    let project = Project::new(
        project_name.clone(),
        language.clone(),
        version.clone(),
        package_manager.clone(),
    );
    
    // Save project configuration
    let current_dir = env::current_dir()?;
    let config_path = current_dir.join("cyrus.toml");
    
    project.save_to_file(&config_path)
        .context("Failed to save project configuration")?;
    
    // Setup language environment
    handler.setup_environment(&current_dir).await
        .context("Failed to setup language environment")?;
    
    println!("{} Project '{}' initialized successfully!", 
             "✅".green(), 
             project_name.yellow());
    println!("Configuration saved to: {}", config_path.display().to_string().blue());
    
    Ok(())
}
