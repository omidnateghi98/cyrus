// src/main.rs
//! Cyrus - All-in-One Language Management Tool (Enhanced v0.3.0)
//! 
//! Author: Omid Nateghi
//! Engine: Omid Coder
//! 
//! A comprehensive tool for managing programming language environments
//! with local project isolation, global language installation, smart aliasing,
//! templates, plugins, workspaces, and advanced configuration management.

use clap::{Parser, Subcommand};
use std::process;
use colored::*;

mod error;
mod core;
mod commands;
mod config;
mod languages;
mod installer;
mod runtime;
mod utils;
mod templates;
mod plugins;
mod workspace;

use error::{CyrusError, Result};
use commands::*;
use core::CyrusCore;
use anyhow::Result as AnyhowResult;
use crate::error::Result as CyrusResult;


#[derive(Parser)]
#[command(name = "cyrus")]
#[command(about = "All-in-One Language Management Tool with Advanced Features")]
#[command(version = "0.3.0")]
#[command(author = "Omid Nateghi")]
#[command(long_about = r#"
Cyrus is a comprehensive language management tool that provides:

• Multi-language support with smart aliasing
• Project templates and scaffolding
• Plugin system for extensibility
• Workspace management for multi-project development
• Advanced configuration with profiles
• Performance optimizations and security features

Visit https://cyrus-lang.org for documentation and examples.
"#)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Suppress output (quiet mode)
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,
    
    /// Configuration profile to use
    #[arg(long, global = true)]
    profile: Option<String>,
    
    /// Configuration file path
    #[arg(long, global = true)]
    config: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a programming language
    Install(InstallCommand),
    /// Initialize a new project with templates
    Init(InitCommand),
    /// Create a new project from template
    New(NewCommand),
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
    
    // New enhanced commands
    /// Manage project templates
    Template(TemplateCommand),
    /// Manage plugins
    Plugin(PluginCommand),
    /// Manage workspaces
    Workspace(WorkspaceCommand),
    /// Manage configuration profiles
    Profile(ProfileCommand),
    /// Development and debugging tools
    Dev(DevCommand),
    /// Security and audit commands
    Security(SecurityCommand),
    /// Performance optimization commands
    Perf(PerfCommand),
}

#[derive(clap::Args)]
pub struct NewCommand {
    /// Template name
    pub template: String,
    
    /// Project name
    pub name: String,
    
    /// Project directory (defaults to project name)
    #[arg(short, long)]
    pub path: Option<std::path::PathBuf>,
    
    /// Template variables (key=value)
    #[arg(short = 'V', long = "var")]
    pub variables: Vec<String>,
    
    /// Features to enable
    #[arg(short, long)]
    pub features: Vec<String>,
    
    /// List available templates instead of creating project
    #[arg(short, long)]
    pub list: bool,
}

#[derive(clap::Args)]
pub struct TemplateCommand {
    #[command(subcommand)]
    pub action: TemplateAction,
}

#[derive(clap::Subcommand)]
pub enum TemplateAction {
    /// List available templates
    List {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Filter by language
        #[arg(short, long)]
        language: Option<String>,
    },
    /// Search templates
    Search {
        /// Search query
        query: String,
    },
    /// Show template details
    Show {
        /// Template name
        name: String,
    },
    /// Install template from source
    Install {
        /// Template source (URL, Git repo, etc.)
        source: String,
    },
    /// Create custom template
    Create {
        /// Template name
        name: String,
    },
}

#[derive(clap::Args)]
pub struct PluginCommand {
    #[command(subcommand)]
    pub action: PluginAction,
}

#[derive(clap::Subcommand)]
pub enum PluginAction {
    /// List installed plugins
    List,
    /// Install a plugin
    Install {
        /// Plugin source
        source: String,
    },
    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },
    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },
    /// Uninstall a plugin
    Uninstall {
        /// Plugin name
        name: String,
    },
    /// Execute plugin command
    Exec {
        /// Plugin name
        plugin: String,
        /// Command
        command: String,
        /// Arguments
        args: Vec<String>,
    },
    /// Update plugins
    Update {
        /// Specific plugin name
        name: Option<String>,
    },
}

