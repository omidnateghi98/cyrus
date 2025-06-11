//! Cyrus - All-in-One Language Management Tool (Enhanced)
//! 
//! Author: Omid Nateghi
//! Engine: Omid Coder
//! 
//! A comprehensive tool for managing programming language environments
//! with local project isolation, global language installation, and smart aliasing.

use clap::{Parser, Subcommand};
use std::process;

mod core;
mod commands;
mod config;
mod languages;
mod installer;
mod runtime;
mod utils;

use commands::*;
use core::CyrusCore;

#[derive(Parser)]
#[command(name = "cyrus")]
#[command(about = "All-in-One Language Management Tool with Smart Aliasing")]
#[command(version = "0.2.0")]
#[command(author = "Omid Nateghi")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a programming language
    Install(InstallCommand),
    /// Initialize a new project
    Init(InitCommand),
    /// Run commands in the project environment with smart aliasing
    Run(RunCommand),
    /// List installed languages and versions
    List(ListCommand),
    /// Update Cyrus or installed languages
    Update(UpdateCommand),
    /// Remove installed languages
    Remove(RemoveCommand),
    /// Show project or global configuration
    Config(ConfigCommand),
    /// Manage project aliases
    Alias(run::AliasCommand),
    /// Show supported languages
    Languages,
    /// Show version information
    Version(VersionCommand),
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let cli = Cli::parse();
    let core = CyrusCore::new().unwrap_or_else(|e| {
        eprintln!("Failed to initialize Cyrus: {}", e);
        process::exit(1);
    });

    let result = match cli.command {
        Commands::Install(cmd) => install::execute(cmd, &core).await,
        Commands::Init(cmd) => init::execute(cmd, &core).await,
        Commands::Run(cmd) => run::execute(cmd, &core).await,
        Commands::List(cmd) => list::execute(cmd, &core).await,
        Commands::Update(cmd) => update::execute(cmd, &core).await,
        Commands::Remove(cmd) => remove::execute(cmd, &core).await,
        Commands::Config(cmd) => config::execute(cmd, &core).await,
        Commands::Alias(cmd) => run::execute_alias(cmd, &core).await,
        Commands::Languages => languages_command(&core).await,
        Commands::Version(cmd) => version::execute(cmd, &core).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn languages_command(_core: &CyrusCore) -> anyhow::Result<()> {
    use colored::*;
    
    println!("{}", "🌐 Supported Languages:".cyan().bold());
    println!();
    
    let supported = languages::get_supported_languages();
    
    for language in supported {
        let display_name = languages::get_language_display_name(language);
        let aliases = languages::get_language_aliases(language);
        
        println!("{} {} {}", 
                 "🔧".blue(), 
                 display_name.yellow().bold(),
                 format!("({})", language).cyan());
        
        if !aliases.is_empty() {
            println!("   Aliases: {}", aliases.join(", ").green());
        }
        
        if let Some(handler) = languages::get_language_handler(language) {
            let config = handler.get_config();
            println!("   Versions: {}", config.versions.join(", ").blue());
            println!("   Package Managers: {}", config.package_managers.join(", ").magenta());
        }
        
        println!();
    }
    
    println!("{}", "💡 Tips:".yellow().bold());
    println!("• Use 'cyrus init' to create a new project");
    println!("• Use 'cyrus install <language><version>' to install a language");
    println!("• Use 'cyrus run <command>' for smart command aliasing");
    
    Ok(())
}