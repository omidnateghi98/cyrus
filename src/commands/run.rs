//! Enhanced run command implementation with alias support
//! src/commands/run.rs

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

    // Resolve command through project aliases and package manager integration
    let (resolved_command, resolved_args) = project.resolve_command(&cmd.command, &cmd.args);
    
    // Show what we're actually executing
    if resolved_command != cmd.command || resolved_args != cmd.args {
        println!("{} Aliased: {} {} → {} {}", 
                 "🔗".blue(),
                 cmd.command.yellow(),
                 cmd.args.join(" ").cyan(),
                 resolved_command.yellow(),
                 resolved_args.join(" ").cyan());
    }
    
    println!("{} Running: {} {}", 
             "🚀".blue(), 
             resolved_command.yellow(), 
             resolved_args.join(" ").cyan());
    
    // Execute command with project environment
    handler.run_command(&resolved_command, &resolved_args).await
        .context("Failed to execute command")?;
    
    Ok(())
}

// Add a new command for managing aliases
#[derive(clap::Args)]
pub struct AliasCommand {
    #[command(subcommand)]
    pub action: AliasAction,
}

#[derive(clap::Subcommand)]
pub enum AliasAction {
    /// List all aliases
    List,
    /// Add a new alias
    Add {
        /// Alias name
        alias: String,
        /// Command to alias to
        command: String,
    },
    /// Remove an alias
    Remove {
        /// Alias name to remove
        alias: String,
    },
    /// Toggle alias functionality
    Toggle,
}

pub async fn execute_alias(cmd: AliasCommand, _core: &CyrusCore) -> Result<()> {
    // Find project root
    let project_root = Project::find_project_root()
        .context("No cyrus.toml found. Run 'cyrus init' first.")?;
    
    let config_path = project_root.join("cyrus.toml");
    let mut project = Project::load_from_file(&config_path)?;
    
    match cmd.action {
        AliasAction::List => {
            println!("{}", "📋 Project Aliases:".cyan().bold());
            
            if !project.enable_aliases {
                println!("{}", "❌ Aliases are currently disabled".yellow());
                return Ok(());
            }
            
            println!("\n{}", "Custom Aliases:".green().bold());
            if project.custom_aliases.is_empty() {
                println!("  {}", "No custom aliases defined".yellow());
            } else {
                for (alias, command) in &project.custom_aliases {
                    println!("  {} → {}", alias.blue(), command.cyan());
                }
            }
            
            println!("\n{}", "Script Aliases:".green().bold());
            if project.scripts.is_empty() {
                println!("  {}", "No scripts defined".yellow());
            } else {
                for (script, command) in &project.scripts {
                    println!("  {} → {}", script.blue(), command.cyan());
                }
            }
        },
        
        AliasAction::Add { alias, command } => {
            project.add_alias(alias.clone(), command.clone());
            project.save_to_file(&config_path)?;
            
            println!("{} Added alias: {} → {}", 
                     "✅".green(), 
                     alias.blue(), 
                     command.cyan());
        },
        
        AliasAction::Remove { alias } => {
            if project.custom_aliases.contains_key(&alias) {
                project.remove_alias(&alias);
                project.save_to_file(&config_path)?;
                
                println!("{} Removed alias: {}", 
                         "✅".green(), 
                         alias.blue());
            } else {
                println!("{} Alias not found: {}", 
                         "❌".red(), 
                         alias.blue());
            }
        },
        
        AliasAction::Toggle => {
            project.toggle_aliases();
            project.save_to_file(&config_path)?;
            
            let status = if project.enable_aliases { "enabled" } else { "disabled" };
            println!("{} Aliases are now {}", 
                     "🔄".blue(), 
                     status.yellow());
        },
    }
    
    Ok(())
}