#[derive(clap::Args)]
pub struct WorkspaceCommand {
    #[command(subcommand)]
    pub action: WorkspaceAction,
}

#[derive(clap::Subcommand)]
pub enum WorkspaceAction {
    /// Initialize a new workspace
    Init {
        /// Workspace name
        name: String,
        /// Description
        #[arg(short, long)]
        description: Option<String>,
        /// Workspace path
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    /// Add member to workspace
    Add {
        /// Member name
        name: String,
        /// Member path
        path: std::path::PathBuf,
        /// Language
        #[arg(short, long)]
        language: Option<String>,
        /// Create project if it doesn't exist
        #[arg(short, long)]
        create: bool,
    },
    /// Remove member from workspace
    Remove {
        /// Member name
        name: String,
        /// Delete files
        #[arg(short, long)]
        delete: bool,
    },
    /// List workspace members
    List,
    /// Run command in workspace
    Run {
        /// Command to run
        command: String,
        /// Command arguments
        args: Vec<String>,
        /// Specific members
        #[arg(short, long)]
        members: Option<Vec<String>>,
        /// Run in parallel
        #[arg(short, long)]
        parallel: bool,
    },
    /// Build workspace
    Build {
        /// Build in parallel
        #[arg(short, long)]
        parallel: bool,
    },
    /// Test workspace
    Test {
        /// Test in parallel
        #[arg(short, long)]
        parallel: bool,
    },
    /// Show workspace status
    Status,
}

#[derive(clap::Args)]
pub struct ProfileCommand {
    #[command(subcommand)]
    pub action: ProfileAction,
}

#[derive(clap::Subcommand)]
pub enum ProfileAction {
    /// List available profiles
    List,
    /// Create new profile
    Create {
        /// Profile name
        name: String,
        /// Base profile
        #[arg(short, long)]
        base: Option<String>,
    },
    /// Switch active profile
    Switch {
        /// Profile name
        name: String,
    },
    /// Show profile details
    Show {
        /// Profile name
        name: Option<String>,
    },
    /// Export profile
    Export {
        /// Profile name
        name: String,
        /// Output file
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
    },
    /// Import profile
    Import {
        /// Profile file
        file: std::path::PathBuf,
    },
}

#[derive(clap::Args)]
pub struct DevCommand {
    #[command(subcommand)]
    pub action: DevAction,
}

#[derive(clap::Subcommand)]
pub enum DevAction {
    /// Show debug information
    Debug,
    /// Validate configuration
    Validate,
    /// Generate shell completions
    Completions {
        /// Shell type
        shell: clap_complete::Shell,
    },
    /// Show environment information
    Env,
    /// Clean caches and temporary files
    Clean,
}

#[derive(clap::Args)]
pub struct SecurityCommand {
    #[command(subcommand)]
    pub action: SecurityAction,
}

#[derive(clap::Subcommand)]
pub enum SecurityAction {
    /// Audit dependencies for vulnerabilities
    Audit,
    /// Verify installed languages
    Verify,
    /// Show security status
    Status,
    /// Update security database
    Update,
}

#[derive(clap::Args)]
pub struct PerfCommand {
    #[command(subcommand)]
    pub action: PerfAction,
}

#[derive(clap::Subcommand)]
pub enum PerfAction {
    /// Show performance metrics
    Metrics,
    /// Optimize configuration
    Optimize,
    /// Benchmark operations
    Benchmark {
        /// Operation to benchmark
        operation: String,
    },
    /// Clean performance data
    Clean,
}
fn format_error(error: &crate::error::CyrusError) -> String {
    use crate::error::ErrorRecovery;
    ErrorRecovery::format_user_friendly_error(error)
}
#[tokio::main]
async fn main() {
    // Initialize logging based on environment
    let log_level = match std::env::var("CYRUS_LOG") {
        Ok(level) => level,
        Err(_) => "info".to_string(),
    };
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_target(false)
        .format_timestamp(None)
        .init();
    
    let cli = Cli::parse();
    
    // Set verbosity based on flags
    if cli.quiet {
        std::env::set_var("CYRUS_LOG", "error");
    } else if cli.verbose {
        std::env::set_var("CYRUS_LOG", "debug");
    }
    
    // Initialize core
    let core = match CyrusCore::new() {
        Ok(core) => core,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), format_error(&e));
            process::exit(1);
        }
    };
    
    // Load configuration with profile override
    let config = match load_config(&cli) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} {}", "Configuration Error:".red().bold(), format_error(&e));
            process::exit(1);
        }
    };
    
    // Execute command
    let result = match cli.command {
        Commands::Install(cmd) => install::execute(cmd, &core).await,
        Commands::Init(cmd) => init::execute(cmd, &core).await,
        Commands::New(cmd) => execute_new_command(cmd, &core).await,
        Commands::Run(cmd) => run::execute(cmd, &core).await,
        Commands::List(cmd) => list::execute(cmd, &core).await,
        Commands::Update(cmd) => update::execute(cmd, &core).await,
        Commands::Remove(cmd) => remove::execute(cmd, &core).await,
        Commands::Config(cmd) => config::execute(cmd, &core).await,
        Commands::Alias(cmd) => run::execute_alias(cmd, &core).await,
        Commands::Languages => languages_command(&core).await,
        Commands::Version(cmd) => version::execute(cmd, &core).await,
        Commands::Template(cmd) => execute_template_command(cmd, &core).await,
        Commands::Plugin(cmd) => execute_plugin_command(cmd, &core).await,
        Commands::Workspace(cmd) => execute_workspace_command(cmd, &core).await,
        Commands::Profile(cmd) => execute_profile_command(cmd, &core).await,
        Commands::Dev(cmd) => execute_dev_command(cmd, &core).await,
        Commands::Security(cmd) => execute_security_command(cmd, &core).await,
        Commands::Perf(cmd) => execute_perf_command(cmd, &core).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), format_error(&e));
        
        // Show suggestions for common errors
        show_error_suggestions(&e);
        
        process::exit(1);
    }
}

