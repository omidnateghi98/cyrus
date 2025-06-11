// src/config/mod.rs
//! Enhanced configuration management with profiles and validation

use crate::error::{CyrusError, Result, ValidationError, ValidationWarning};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use validator::{Validate, ValidationErrors};

pub mod profiles;
pub mod validation;

use profiles::CyrusProfile;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct GlobalConfig {
    #[validate(length(min = 1))]
    pub default_profile: String,
    
    pub profiles: HashMap<String, CyrusProfile>,
    
    #[serde(default)]
    pub global_aliases: HashMap<String, String>,
    
    #[serde(default)]
    pub security_settings: SecuritySettings,
    
    #[serde(default)]
    pub network_settings: NetworkSettings,
    
    #[serde(default)]
    pub ui_settings: UiSettings,
    
    #[serde(default)]
    pub plugin_settings: PluginSettings,
    
    #[validate(range(min = 1, max = 10))]
    #[serde(default = "default_parallel_downloads")]
    pub parallel_downloads: u32,
    
    #[serde(default)]
    pub cache_settings: CacheSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecuritySettings {
    #[serde(default = "default_true")]
    pub verify_downloads: bool,
    
    #[serde(default = "default_true")]
    pub check_signatures: bool,
    
    #[serde(default)]
    pub trusted_sources: Vec<String>,
    
    #[serde(default = "default_true")]
    pub audit_dependencies: bool,
    
    #[serde(default = "default_security_level")]
    pub security_level: SecurityLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Paranoid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkSettings {
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    
    pub proxy: Option<String>,
    
    #[serde(default)]
    pub mirrors: HashMap<String, Vec<String>>,
    
    #[serde(default = "default_true")]
    pub use_ipv6: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiSettings {
    #[serde(default = "default_true")]
    pub colored_output: bool,
    
    #[serde(default = "default_true")]
    pub show_progress: bool,
    
    #[serde(default = "default_true")]
    pub interactive_prompts: bool,
    
    #[serde(default = "default_verbosity")]
    pub verbosity: VerbosityLevel,
    
    #[serde(default)]
    pub theme: UiTheme,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerbosityLevel {
    Quiet,
    Normal,
    Verbose,
    Debug,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UiTheme {
    Default,
    Dark,
    Light,
    Colorblind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default)]
    pub plugin_directories: Vec<PathBuf>,
    
    #[serde(default)]
    pub trusted_plugins: Vec<String>,
    
    #[serde(default = "default_false")]
    pub auto_update_plugins: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default = "default_cache_size")]
    pub max_size_mb: u64,
    
    #[serde(default = "default_cache_ttl")]
    pub ttl_hours: u32,
    
    #[serde(default = "default_true")]
    pub auto_cleanup: bool,
}

// Default value functions
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_parallel_downloads() -> u32 { 4 }
fn default_timeout() -> u64 { 30 }
fn default_retries() -> u32 { 3 }
fn default_verbosity() -> VerbosityLevel { VerbosityLevel::Normal }
fn default_security_level() -> SecurityLevel { SecurityLevel::Medium }
fn default_cache_size() -> u64 { 1024 }
fn default_cache_ttl() -> u32 { 24 }

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            verify_downloads: true,
            check_signatures: true,
            trusted_sources: vec![
                "https://nodejs.org".to_string(),
                "https://www.python.org".to_string(),
                "https://go.dev".to_string(),
                "https://forge.rust-lang.org".to_string(),
            ],
            audit_dependencies: true,
            security_level: SecurityLevel::Medium,
        }
    }
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            proxy: None,
            mirrors: HashMap::new(),
            use_ipv6: true,
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            colored_output: true,
            show_progress: true,
            interactive_prompts: true,
            verbosity: VerbosityLevel::Normal,
            theme: UiTheme::Default,
        }
    }
}

impl Default for PluginSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin_directories: vec![],
            trusted_plugins: vec![],
            auto_update_plugins: false,
        }
    }
}

impl Default for CacheSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 1024,
            ttl_hours: 24,
            auto_cleanup: true,
        }
    }
}

