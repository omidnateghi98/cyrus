// src/plugins/interface.rs
//! Plugin interface definitions and API

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Main plugin trait that all plugins must implement
#[async_trait]
pub trait CyrusPlugin: Send + Sync {
    /// Get plugin information
    fn get_info(&self) -> PluginInfo;
    
    /// Initialize the plugin
    async fn initialize(&mut self) -> Result<()>;
    
    /// Called when plugin is enabled
    fn on_enable(&self) -> Result<()>;
    
    /// Called when plugin is disabled
    fn on_disable(&self) -> Result<()>;
    
    /// Called when plugin is being uninstalled
    fn on_uninstall(&self) -> Result<()>;
    
    /// Execute a plugin-specific command
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<()>;
    
    /// Get plugin configuration
    fn get_config(&self) -> Option<serde_json::Value>;
    
    /// Set plugin configuration
    fn set_config(&mut self, config: serde_json::Value) -> Result<()>;
    
    /// Get available commands
    fn get_commands(&self) -> Vec<PluginCommand>;
    
    /// Handle project events (optional)
    async fn on_project_event(&self, event: ProjectEvent) -> Result<()> {
        let _ = event; // Default implementation does nothing
        Ok(())
    }
    
    /// Handle language events (optional)
    async fn on_language_event(&self, event: LanguageEvent) -> Result<()> {
        let _ = event; // Default implementation does nothing
        Ok(())
    }
    
    /// Provide custom language handlers (optional)
    fn get_language_handlers(&self) -> Vec<Box<dyn crate::languages::LanguageHandler + Send + Sync>> {
        Vec::new() // Default implementation provides no handlers
    }
    
    /// Provide custom templates (optional)
    fn get_templates(&self) -> Vec<crate::templates::ProjectTemplate> {
        Vec::new() // Default implementation provides no templates
    }
    
    /// Hook into command execution (optional)
    async fn pre_command_hook(&self, command: &str, args: &[String]) -> Result<()> {
        let _ = (command, args); // Default implementation does nothing
        Ok(())
    }
    
    /// Hook into command execution (optional)
    async fn post_command_hook(&self, command: &str, args: &[String], result: &Result<()>) -> Result<()> {
        let _ = (command, args, result); // Default implementation does nothing
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub capabilities: PluginCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub provides_languages: Vec<String>,
    pub provides_templates: Vec<String>,
    pub provides_commands: Vec<String>,
    pub modifies_core_behavior: bool,
    pub requires_network: bool,
    pub requires_filesystem: bool,
}

impl Default for PluginCapabilities {
    fn default() -> Self {
        Self {
            provides_languages: Vec::new(),
            provides_templates: Vec::new(),
            provides_commands: Vec::new(),
            modifies_core_behavior: false,
            requires_network: false,
            requires_filesystem: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
    pub aliases: Vec<String>,
}

/// Events that plugins can react to
#[derive(Debug, Clone)]
pub enum ProjectEvent {
    /// Project initialization started
    InitStarted { project_name: String, template: String },
    /// Project initialization completed
    InitCompleted { project_path: std::path::PathBuf },
    /// Project configuration changed
    ConfigChanged { project_path: std::path::PathBuf },
    /// Dependencies added
    DependenciesAdded { dependencies: Vec<String> },
    /// Dependencies removed
    DependenciesRemoved { dependencies: Vec<String> },
    /// Build started
    BuildStarted { project_path: std::path::PathBuf },
    /// Build completed
    BuildCompleted { project_path: std::path::PathBuf, success: bool },
    /// Tests started
    TestsStarted { project_path: std::path::PathBuf },
    /// Tests completed
    TestsCompleted { project_path: std::path::PathBuf, success: bool },
}

#[derive(Debug, Clone)]
pub enum LanguageEvent {
    /// Language installation started
    InstallStarted { language: String, version: String },
    /// Language installation completed
    InstallCompleted { language: String, version: String, success: bool },
    /// Language removal started
    RemovalStarted { language: String, version: String },
    /// Language removal completed
    RemovalCompleted { language: String, version: String },
    /// Language environment setup
    EnvironmentSetup { language: String, project_path: std::path::PathBuf },
}

/// Context provided to plugins
pub struct PluginContext {
    pub cyrus_version: String,
    pub cyrus_core: std::sync::Arc<crate::core::CyrusCore>,
    pub config: crate::config::GlobalConfig,
    pub logger: slog::Logger,
}

impl PluginContext {
    pub fn new(
        cyrus_core: std::sync::Arc<crate::core::CyrusCore>,
        config: crate::config::GlobalConfig,
    ) -> Self {
        use slog::Drain;
        
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, slog::o!("module" => "plugin"));
        
        Self {
            cyrus_version: env!("CARGO_PKG_VERSION").to_string(),
            cyrus_core,
            config,
            logger,
        }
    }
}

/// Helper trait for creating plugins
pub trait PluginFactory {
    fn create_plugin() -> Box<dyn CyrusPlugin + Send + Sync>;
}

/// Macro to help implement plugin factories
#[macro_export]
macro_rules! cyrus_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn cyrus_plugin_create() -> *mut std::os::raw::c_void {
            let plugin: Box<dyn $crate::plugins::interface::CyrusPlugin + Send + Sync> = 
                Box::new(<$plugin_type>::new());
            Box::into_raw(Box::new(plugin)) as *mut std::os::raw::c_void
        }
        