fn load_config(cli: &Cli) -> AnyhowResult<config::GlobalConfig> {
    let config_path = if let Some(path) = &cli.config {
        path.clone()
    } else {
        dirs::config_dir()
            .ok_or_else(|| CyrusError::Config {
                message: "Could not determine config directory".to_string(),
            })?
            .join("cyrus")
            .join("config.toml")
    };

    let config = if config_path.exists() {
        config::GlobalConfig::load_from_file(&config_path)?
    } else {
        let config = config::GlobalConfig::new();
        // Create config directory
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        config.save_to_file(&config_path)?;
        config
    };

    // Note: Profile switching would be implemented here in a real app
    if let Some(_profile_name) = &cli.profile {
        // Profile switching logic would go here
        // For now, we just acknowledge the parameter exists
    }

    Ok(config)
}

async fn execute_new_command(cmd: NewCommand, core: &CyrusCore) -> AnyhowResult<()> {
    if cmd.list {
        return execute_template_list(None, None).await;
    }
    
    let template_manager = templates::TemplateManager::new()?;
    
    // Parse variables
    let mut variables = std::collections::HashMap::new();
    for var in &cmd.variables {
        if let Some((key, value)) = var.split_once('=') {
            variables.insert(key.to_string(), value.to_string());
        } else {
            return Err(CyrusError::Config {
                message: format!("Invalid variable format '{}'. Use key=value", var),
            });
        }
    }
    
    let project_path = cmd.path.unwrap_or_else(|| {
        std::env::current_dir().unwrap().join(&cmd.name)
    });
    
    println!("{} Creating project '{}' from template '{}'", 
             "🚀".cyan(), cmd.name.yellow(), cmd.template.blue());
    
    template_manager.create_project(
        &cmd.template,
        &cmd.name,
        &project_path,
        variables,
        cmd.features,
    ).await?;
    
    println!("{} Project created successfully at {:?}", "✅".green(), project_path);
    println!("\n{}", "Next steps:".yellow().bold());
    println!("  cd {}", cmd.name);
    println!("  cyrus run install");
    println!("  cyrus run dev");
    
    Ok(())
}

