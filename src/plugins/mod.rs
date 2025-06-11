// src/plugins/mod.rs
//! Plugin system for extending Cyrus functionality

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use libloading::{Library, Symbol};
use crate::error::{CyrusError, Result as CyrusResult};

pub mod interface;
pub mod registry;
pub mod loader;

use interface::{CyrusPlugin, PluginInfo, PluginCapabilities};
use registry::PluginRegistry;
use loader::PluginLoader;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub min_cyrus_version: String,
    pub capabilities: PluginCapabilities,
    pub dependencies: Vec<String>,
    pub entry_point: String, // Path to the plugin library
    pub config_schema: Option<serde_json::Value>,
    pub permissions: PluginPermissions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginPermissions {
    pub filesystem_read: bool,
    pub filesystem_write: bool,
    pub network_access: bool,
    pub system_commands: bool,
    pub environment_access: bool,
}

impl Default for PluginPermissions {
    fn default() -> Self {
        Self {
            filesystem_read: false,
            filesystem_write: false,
            network_access: false,
            system_commands: false,
            environment_access: false,
        }
    }
}

pub struct PluginManager {
    registry: PluginRegistry,
    loader: PluginLoader,
    loaded_plugins: HashMap<String, LoadedPlugin>,
    plugin_directories: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct LoadedPlugin {
    pub info: PluginInfo,
    pub manifest: PluginManifest,
    pub library: Library,
    pub plugin: Box<dyn CyrusPlugin + Send + Sync>,
    pub enabled: bool,
}

impl PluginManager {
    pub fn new(plugin_directories: Vec<PathBuf>) -> Self {
        Self {
            registry: PluginRegistry::new(),
            loader: PluginLoader::new(),
            loaded_plugins: HashMap::new(),
            plugin_directories,
        }
    }
    
    /// Discover and load all plugins from configured directories
    pub async fn discover_plugins(&mut self) -> Result<()> {
        for directory in &self.plugin_directories.clone() {
            if directory.exists() {
                self.discover_plugins_in_directory(directory).await?;
            }
        }
        Ok(())
    }
    
