//! Command implementations for Cyrus CLI

pub mod install;
pub mod init;
pub mod run;
pub mod list;
pub mod update;
pub mod remove;
pub mod config;
pub mod version;

use clap::Args;

#[derive(Args)]
pub struct InstallCommand {
    /// Language and version to install (e.g., python3.11)
    pub language_version: String,
    
    /// Package manager to use
    #[arg(short, long)]
    pub package_manager: Option<String>,
    
    /// Use default configuration without prompts
    #[arg(short, long)]
    pub default: bool,
}

#[derive(Args)]
pub struct InitCommand {
    /// Project name
    pub name: Option<String>,
    
    /// Language to use
    #[arg(short, long)]
    pub language: Option<String>,
    
    /// Language version
    #[arg(short, long)]
    pub version: Option<String>,
    
    /// Package manager
    #[arg(short, long)]
    pub package_manager: Option<String>,
}

#[derive(Args)]
pub struct RunCommand {
    /// Command to run
    pub command: String,
    
    /// Arguments for the command
    pub args: Vec<String>,
}

#[derive(Args)]
pub struct ListCommand {
    /// Show only installed languages
    #[arg(short, long)]
    pub installed: bool,
}

#[derive(Args)]
pub struct UpdateCommand {
    /// Update specific language
    pub language: Option<String>,
}

#[derive(Args)]
pub struct RemoveCommand {
    /// Language and version to remove
    pub language_version: String,
}

#[derive(Args)]
pub struct ConfigCommand {
    /// Show global configuration
    #[arg(short, long)]
    pub global: bool,
}

#[derive(Args)]
pub struct VersionCommand {}