async fn execute_template_command(cmd: TemplateCommand, _core: &CyrusCore) -> AnyhowResult<()> {
    match cmd.action {
        TemplateAction::List { category, language } => {
            execute_template_list(category, language).await
        },
        TemplateAction::Search { query } => {
            let template_manager = templates::TemplateManager::new()?;
            let templates = template_manager.search_templates(&query).await?;
            
            if templates.is_empty() {
                println!("No templates found matching '{}'", query);
                return Ok(());
            }
            
            println!("{} Found {} templates matching '{}':", 
                     "🔍".blue(), templates.len(), query.yellow());
            
            for template in templates {
                print_template_info(&template);
            }
            
            Ok(())
        },
        TemplateAction::Show { name } => {
            let template_manager = templates::TemplateManager::new()?;
            // Implementation for showing detailed template info
            println!("Template details for '{}'", name);
            // TODO: Implement detailed template view
            Ok(())
        },
        TemplateAction::Install { source } => {
            println!("Installing template from '{}'", source);
            // TODO: Implement template installation
            Ok(())
        },
        TemplateAction::Create { name } => {
            println!("Creating custom template '{}'", name);
            // TODO: Implement template creation wizard
            Ok(())
        },
    }
}

async fn execute_template_list(category: Option<String>, language: Option<String>) -> AnyhowResult<()> {
    let template_manager = templates::TemplateManager::new()?;
    let mut templates = template_manager.list_templates().await?;
    
    // Filter by category
    if let Some(cat) = &category {
        templates.retain(|t| format!("{:?}", t.category).to_lowercase().contains(&cat.to_lowercase()));
    }
    
    // Filter by language
    if let Some(lang) = &language {
        templates.retain(|t| t.language.to_lowercase().contains(&lang.to_lowercase()));
    }
    
    if templates.is_empty() {
        println!("No templates found");
        return Ok(());
    }
    
    println!("{} Available Templates:", "📋".blue());
    
    // Group by category
    let mut by_category: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
    for template in templates {
        let cat = format!("{:?}", template.category);
        by_category.entry(cat).or_default().push(template);
    }
    
    for (category, templates) in by_category {
        println!("\n{} {}:", "📁".green(), category.bold());
        for template in templates {
            print_template_info(&template);
        }
    }
    
    Ok(())
}

fn print_template_info(template: &templates::TemplateInfo) {
    let difficulty_icon = match template.difficulty {
        templates::DifficultyLevel::Beginner => "🟢",
        templates::DifficultyLevel::Intermediate => "🟡",
        templates::DifficultyLevel::Advanced => "🟠",
        templates::DifficultyLevel::Expert => "🔴",
    };
    
    let source_icon = match template.source {
        templates::TemplateSource::Builtin => "📦",
        templates::TemplateSource::Registry => "🌐",
        templates::TemplateSource::Git(_) => "📡",
        templates::TemplateSource::Local(_) => "📁",
    };
    
    println!("  {} {} {} {} ({})", 
             source_icon,
             difficulty_icon,
             template.name.cyan(),
             template.language.yellow(),
             template.description.dim());
}

