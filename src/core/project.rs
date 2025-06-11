//! Project management for Cyrus

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub language: String,
    pub version: String,
    pub package_manager: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub scripts: std::collections::HashMap<String, String>,
    pub environment: std::collections::HashMap<String, String>,
}

impl Project {
    pub fn new(
        name: String,
        language: String,
        version: String,
        package_manager: String,
    ) -> Self {
        Self {
            name,
            language,
            version,
            package_manager,
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            scripts: std::collections::HashMap::new(),
            environment: std::collections::HashMap::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read cyrus.toml file")?;
        
        toml::from_str(&content)
            .context("Failed to parse cyrus.toml file")
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize project configuration")?;
        
        fs::write(path, content)
            .context("Failed to write cyrus.toml file")
    }

    pub fn find_project_root() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        
        loop {
            if current.join("cyrus.toml").exists() {
                return Some(current);
            }
            
            if !current.pop() {
                break;
            }
        }
        
        None
    }
}