    async fn discover_plugins_in_directory(&mut self, directory: &Path) -> Result<()> {
        let entries = std::fs::read_dir(directory)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Look for plugin manifest
                let manifest_path = path.join("cyrus-plugin.toml");
                if manifest_path.exists() {
                    match self.load_plugin_from_directory(&path).await {
                        Ok(_) => println!("✅ Loaded plugin from {:?}", path),
                        Err(e) => eprintln!("❌ Failed to load plugin from {:?}: {}", path, e),
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Load a plugin from a directory containing manifest and library
    pub async fn load_plugin_from_directory(&mut self, plugin_dir: &Path) -> CyrusResult<()> {
        // Read manifest
        let manifest_path = plugin_dir.join("cyrus-plugin.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| CyrusError::Plugin { 
                message: format!("Failed to read plugin manifest: {}", e) 
            })?;
        
        let manifest: PluginManifest = toml::from_str(&manifest_content)
            .map_err(|e| CyrusError::Plugin { 
                message: format!("Failed to parse plugin manifest: {}", e) 
            })?;
        
        // Validate plugin
        self.validate_plugin(&manifest)?;
        
        // Load the plugin library
        let library_path = plugin_dir.join(&manifest.entry_point);
        let loaded_plugin = self.loader.load_plugin(&library_path, manifest).await?;
        
        // Register the plugin
        self.loaded_plugins.insert(loaded_plugin.info.name.clone(), loaded_plugin);
        
        Ok(())
    }
    
    /// Install a plugin from a package (tar.gz, zip, or Git repository)
    pub async fn install_plugin(&mut self, source: &str) -> CyrusResult<()> {
        // Determine source type and download/extract
        let temp_dir = tempfile::tempdir()
            .map_err(|e| CyrusError::Plugin { 
                message: format!("Failed to create temp directory: {}", e) 
            })?;
        
        if source.starts_with("git:") || source.contains("://") {
            // Git repository
            self.clone_plugin_from_git(source, temp_dir.path()).await?;
        } else if source.ends_with(".tar.gz") || source.ends_with(".zip") {
            // Archive file
            self.download_and_extract_plugin(source, temp_dir.path()).await?;
        } else {
            return Err(CyrusError::Plugin {
                message: format!("Unsupported plugin source: {}", source),
            });
        }
        
        // Load the plugin
        self.load_plugin_from_directory(temp_dir.path()).await?;
        
        // Move to permanent location
        let manifest_path = temp_dir.path().join("cyrus-plugin.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest: PluginManifest = toml::from_str(&manifest_content)?;
        
        let plugin_install_dir = self.plugin_directories[0].join(&manifest.name);
        if plugin_install_dir.exists() {
            std::fs::remove_dir_all(&plugin_install_dir)?;
        }
        
        fs_extra::dir::copy(temp_dir.path(), &plugin_install_dir, &fs_extra::dir::CopyOptions::new())?;
        
        println!("✅ Plugin '{}' installed successfully", manifest.name);
        Ok(())
    }
    
    /// Enable a plugin
    pub fn enable_plugin(&mut self, name: &str) -> CyrusResult<()> {
        if let Some(plugin) = self.loaded_plugins.get_mut(name) {
            plugin.enabled = true;
            plugin.plugin.on_enable()?;
            println!("✅ Plugin '{}' enabled", name);
            Ok(())
        } else {
            Err(CyrusError::Plugin {
                message: format!("Plugin '{}' not found", name),
            })
        }
    }
    
    /// Disable a plugin
    pub fn disable_plugin(&mut self, name: &str) -> CyrusResult<()> {
        if let Some(plugin) = self.loaded_plugins.get_mut(name) {
            plugin.enabled = false;
            plugin.plugin.on_disable()?;
            println!("⏸️  Plugin '{}' disabled", name);
            Ok(())
        } else {
            Err(CyrusError::Plugin {
                message: format!("Plugin '{}' not found", name),
            })
        }
    }
    
    /// Uninstall a plugin
    pub async fn uninstall_plugin(&mut self, name: &str) -> CyrusResult<()> {
        // Disable first if enabled
        if self.loaded_plugins.contains_key(name) {
            self.disable_plugin(name)?;
        }
        
        // Remove from loaded plugins
        if let Some(plugin) = self.loaded_plugins.remove(name) {
            plugin.plugin.on_uninstall()?;
        }
        
        // Remove plugin directory
        for plugin_dir in &self.plugin_directories {
            let plugin_path = plugin_dir.join(name);
            if plugin_path.exists() {
                std::fs::remove_dir_all(&plugin_path)?;
                println!("🗑️  Plugin '{}' uninstalled", name);
                return Ok(());
            }
        }
        
        Err(CyrusError::Plugin {
            message: format!("Plugin '{}' installation directory not found", name),
        })
    }
    
    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<&LoadedPlugin> {
        self.loaded_plugins.values().collect()
    }
    
    /// Get enabled plugins only
    pub fn get_enabled_plugins(&self) -> Vec<&LoadedPlugin> {
        self.loaded_plugins.values()
            .filter(|plugin| plugin.enabled)
            .collect()
    }
    
    /// Execute a plugin command
    pub async fn execute_plugin_command(
        &self,
        plugin_name: &str,
        command: &str,
        args: &[String],
    ) -> CyrusResult<()> {
        if let Some(plugin) = self.loaded_plugins.get(plugin_name) {
            if !plugin.enabled {
                return Err(CyrusError::Plugin {
                    message: format!("Plugin '{}' is not enabled", plugin_name),
                });
            }
            
            plugin.plugin.execute_command(command, args).await
        } else {
            Err(CyrusError::Plugin {
                message: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }
    
    /// Get plugin configuration
    pub fn get_plugin_config(&self, plugin_name: &str) -> CyrusResult<Option<serde_json::Value>> {
        if let Some(plugin) = self.loaded_plugins.get(plugin_name) {
            Ok(plugin.plugin.get_config())
        } else {
            Err(CyrusError::Plugin {
                message: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }
    
    /// Set plugin configuration
    pub fn set_plugin_config(
        &mut self,
        plugin_name: &str,
        config: serde_json::Value,
    ) -> CyrusResult<()> {
        if let Some(plugin) = self.loaded_plugins.get_mut(plugin_name) {
            plugin.plugin.set_config(config)
        } else {
            Err(CyrusError::Plugin {
                message: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }
    
    /// Update all plugins
    pub async fn update_plugins(&mut self) -> CyrusResult<()> {
        let plugin_names: Vec<String> = self.loaded_plugins.keys().cloned().collect();
        
        for plugin_name in plugin_names {
            if let Err(e) = self.update_plugin(&plugin_name).await {
                eprintln!("❌ Failed to update plugin '{}': {}", plugin_name, e);
            }
        }
        
        Ok(())
    }
    
    async fn update_plugin(&mut self, plugin_name: &str) -> CyrusResult<()> {
        // This is a simplified update mechanism
        // In practice, you'd check for updates from the original source
        println!("🔄 Checking for updates for plugin '{}'", plugin_name);
        
        // For now, just return success
        // Real implementation would:
        // 1. Check the plugin's source for updates
        // 2. Download new version
        // 3. Safely replace the old version
        // 4. Reload the plugin
        
        Ok(())
    }
    
    fn validate_plugin(&self, manifest: &PluginManifest) -> CyrusResult<()> {
        // Validate Cyrus version compatibility
        let min_version = semver::Version::parse(&manifest.min_cyrus_version)
            .map_err(|_| CyrusError::Plugin {
                message: "Invalid min_cyrus_version format".to_string(),
            })?;
        
        let current_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
            .map_err(|_| CyrusError::Plugin {
                message: "Invalid current Cyrus version".to_string(),
            })?;
        
        if current_version < min_version {
            return Err(CyrusError::Plugin {
                message: format!(
                    "Plugin requires Cyrus {} or higher, but current version is {}",
                    manifest.min_cyrus_version,
                    env!("CARGO_PKG_VERSION")
                ),
            });
        }
        
        // Check if plugin with same name is already loaded
        if self.loaded_plugins.contains_key(&manifest.name) {
            return Err(CyrusError::Plugin {
                message: format!("Plugin '{}' is already loaded", manifest.name),
            });
        }
        
        Ok(())
    }
    
    async fn clone_plugin_from_git(&self, repo_url: &str, target_dir: &Path) -> CyrusResult<()> {
        let clean_url = repo_url.strip_prefix("git:").unwrap_or(repo_url);
        
        let output = tokio::process::Command::new("git")
            .args(["clone", clean_url, target_dir.to_str().unwrap()])
            .output()
            .await?;
        
        if !output.status.success() {
            return Err(CyrusError::Plugin {
                message: format!("Failed to clone plugin repository: {}", 
                    String::from_utf8_lossy(&output.stderr)),
            });
        }
        
        Ok(())
    }
    
    async fn download_and_extract_plugin(&self, url: &str, target_dir: &Path) -> CyrusResult<()> {
        // Download the archive
        let response = reqwest::get(url).await
            .map_err(|e| CyrusError::Plugin {
                message: format!("Failed to download plugin: {}", e),
            })?;
        
        let bytes = response.bytes().await
            .map_err(|e| CyrusError::Plugin {
                message: format!("Failed to download plugin content: {}", e),
            })?;
        
        // Save to temporary file
        let temp_file = target_dir.join("plugin_archive");
        std::fs::write(&temp_file, bytes)?;
        
        // Extract based on file extension
        if url.ends_with(".tar.gz") {
            crate::utils::archive::extract_tar_gz(&temp_file, target_dir)?;
        } else if url.ends_with(".zip") {
            crate::utils::archive::extract_zip(&temp_file, target_dir)?;
        } else {
            return Err(CyrusError::Plugin {
                message: "Unsupported archive format".to_string(),
            });
        }
        
        // Clean up temp file
        std::fs::remove_file(&temp_file)?;
        
        Ok(())
    }
}