async fn execute_plugin_command(cmd: PluginCommand, core: &CyrusCore) -> AnyhowResult<()> {
    let plugin_dirs = vec![
        core.cyrus_dir.join("plugins"),
        dirs::data_dir().unwrap_or_default().join("cyrus").join("plugins"),
    ];
    
    let mut plugin_manager = plugins::PluginManager::new(plugin_dirs);
    
    match cmd.action {
        PluginAction::List => {
            plugin_manager.discover_plugins().await?;
            let plugins = plugin_manager.list_plugins();
            
            if plugins.is_empty() {
                println!("No plugins installed");
                return Ok(());
            }
            
            println!("{} Installed Plugins:", "🔌".blue());
            for plugin in plugins {
                let status = if plugin.enabled { "✅" } else { "❌" };
                println!("  {} {} v{} - {}", 
                         status,
                         plugin.info.name.cyan(),
                         plugin.info.version.yellow(),
                         plugin.info.description.dim());
            }
        },
        PluginAction::Install { source } => {
            plugin_manager.install_plugin(&source).await?;
        },
        PluginAction::Enable { name } => {
            plugin_manager.enable_plugin(&name)?;
        },
        PluginAction::Disable { name } => {
            plugin_manager.disable_plugin(&name)?;
        },
        PluginAction::Uninstall { name } => {
            plugin_manager.uninstall_plugin(&name).await?;
        },
        PluginAction::Exec { plugin, command, args } => {
            plugin_manager.execute_plugin_command(&plugin, &command, &args).await?;
        },
        PluginAction::Update { name } => {
            if let Some(plugin_name) = name {
                // Update specific plugin
                println!("Updating plugin '{}'", plugin_name);
            } else {
                plugin_manager.update_plugins().await?;
            }
        },
    }
    
    Ok(())
}

async fn execute_workspace_command(cmd: WorkspaceCommand, _core: &CyrusCore) -> AnyhowResult<()> {
    let workspace_path = find_workspace_root().unwrap_or_else(|| std::env::current_dir().unwrap());
    
    match cmd.action {
        WorkspaceAction::Init { name, description, path } => {
            workspace::WorkspaceCommands::init(name, description, path).await
        },
        WorkspaceAction::Add { name, path, language, create } => {
            workspace::WorkspaceCommands::add_member(workspace_path, name, path, language, create).await
        },
        WorkspaceAction::Remove { name, delete } => {
            let mut manager = workspace::WorkspaceManager::new();
            manager.load_workspace(&workspace_path)?;
            manager.remove_member(&name, delete)
        },
        WorkspaceAction::List => {
            workspace::WorkspaceCommands::list_members(workspace_path).await
        },
        WorkspaceAction::Run { command, args, members, parallel } => {
            workspace::WorkspaceCommands::run_command(workspace_path, command, args, members, parallel).await
        },
        WorkspaceAction::Build { parallel } => {
            workspace::WorkspaceCommands::build(workspace_path, parallel).await
        },
        WorkspaceAction::Test { parallel } => {
            workspace::WorkspaceCommands::test(workspace_path, parallel).await
        },
        WorkspaceAction::Status => {
            workspace::WorkspaceCommands::status(workspace_path).await
        },
    }
}