impl GlobalConfig {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_string(), CyrusProfile::default());
        
        Self {
            default_profile: "default".to_string(),
            profiles,
            global_aliases: HashMap::new(),
            security_settings: SecuritySettings::default(),
            network_settings: NetworkSettings::default(),
            ui_settings: UiSettings::default(),
            plugin_settings: PluginSettings::default(),
            parallel_downloads: 4,
            cache_settings: CacheSettings::default(),
        }
    }
    
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CyrusError::Config { 
                message: format!("Failed to read config file: {}", e) 
            })?;
        
        let config: GlobalConfig = toml::from_str(&content)
            .map_err(|e| CyrusError::Config { 
                message: format!("Failed to parse config file: {}", e) 
            })?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.validate()?;
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| CyrusError::Config { 
                message: format!("Failed to serialize config: {}", e) 
            })?;
        
        std::fs::write(path, content)
            .map_err(|e| CyrusError::Config { 
                message: format!("Failed to write config file: {}", e) 
            })?;
        
        Ok(())
    }
    
    pub fn validate(&self) -> Result<()> {
        if let Err(errors) = Validate::validate(self) {
            return Err(CyrusError::Validation { 
                errors: validation_errors_to_vec(errors) 
            });
        }
        
        // Custom validation
        if !self.profiles.contains_key(&self.default_profile) {
            return Err(CyrusError::Config { 
                message: format!("Default profile '{}' does not exist", self.default_profile) 
            });
        }
        
        Ok(())
    }
    
    pub fn get_current_profile(&self) -> &CyrusProfile {
        self.profiles.get(&self.default_profile)
            .expect("Default profile should exist after validation")
    }
    
    pub fn add_profile(&mut self, name: String, profile: CyrusProfile) {
        self.profiles.insert(name, profile);
    }
    
    pub fn switch_profile(&mut self, name: &str) -> Result<()> {
        if !self.profiles.contains_key(name) {
            return Err(CyrusError::Config { 
                message: format!("Profile '{}' does not exist", name) 
            });
        }
        self.default_profile = name.to_string();
        Ok(())
    }
    
    pub fn add_global_alias(&mut self, alias: String, command: String) {
        self.global_aliases.insert(alias, command);
    }
    
    pub fn get_warnings(&self) -> Vec<ValidationWarning> {
        let mut warnings = Vec::new();
        
        // Check for performance warnings
        if self.parallel_downloads > 8 {
            warnings.push(ValidationWarning::TooManyDependencies { 
                count: self.parallel_downloads as usize 
            });
        }
        
        // Check security settings
        if !self.security_settings.verify_downloads {
            warnings.push(ValidationWarning::SecurityVulnerability { 
                dep: "Download verification disabled".to_string() 
            });
        }
        
        warnings
    }
}

fn validation_errors_to_vec(errors: ValidationErrors) -> Vec<ValidationError> {
    errors.field_errors()
        .into_iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |error| {
                ValidationError::InvalidConfig { 
                    message: format!("Field '{}': {}", field, error.message.as_ref().unwrap_or(&std::borrow::Cow::Borrowed("validation failed")))
                }
            })
        })
        .collect()
}

/// Enhanced project configuration with validation
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct EnhancedProject {
    #[validate(length(min = 1))]
    pub name: String,
    
    #[validate(length(min = 1))]
    pub language: String,
    
    #[validate(custom = "validate_version")]
    pub version: String,
    
    #[validate(length(min = 1))]
    pub package_manager: String,
    
    #[serde(default)]
    pub dependencies: Vec<String>,
    
    #[serde(default)]
    pub dev_dependencies: Vec<String>,
    
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    
    #[serde(default)]
    pub environment: HashMap<String, String>,
    
    #[serde(default = "default_true")]
    pub enable_aliases: bool,
    
    #[serde(default)]
    pub custom_aliases: HashMap<String, String>,
    
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
    
    #[serde(default)]
    pub template_source: Option<String>,
    
    #[serde(default)]
    pub metadata: ProjectMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceConfig {
    pub root: PathBuf,
    pub members: Vec<String>,
    pub shared_dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub cyrus_version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            author: None,
            description: None,
            tags: Vec::new(),
            cyrus_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

fn validate_version(version: &str) -> std::result::Result<(), validator::ValidationError> {
    if semver::Version::parse(version).is_ok() || version.chars().all(|c| c.is_numeric() || c == '.') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_version"))
    }
}

impl EnhancedProject {
    pub fn validate_comprehensive(&self) -> Result<Vec<ValidationWarning>> {
        // Basic validation
        self.validate()
            .map_err(|errors| CyrusError::Validation { 
                errors: validation_errors_to_vec(errors) 
            })?;
        
        let mut warnings = Vec::new();
        
        // Language-specific validations
        match self.language.as_str() {
            "javascript" => {
                if !self.dependencies.iter().any(|d| d.contains("typescript")) 
                   && !self.dev_dependencies.iter().any(|d| d.contains("typescript")) {
                    warnings.push(ValidationWarning::SuggestTypescript);
                }
                
                if self.package_manager == "npm" && 
                   (self.dependencies.len() + self.dev_dependencies.len()) > 10 {
                    warnings.push(ValidationWarning::SuboptimalPackageManager { 
                        pm: "npm".to_string(), 
                        suggested: "pnpm".to_string() 
                    });
                }
            },
            "python" => {
                let common_dev_deps = ["pytest", "black", "flake8", "mypy"];
                let missing: Vec<String> = common_dev_deps.iter()
                    .filter(|&dep| !self.dev_dependencies.iter().any(|d| d.contains(dep)))
                    .map(|&s| s.to_string())
                    .collect();
                
                if !missing.is_empty() {
                    warnings.push(ValidationWarning::MissingDevDependencies { 
                        suggestions: missing 
                    });
                }
            },
            _ => {}
        }
        
        // Check for too many dependencies
        let total_deps = self.dependencies.len() + self.dev_dependencies.len();
        if total_deps > 50 {
            warnings.push(ValidationWarning::TooManyDependencies { count: total_deps });
        }
        
        Ok(warnings)
    }
}