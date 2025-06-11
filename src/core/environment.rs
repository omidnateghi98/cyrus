//! Environment management for Cyrus projects

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Environment {
    pub language: String,
    pub version: String,
    pub package_manager: String,
    pub variables: HashMap<String, String>,
    pub paths: Vec<PathBuf>,
}

impl Environment {
    pub fn new(language: String, version: String, package_manager: String) -> Self {
        Self {
            language,
            version,
            package_manager,
            variables: HashMap::new(),
            paths: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    pub fn add_path(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn get_executable_path(&self, base_path: &PathBuf) -> PathBuf {
        match self.language.as_str() {
            "python" => base_path.join("bin").join("python"),
            "javascript" => base_path.join("bin").join("node"),
            "golang" => base_path.join("bin").join("go"),
            _ => base_path.join("bin").join(&self.language),
        }
    }
}