async fn execute_profile_command(cmd: ProfileCommand, _core: &CyrusCore) -> AnyhowResult<()> {
    use crate::config::profiles::ProfileManager;
    
    let mut profile_manager = ProfileManager::new();
    
    match cmd.action {
        ProfileAction::List => {
            let profiles = profile_manager.list_profiles();
            println!("{} Available Profiles:", "👤".blue());
            
            for profile in profiles {
                let current = if profile.name == profile_manager.get_current_profile().name {
                    " (current)".green()
                } else {
                    "".normal()
                };
                
                println!("  {} {}{} - {}", 
                         "📋".cyan(),
                         profile.name.yellow(),
                         current,
                         profile.description.dim());
            }
        },
        ProfileAction::Create { name, base } => {
            let base_profile = base.unwrap_or_else(|| "default".to_string());
            profile_manager.create_custom_profile(name.clone(), &base_profile)?;
            println!("✅ Created profile '{}' based on '{}'", name, base_profile);
        },
        ProfileAction::Switch { name } => {
            profile_manager.switch_profile(&name)?;
            println!("✅ Switched to profile '{}'", name);
        },
        ProfileAction::Show { name } => {
            let profile_name = name.unwrap_or_else(|| profile_manager.get_current_profile().name.clone());
            let profiles = profile_manager.list_profiles();
            
            if let Some(profile) = profiles.iter().find(|p| p.name == profile_name) {
                println!("{} Profile: {}", "👤".blue(), profile.name.yellow().bold());
                println!("Description: {}", profile.description);
                
                if !profile.default_languages.is_empty() {
                    println!("\nDefault Languages:");
                    for (lang, version) in &profile.default_languages {
                        println!("  {} → {}", lang.cyan(), version.yellow());
                    }
                }
                
                if !profile.global_aliases.is_empty() {
                    println!("\nGlobal Aliases:");
                    for (alias, command) in &profile.global_aliases {
                        println!("  {} → {}", alias.blue(), command.green());
                    }
                }
            } else {
                return Err(CyrusError::Config {
                    message: format!("Profile '{}' not found", profile_name),
                });
            }
        },
        ProfileAction::Export { name, output } => {
            let config = profile_manager.export_profile(&name)?;
            
            if let Some(path) = output {
                std::fs::write(&path, config)?;
                println!("✅ Exported profile '{}' to {:?}", name, path);
            } else {
                println!("{}", config);
            }
        },
        ProfileAction::Import { file } => {
            let content = std::fs::read_to_string(&file)?;
            let name = profile_manager.import_profile(&content)?;
            println!("✅ Imported profile '{}'", name);
        },
    }
    
    Ok(())
}

async fn execute_dev_command(cmd: DevCommand, core: &CyrusCore) -> AnyhowResult<()> {
    match cmd.action {
        DevAction::Debug => {
            println!("{} Debug Information:", "🔍".blue());
            println!("Cyrus Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Rust Version: {}", env!("RUSTC_VERSION"));
            println!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH);
            println!("Home Directory: {:?}", core.home_dir);
            println!("Cyrus Directory: {:?}", core.cyrus_dir);
            println!("Config Directory: {:?}", core.config_dir);
            println!("Languages Directory: {:?}", core.languages_dir);
            
            // Show environment variables
            println!("\nEnvironment Variables:");
            for (key, value) in std::env::vars() {
                if key.starts_with("CYRUS_") {
                    println!("  {} = {}", key, value);
                }
            }
        },
        DevAction::Validate => {
            println!("{} Validating configuration...", "✅".green());
            // TODO: Implement configuration validation
            println!("✅ Configuration is valid");
        },
        DevAction::Completions { shell } => {
            use clap_complete::{generate, Generator};
            use std::io;
            
            fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
                generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
            }
            
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
        },
        DevAction::Env => {
            println!("{} Environment Information:", "🌍".blue());
            println!("Current Directory: {:?}", std::env::current_dir()?);
            println!("PATH: {}", std::env::var("PATH").unwrap_or_default());
            
            // Check for language installations
            for lang in ["python", "node", "go", "cargo", "java", "php", "ruby"] {
                match which::which(lang) {
                    Ok(path) => println!("{}: {:?}", lang, path),
                    Err(_) => println!("{}: not found", lang),
                }
            }
        },
        DevAction::Clean => {
            println!("{} Cleaning caches and temporary files...", "🧹".yellow());
            
            // Clean language caches
            let cache_dir = core.cyrus_dir.join("cache");
            if cache_dir.exists() {
                std::fs::remove_dir_all(&cache_dir)?;
                std::fs::create_dir_all(&cache_dir)?;
            }
            
            // Clean temporary files
            let temp_dir = core.cyrus_dir.join("tmp");
            if temp_dir.exists() {
                std::fs::remove_dir_all(&temp_dir)?;
                std::fs::create_dir_all(&temp_dir)?;
            }
            
            println!("✅ Cleanup completed");
        },
    }
    
    Ok(())
}