        #[no_mangle]
        pub extern "C" fn cyrus_plugin_destroy(plugin: *mut std::os::raw::c_void) {
            if !plugin.is_null() {
                unsafe {
                    let _: Box<Box<dyn $crate::plugins::interface::CyrusPlugin + Send + Sync>> = 
                        Box::from_raw(plugin as *mut _);
                }
            }
        }
        
        #[no_mangle]
        pub extern "C" fn cyrus_plugin_version() -> *const std::os::raw::c_char {
            concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const std::os::raw::c_char
        }
    };
}

/// Standard plugin implementation for simple use cases
pub struct StandardPlugin {
    pub info: PluginInfo,
    pub commands: Vec<PluginCommand>,
    pub config: Option<serde_json::Value>,
    pub command_handlers: HashMap<String, Box<dyn Fn(&[String]) -> Result<()> + Send + Sync>>,
}

impl StandardPlugin {
    pub fn new(info: PluginInfo) -> Self {
        Self {
            info,
            commands: Vec::new(),
            config: None,
            command_handlers: HashMap::new(),
        }
    }
    
    pub fn add_command<F>(&mut self, command: PluginCommand, handler: F)
    where
        F: Fn(&[String]) -> Result<()> + Send + Sync + 'static,
    {
        self.command_handlers.insert(command.name.clone(), Box::new(handler));
        self.commands.push(command);
    }
}

#[async_trait]
impl CyrusPlugin for StandardPlugin {
    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }
    
    async fn initialize(&mut self) -> Result<()> {
        println!("Initializing plugin: {}", self.info.name);
        Ok(())
    }
    
    fn on_enable(&self) -> Result<()> {
        println!("Enabled plugin: {}", self.info.name);
        Ok(())
    }
    
    fn on_disable(&self) -> Result<()> {
        println!("Disabled plugin: {}", self.info.name);
        Ok(())
    }
    
    fn on_uninstall(&self) -> Result<()> {
        println!("Uninstalling plugin: {}", self.info.name);
        Ok(())
    }
    
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<()> {
        if let Some(handler) = self.command_handlers.get(command) {
            handler(args)
        } else {
            anyhow::bail!("Command '{}' not found in plugin '{}'", command, self.info.name);
        }
    }
    
    fn get_config(&self) -> Option<serde_json::Value> {
        self.config.clone()
    }
    
    fn set_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.config = Some(config);
        Ok(())
    }
    
    fn get_commands(&self) -> Vec<PluginCommand> {
        self.commands.clone()
    }
}

