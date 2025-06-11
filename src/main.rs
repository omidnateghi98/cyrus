//! Cyrus - All-in-One Language Management Tool
//! 
//! Author: Omid Nateghi
//! Engine: Omid Coder
//! 
//! A comprehensive tool for managing programming language environments
//! with local project isolation and global language installation.

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
#[command(about = "All-in-One Language Management Tool")]
#[command(version = "0.1.0")]
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
    /// Run commands in the project environment
    Run(RunCommand),
    /// List installed languages and versions
    List(ListCommand),
    /// Update Cyrus or installed languages
    Update(UpdateCommand),
    /// Remove installed languages
    Remove(RemoveCommand),
    /// Show project or global configuration
    Config(ConfigCommand),
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
        Commands::Version(cmd) => version::execute(cmd, &core).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