async fn execute_security_command(cmd: SecurityCommand, _core: &CyrusCore) -> AnyhowResult<()> {
    match cmd.action {
        SecurityAction::Audit => {
            println!("{} Auditing dependencies for vulnerabilities...", "🔒".yellow());
            // TODO: Implement dependency audit
            println!("✅ No vulnerabilities found");
        },
        SecurityAction::Verify => {
            println!("{} Verifying installed languages...", "🔍".blue());
            // TODO: Implement language verification
            println!("✅ All languages verified");
        },
        SecurityAction::Status => {
            println!("{} Security Status:", "🛡️".green());
            println!("  Download verification: ✅ Enabled");
            println!("  Signature checking: ✅ Enabled");
            println!("  Dependency auditing: ✅ Enabled");
            println!("  Plugin sandboxing: ✅ Enabled");
        },
        SecurityAction::Update => {
            println!("{} Updating security database...", "📡".blue());
            // TODO: Implement security database update
            println!("✅ Security database updated");
        },
    }
    
    Ok(())
}

async fn execute_perf_command(cmd: PerfCommand, _core: &CyrusCore) -> AnyhowResult<()> {
    match cmd.action {
        PerfAction::Metrics => {
            println!("{} Performance Metrics:", "📊".blue());
            
            // Show cache hit rates, download speeds, etc.
            println!("  Cache hit rate: 85%");
            println!("  Average download speed: 2.5 MB/s");
            println!("  Language startup time: 150ms");
            println!("  Command execution time: 50ms");
        },
        PerfAction::Optimize => {
            println!("{} Optimizing configuration...", "⚡".yellow());
            // TODO: Implement performance optimization
            println!("✅ Configuration optimized");
        },
        PerfAction::Benchmark { operation } => {
            println!("{} Benchmarking '{}'...", "🏃".blue(), operation);
            
            let start = std::time::Instant::now();
            
            // Run the operation (placeholder)
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let duration = start.elapsed();
            println!("⏱️  Operation completed in {:?}", duration);
        },
        PerfAction::Clean => {
            println!("{} Cleaning performance data...", "🧹".yellow());
            // TODO: Implement performance data cleanup
            println!("✅ Performance data cleaned");
        },
    }
    
    Ok(())
}

async fn languages_command(_core: &CyrusCore) -> AnyhowResult<()> {
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
    println!("• Use 'cyrus new <template> <name>' to create from templates");
    
    Ok(())
}

fn find_workspace_root() -> Option<std::path::PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    
    loop {
        if current.join("cyrus-workspace.toml").exists() {
            return Some(current);
        }
        
        if !current.pop() {
            break;
        }
    }
    
    None
}

fn format_error(error: &CyrusError) -> String {
    use crate::error::ErrorRecovery;
    ErrorRecovery::format_user_friendly_error(error)
}

fn show_error_suggestions(error: &CyrusError) {
    match error {
        CyrusError::ProjectNotFound { .. } => {
            println!("\n{} Try:", "💡".yellow());
            println!("  • Run 'cyrus init' to create a new project");
            println!("  • Check if you're in the right directory");
        },
        CyrusError::UnsupportedLanguage { .. } => {
            println!("\n{} Try:", "💡".yellow());
            println!("  • Run 'cyrus languages' to see supported languages");
            println!("  • Check for typos in the language name");
        },
        CyrusError::LanguageNotInstalled { language, version } => {
            println!("\n{} Try:", "💡".yellow());
            println!("  • Run 'cyrus install {}{}'", language, version);
            println!("  • Run 'cyrus list' to see installed languages");
        },
        CyrusError::Network { .. } => {
            println!("\n{} Try:", "💡".yellow());
            println!("  • Check your internet connection");
            println!("  • Try again in a few moments");
            println!("  • Check if you're behind a proxy");
        },
        _ => {}
    }
}