/// Example plugin implementation
pub struct ExamplePlugin {
    info: PluginInfo,
    config: Option<serde_json::Value>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "example-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "An example plugin for Cyrus".to_string(),
                author: "Cyrus Team".to_string(),
                license: "MIT".to_string(),
                capabilities: PluginCapabilities {
                    provides_commands: vec!["hello".to_string(), "info".to_string()],
                    ..Default::default()
                },
            },
            config: None,
        }
    }
}

#[async_trait]
impl CyrusPlugin for ExamplePlugin {
    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }
    
    async fn initialize(&mut self) -> Result<()> {
        println!("🔌 Example plugin initialized!");
        Ok(())
    }
    
    fn on_enable(&self) -> Result<()> {
        println!("✅ Example plugin enabled!");
        Ok(())
    }
    
    fn on_disable(&self) -> Result<()> {
        println!("⏸️  Example plugin disabled!");
        Ok(())
    }
    
    fn on_uninstall(&self) -> Result<()> {
        println!("🗑️  Example plugin uninstalled!");
        Ok(())
    }
    
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<()> {
        match command {
            "hello" => {
                let name = args.first().unwrap_or(&"World".to_string());
                println!("👋 Hello, {}! This is the example plugin.", name);
            },
            "info" => {
                println!("📋 Example Plugin Information:");
                println!("  Name: {}", self.info.name);
                println!("  Version: {}", self.info.version);
                println!("  Description: {}", self.info.description);
                println!("  Author: {}", self.info.author);
            },
            _ => {
                anyhow::bail!("Unknown command: {}", command);
            }
        }
        Ok(())
    }
    
    fn get_config(&self) -> Option<serde_json::Value> {
        self.config.clone()
    }
    
    fn set_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.config = Some(config);
        println!("🔧 Example plugin configuration updated!");
        Ok(())
    }
    
    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                name: "hello".to_string(),
                description: "Say hello".to_string(),
                usage: "cyrus plugin example hello [name]".to_string(),
                examples: vec![
                    "cyrus plugin example hello".to_string(),
                    "cyrus plugin example hello Alice".to_string(),
                ],
                aliases: vec!["hi".to_string()],
            },
            PluginCommand {
                name: "info".to_string(),
                description: "Show plugin information".to_string(),
                usage: "cyrus plugin example info".to_string(),
                examples: vec!["cyrus plugin example info".to_string()],
                aliases: vec![],
            },
        ]
    }
    
    async fn on_project_event(&self, event: ProjectEvent) -> Result<()> {
        match event {
            ProjectEvent::InitStarted { project_name, template } => {
                println!("🚀 Example plugin detected project init: {} with template {}", project_name, template);
            },
            ProjectEvent::InitCompleted { project_path } => {
                println!("✅ Example plugin detected project init completed: {:?}", project_path);
            },
            _ => {
                // Handle other events as needed
            }
        }
        Ok(())
    }
}

// Export the plugin using our macro
cyrus_plugin!(ExamplePlugin);

/// Plugin for adding Docker support
pub struct DockerPlugin {
    info: PluginInfo,
    config: Option<serde_json::Value>,
}

impl DockerPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "docker-plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "Adds Docker support to Cyrus projects".to_string(),
                author: "Cyrus Team".to_string(),
                license: "MIT".to_string(),
                capabilities: PluginCapabilities {
                    provides_commands: vec!["docker-init".to_string(), "docker-build".to_string(), "docker-run".to_string()],
                    provides_templates: vec!["dockerfile".to_string()],
                    requires_filesystem: true,
                    ..Default::default()
                },
            },
            config: None,
        }
    }
    
    fn generate_dockerfile(&self, language: &str) -> String {
        match language {
            "python" => r#"FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 8000

CMD ["python", "main.py"]
"#.to_string(),
            "javascript" => r#"FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3000

CMD ["npm", "start"]
"#.to_string(),
            "rust" => r#"FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/* /app/
CMD ["./main"]
"#.to_string(),
            _ => r#"FROM alpine:latest

WORKDIR /app
COPY . .

# Add your commands here

CMD ["echo", "Hello from Docker!"]
"#.to_string(),
        }
    }
}

#[async_trait]
impl CyrusPlugin for DockerPlugin {
    fn get_info(&self) -> PluginInfo {
        self.info.clone()
    }
    
