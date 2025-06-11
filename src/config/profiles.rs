// src/config/profiles.rs
//! Configuration profiles for different development environments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{CyrusError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CyrusProfile {
    pub name: String,
    pub description: String,
    
    /// Default language versions for this profile
    pub default_languages: HashMap<String, String>,
    
    /// Default package managers for each language
    pub package_managers: HashMap<String, String>,
    
    /// Global aliases for this profile
    pub global_aliases: HashMap<String, String>,
    
    /// Environment variables
    pub environment_vars: HashMap<String, String>,
    
    /// Template preferences
    pub preferred_templates: HashMap<String, String>,
    
    /// Development tools configuration
    pub dev_tools: DevToolsConfig,
    
    /// Quality gates and policies
    pub quality_gates: QualityGates,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevToolsConfig {
    /// Preferred editors/IDEs
    pub preferred_editor: Option<String>,
    
    /// Code formatters per language
    pub formatters: HashMap<String, String>,
    
    /// Linters per language
    pub linters: HashMap<String, String>,
    
    /// Testing frameworks per language
    pub test_frameworks: HashMap<String, String>,
    
    /// Build tools per language
    pub build_tools: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QualityGates {
    /// Require tests before allowing commits
    pub require_tests: bool,
    
    /// Minimum test coverage percentage
    pub min_test_coverage: Option<f32>,
    
    /// Require linting to pass
    pub require_linting: bool,
    
    /// Require security audit to pass
    pub require_security_audit: bool,
    
    /// Maximum number of dependencies
    pub max_dependencies: Option<usize>,
    
    /// Banned dependencies (security/license issues)
    pub banned_dependencies: Vec<String>,
}

impl Default for CyrusProfile {
    fn default() -> Self {
        let mut default_languages = HashMap::new();
        default_languages.insert("python".to_string(), "3.11".to_string());
        default_languages.insert("javascript".to_string(), "20".to_string());
        default_languages.insert("golang".to_string(), "1.21".to_string());
        default_languages.insert("rust".to_string(), "1.75".to_string());
        default_languages.insert("java".to_string(), "21".to_string());
        
        let mut package_managers = HashMap::new();
        package_managers.insert("python".to_string(), "pip".to_string());
        package_managers.insert("javascript".to_string(), "npm".to_string());
        package_managers.insert("golang".to_string(), "go mod".to_string());
        package_managers.insert("rust".to_string(), "cargo".to_string());
        package_managers.insert("java".to_string(), "maven".to_string());
        
        Self {
            name: "default".to_string(),
            description: "Default Cyrus profile".to_string(),
            default_languages,
            package_managers,
            global_aliases: HashMap::new(),
            environment_vars: HashMap::new(),
            preferred_templates: HashMap::new(),
            dev_tools: DevToolsConfig::default(),
            quality_gates: QualityGates::default(),
        }
    }
}

impl Default for DevToolsConfig {
    fn default() -> Self {
        let mut formatters = HashMap::new();
        formatters.insert("python".to_string(), "black".to_string());
        formatters.insert("javascript".to_string(), "prettier".to_string());
        formatters.insert("rust".to_string(), "rustfmt".to_string());
        formatters.insert("golang".to_string(), "gofmt".to_string());
        
        let mut linters = HashMap::new();
        linters.insert("python".to_string(), "flake8".to_string());
        linters.insert("javascript".to_string(), "eslint".to_string());
        linters.insert("rust".to_string(), "clippy".to_string());
        linters.insert("golang".to_string(), "golint".to_string());
        
        let mut test_frameworks = HashMap::new();
        test_frameworks.insert("python".to_string(), "pytest".to_string());
        test_frameworks.insert("javascript".to_string(), "jest".to_string());
        test_frameworks.insert("rust".to_string(), "cargo test".to_string());
        test_frameworks.insert("golang".to_string(), "go test".to_string());
        test_frameworks.insert("java".to_string(), "junit".to_string());
        
        Self {
            preferred_editor: None,
            formatters,
            linters,
            test_frameworks,
            build_tools: HashMap::new(),
        }
    }
}

impl Default for QualityGates {
    fn default() -> Self {
        Self {
            require_tests: false,
            min_test_coverage: None,
            require_linting: false,
            require_security_audit: false,
            max_dependencies: None,
            banned_dependencies: vec![
                "lodash".to_string(), // Example: security issues in old versions
            ],
        }
    }
}

impl CyrusProfile {
    pub fn enterprise() -> Self {
        let mut profile = Self::default();
        profile.name = "enterprise".to_string();
        profile.description = "Enterprise profile with strict quality gates".to_string();
        
        // Stricter quality gates
        profile.quality_gates = QualityGates {
            require_tests: true,
            min_test_coverage: Some(80.0),
            require_linting: true,
            require_security_audit: true,
            max_dependencies: Some(30),
            banned_dependencies: vec![
                "lodash".to_string(),
                "moment".to_string(), // Deprecated
                "request".to_string(), // Deprecated
            ],
        };
        
        // Prefer more stable package managers
        profile.package_managers.insert("javascript".to_string(), "pnpm".to_string());
        profile.package_managers.insert("python".to_string(), "poetry".to_string());
        
        // Add enterprise-specific aliases
        profile.global_aliases.insert("audit".to_string(), "npm audit".to_string());
        profile.global_aliases.insert("sec-check".to_string(), "safety check".to_string());
        profile.global_aliases.insert("lint-all".to_string(), "find . -name '*.py' -exec flake8 {} +".to_string());
        
        profile
    }
    
    pub fn performance() -> Self {
        let mut profile = Self::default();
        profile.name = "performance".to_string();
        profile.description = "Performance-optimized profile".to_string();
        
        // Prefer faster package managers
        profile.package_managers.insert("javascript".to_string(), "bun".to_string());
        profile.package_managers.insert("python".to_string(), "uv".to_string());
        
        // Performance-focused aliases
        profile.global_aliases.insert("fast-install".to_string(), "bun install".to_string());
        profile.global_aliases.insert("bench".to_string(), "cargo bench".to_string());
        
        profile
    }
    
    pub fn minimal() -> Self {
        let mut profile = Self::default();
        profile.name = "minimal".to_string();
        profile.description = "Minimal profile with basic features only".to_string();
        
        // Only essential languages
        profile.default_languages.clear();
        profile.default_languages.insert("python".to_string(), "3.11".to_string());
        profile.default_languages.insert("javascript".to_string(), "20".to_string());
        
        // No quality gates
        profile.quality_gates = QualityGates {
            require_tests: false,
            min_test_coverage: None,
            require_linting: false,
            require_security_audit: false,
            max_dependencies: None,
            banned_dependencies: vec![],
        };
        
        profile
    }
    
    pub fn student() -> Self {
        let mut profile = Self::default();
        profile.name = "student".to_string();
        profile.description = "Student-friendly profile with learning tools".to_string();
        
        // Educational aliases
        profile.global_aliases.insert("learn".to_string(), "cyrus templates".to_string());
        profile.global_aliases.insert("example".to_string(), "cyrus new --template".to_string());
        profile.global_aliases.insert("docs".to_string(), "cyrus help".to_string());
        
        // Prefer beginner-friendly tools
        profile.package_managers.insert("javascript".to_string(), "npm".to_string());
        profile.package_managers.insert("python".to_string(), "pip".to_string());
        
        profile
    }
    
    pub fn get_default_version(&self, language: &str) -> Option<&String> {
        self.default_languages.get(language)
    }
    
    pub fn get_package_manager(&self, language: &str) -> Option<&String> {
        self.package_managers.get(language)
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate version formats
        for (lang, version) in &self.default_languages {
            if version.is_empty() {
                return Err(CyrusError::Config {
                    message: format!("Empty version for language '{}'", lang),
                });
            }
        }
        
        // Validate quality gates
        if let Some(coverage) = self.quality_gates.min_test_coverage {
            if coverage < 0.0 || coverage > 100.0 {
                return Err(CyrusError::Config {
                    message: "Test coverage must be between 0 and 100".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    pub fn merge_with(&self, other: &CyrusProfile) -> CyrusProfile {
        let mut merged = self.clone();
        
        // Merge default languages (other takes precedence)
        for (lang, version) in &other.default_languages {
            merged.default_languages.insert(lang.clone(), version.clone());
        }
        
        // Merge package managers
        for (lang, pm) in &other.package_managers {
            merged.package_managers.insert(lang.clone(), pm.clone());
        }
        
        // Merge aliases
        for (alias, command) in &other.global_aliases {
            merged.global_aliases.insert(alias.clone(), command.clone());
        }
        
        // Merge environment variables
        for (key, value) in &other.environment_vars {
            merged.environment_vars.insert(key.clone(), value.clone());
        }
        
        merged
    }
}

/// Profile manager for handling multiple profiles
pub struct ProfileManager {
    profiles: HashMap<String, CyrusProfile>,
    current_profile: String,
}

impl ProfileManager {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        
        // Add built-in profiles
        profiles.insert("default".to_string(), CyrusProfile::default());
        profiles.insert("enterprise".to_string(), CyrusProfile::enterprise());
        profiles.insert("performance".to_string(), CyrusProfile::performance());
        profiles.insert("minimal".to_string(), CyrusProfile::minimal());
        profiles.insert("student".to_string(), CyrusProfile::student());
        
        Self {
            profiles,
            current_profile: "default".to_string(),
        }
    }
    
    pub fn get_current_profile(&self) -> &CyrusProfile {
        self.profiles.get(&self.current_profile).unwrap()
    }
    
    pub fn switch_profile(&mut self, name: &str) -> Result<()> {
        if !self.profiles.contains_key(name) {
            return Err(CyrusError::Config {
                message: format!("Profile '{}' does not exist", name),
            });
        }
        self.current_profile = name.to_string();
        Ok(())
    }
    
    pub fn add_profile(&mut self, profile: CyrusProfile) {
        profile.validate().unwrap();
        self.profiles.insert(profile.name.clone(), profile);
    }
    
    pub fn list_profiles(&self) -> Vec<&CyrusProfile> {
        self.profiles.values().collect()
    }
    
    pub fn create_custom_profile(&mut self, name: String, base_profile: &str) -> Result<()> {
        let base = self.profiles.get(base_profile)
            .ok_or_else(|| CyrusError::Config {
                message: format!("Base profile '{}' does not exist", base_profile),
            })?;
        
        let mut custom = base.clone();
        custom.name = name.clone();
        custom.description = format!("Custom profile based on {}", base_profile);
        
        self.profiles.insert(name, custom);
        Ok(())
    }
    
    pub fn export_profile(&self, name: &str) -> Result<String> {
        let profile = self.profiles.get(name)
            .ok_or_else(|| CyrusError::Config {
                message: format!("Profile '{}' does not exist", name),
            })?;
        
        toml::to_string_pretty(profile)
            .map_err(|e| CyrusError::Config {
                message: format!("Failed to serialize profile: {}", e),
            })
    }
    
    pub fn import_profile(&mut self, toml_content: &str) -> Result<String> {
        let profile: CyrusProfile = toml::from_str(toml_content)
            .map_err(|e| CyrusError::Config {
                message: format!("Failed to parse profile: {}", e),
            })?;
        
        profile.validate()?;
        let name = profile.name.clone();
        self.profiles.insert(name.clone(), profile);
        Ok(name)
    }
}