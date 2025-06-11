//! Enhanced init command implementation with extended language support
//! src/commands/init.rs

use crate::core::{CyrusCore, Project};
use crate::languages;
use super::InitCommand;
use anyhow::{Context, Result};
use colored::*;
use dialoguer::{Input, Select, MultiSelect, Confirm};
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
    let supported_languages = languages::get_supported_languages();
    let language_display_names: Vec<String> = supported_languages
        .iter()
        .map(|lang| {
            let display = languages::get_language_display_name(lang);
            let aliases = languages::get_language_aliases(lang);
            if aliases.is_empty() {
                format!("{} ({})", display, lang)
            } else {
                format!("{} ({}, aliases: {})", display, lang, aliases.join(", "))
            }
        })
        .collect();
    
    let language = if let Some(lang) = cmd.language {
        if !languages::is_language_supported(&lang) {
            anyhow::bail!("Unsupported language: {}. Use 'cyrus languages' to see supported languages.", lang);
        }
        lang
    } else {
        let selection = Select::new()
            .with_prompt("Select programming language")
            .items(&language_display_names)
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
        if !config.versions.contains(&ver) {
            anyhow::bail!("Unsupported version {} for {}. Available versions: {}", 
                ver, language, config.versions.join(", "));
        }
        ver
    } else {
        let version_displays: Vec<String> = config.versions
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if v == &config.default_version {
                    format!("{} (default)", v)
                } else {
                    v.clone()
                }
            })
            .collect();
        
        let selection = Select::new()
            .with_prompt("Select version")
            .items(&version_displays)
            .default(config.versions.iter().position(|v| v == &config.default_version).unwrap_or(0))
            .interact()?;
        config.versions[selection].clone()
    };
    
    // Select package manager
    let package_manager = if let Some(pm) = cmd.package_manager {
        if !config.package_managers.contains(&pm) {
            anyhow::bail!("Unsupported package manager {} for {}. Available: {}", 
                pm, language, config.package_managers.join(", "));
        }
        pm
    } else if config.package_managers.len() == 1 {
        config.package_managers[0].clone()
    } else {
        let pm_displays: Vec<String> = config.package_managers
            .iter()
            .enumerate()
            .map(|(i, pm)| {
                if pm == &config.default_package_manager {
                    format!("{} (default)", pm)
                } else {
                    pm.clone()
                }
            })
            .collect();
        
        let selection = Select::new()
            .with_prompt("Select package manager")
            .items(&pm_displays)
            .default(config.package_managers.iter().position(|pm| pm == &config.default_package_manager).unwrap_or(0))
            .interact()?;
        config.package_managers[selection].clone()
    };
    
    // Ask about enabling aliases
    let enable_aliases = Confirm::new()
        .with_prompt("Enable smart command aliasing? (Recommended)")
        .default(true)
        .interact()?;
    
    // Ask about additional dependencies
    let add_dependencies = Confirm::new()
        .with_prompt("Add initial dependencies?")
        .default(false)
        .interact()?;
    
    let mut dependencies = Vec::new();
    if add_dependencies {
        println!("{}", "Enter dependencies (one per line, empty line to finish):".yellow());
        loop {
            let dep: String = Input::new()
                .with_prompt("Dependency")
                .allow_empty(true)
                .interact_text()?;
            
            if dep.trim().is_empty() {
                break;
            }
            dependencies.push(dep.trim().to_string());
        }
    }
    
    // Create project configuration
    let mut project = Project::new(
        project_name.clone(),
        language.clone(),
        version.clone(),
        package_manager.clone(),
    );
    
    // Set aliases preference
    project.enable_aliases = enable_aliases;
    
    // Add dependencies
    project.dependencies = dependencies;
    
    // Save project configuration
    let current_dir = env::current_dir()?;
    let config_path = current_dir.join("cyrus.toml");
    
    project.save_to_file(&config_path)
        .context("Failed to save project configuration")?;
    
    // Setup language environment
    handler.setup_environment(&current_dir).await
        .context("Failed to setup language environment")?;
    
    // Show summary
    println!("\n{}", "📋 Project Summary:".green().bold());
    println!("  Name: {}", project_name.yellow());
    println!("  Language: {} {}", 
             languages::get_language_display_name(&language).blue(), 
             version.cyan());
    println!("  Package Manager: {}", package_manager.magenta());
    println!("  Smart Aliases: {}", 
             if enable_aliases { "✅ Enabled".green() } else { "❌ Disabled".red() });
    
    if !project.dependencies.is_empty() {
        println!("  Dependencies: {}", project.dependencies.join(", ").cyan());
    }
    
    println!("\n{} Project '{}' initialized successfully!", 
             "✅".green(), 
             project_name.yellow());
    println!("Configuration saved to: {}", config_path.display().to_string().blue());
    
    // Show next steps
    println!("\n{}", "🚀 Next Steps:".yellow().bold());
    println!("  • Install the language: {}", format!("cyrus install {}{}", language, version).cyan());
    println!("  • Run commands: {}", "cyrus run <command>".cyan());
    println!("  • View aliases: {}", "cyrus alias list".cyan());
    
    Ok(())
}