    async fn initialize(&mut self) -> Result<()> {
        println!("🐳 Docker plugin initialized!");
        Ok(())
    }
    
    fn on_enable(&self) -> Result<()> {
        println!("✅ Docker plugin enabled!");
        Ok(())
    }
    
    fn on_disable(&self) -> Result<()> {
        println!("⏸️  Docker plugin disabled!");
        Ok(())
    }
    
    fn on_uninstall(&self) -> Result<()> {
        println!("🗑️  Docker plugin uninstalled!");
        Ok(())
    }
    
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<()> {
        match command {
            "docker-init" => {
                // Find project root and language
                let current_dir = std::env::current_dir()?;
                if let Some(project_root) = crate::core::Project::find_project_root() {
                    let project = crate::core::Project::load_from_file(project_root.join("cyrus.toml"))?;
                    
                    // Generate Dockerfile
                    let dockerfile_content = self.generate_dockerfile(&project.language);
                    std::fs::write(current_dir.join("Dockerfile"), dockerfile_content)?;
                    
                    // Generate .dockerignore
                    let dockerignore = r#"target/
node_modules/
.git/
.env
*.log
"#;
                    std::fs::write(current_dir.join(".dockerignore"), dockerignore)?;
                    
                    println!("🐳 Docker files created successfully!");
                } else {
                    anyhow::bail!("No Cyrus project found. Run 'cyrus init' first.");
                }
            },
            "docker-build" => {
                let tag = args.first().unwrap_or(&"cyrus-app".to_string());
                let output = tokio::process::Command::new("docker")
                    .args(["build", "-t", tag, "."])
                    .status()
                    .await?;
                
                if output.success() {
                    println!("🐳 Docker image '{}' built successfully!", tag);
                } else {
                    anyhow::bail!("Docker build failed");
                }
            },
            "docker-run" => {
                let image = args.first().unwrap_or(&"cyrus-app".to_string());
                let port = args.get(1).unwrap_or(&"8000".to_string());
                
                println!("🐳 Running Docker container...");
                let output = tokio::process::Command::new("docker")
                    .args(["run", "-p", &format!("{}:{}", port, port), image])
                    .status()
                    .await?;
                
                if !output.success() {
                    anyhow::bail!("Docker run failed");
                }
            },
            _ => {
                anyhow::bail!("Unknown command: {}", command);
            }
        }
        Ok(())
    }
    
    fn get_config(&self) -> Option<serde_json::Value> {
        self.config.clone()
    }
    
    fn set_config(&mut self, config: serde_json::Value) -> Result<()> {
        self.config = Some(config);
        Ok(())
    }
    
    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                name: "docker-init".to_string(),
                description: "Initialize Docker support for the current project".to_string(),
                usage: "cyrus plugin docker docker-init".to_string(),
                examples: vec!["cyrus plugin docker docker-init".to_string()],
                aliases: vec!["dinit".to_string()],
            },
            PluginCommand {
                name: "docker-build".to_string(),
                description: "Build Docker image for the project".to_string(),
                usage: "cyrus plugin docker docker-build [tag]".to_string(),
                examples: vec![
                    "cyrus plugin docker docker-build".to_string(),
                    "cyrus plugin docker docker-build my-app:latest".to_string(),
                ],
                aliases: vec!["dbuild".to_string()],
            },
            PluginCommand {
                name: "docker-run".to_string(),
                description: "Run the Docker container".to_string(),
                usage: "cyrus plugin docker docker-run [image] [port]".to_string(),
                examples: vec![
                    "cyrus plugin docker docker-run".to_string(),
                    "cyrus plugin docker docker-run my-app 3000".to_string(),
                ],
                aliases: vec!["drun".to_string()],
            },
        ]
    }
    
    async fn on_project_event(&self, event: ProjectEvent) -> Result<()> {
        if let ProjectEvent::InitCompleted { project_path } = event {
            println!("💡 Tip: Add Docker support with 'cyrus plugin docker docker-init'");
            let _ = project_path; // Use the project_path if needed
        }
        Ok(())
    }
}