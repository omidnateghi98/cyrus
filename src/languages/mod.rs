//! Language-specific implementations with extended support
//! src/languages/mod.rs

pub mod python;
pub mod javascript;
pub mod golang;
pub mod rust;
pub mod java;
pub mod php;
pub mod ruby;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub versions: Vec<String>,
    pub default_version: String,
    pub package_managers: Vec<String>,
    pub default_package_manager: String,
    pub install_commands: Vec<String>,
    pub run_commands: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait LanguageHandler {
    async fn install(&self, version: &str, install_path: &std::path::Path) -> Result<()>;
    async fn setup_environment(&self, project_path: &std::path::Path) -> Result<()>;
    async fn run_command(&self, command: &str, args: &[String]) -> Result<()>;
    fn get_config(&self) -> &LanguageConfig;
}

/// Get language handler for the specified language
pub fn get_language_handler(language: &str) -> Option<Box<dyn LanguageHandler + Send + Sync>> {
    match language.to_lowercase().as_str() {
        "python" | "py" => Some(Box::new(python::PythonHandler::new())),
        "javascript" | "js" | "node" | "nodejs" => Some(Box::new(javascript::JavaScriptHandler::new())),
        "golang" | "go" => Some(Box::new(golang::GolangHandler::new())),
        "rust" | "rs" => Some(Box::new(rust::RustHandler::new())),
        "java" => Some(Box::new(java::JavaHandler::new())),
        "php" => Some(Box::new(php::PhpHandler::new())),
        "ruby" | "rb" => Some(Box::new(ruby::RubyHandler::new())),
        _ => None,
    }
}

/// Get list of all supported languages
pub fn get_supported_languages() -> Vec<&'static str> {
    vec![
        "python",
        "javascript", 
        "golang",
        "rust",
        "java",
        "php",
        "ruby",
    ]
}

/// Get language display names
pub fn get_language_display_name(language: &str) -> &'static str {
    match language.to_lowercase().as_str() {
        "python" | "py" => "Python",
        "javascript" | "js" | "node" | "nodejs" => "JavaScript/Node.js",
        "golang" | "go" => "Go",
        "rust" | "rs" => "Rust",
        "java" => "Java",
        "php" => "PHP",
        "ruby" | "rb" => "Ruby",
        _ => "Unknown",
    }
}

/// Check if language is supported
pub fn is_language_supported(language: &str) -> bool {
    get_language_handler(language).is_some()
}

/// Get suggested aliases for a language
pub fn get_language_aliases(language: &str) -> Vec<&'static str> {
    match language.to_lowercase().as_str() {
        "python" => vec!["py", "python3"],
        "javascript" => vec!["js", "node", "nodejs"],
        "golang" => vec!["go"],
        "rust" => vec!["rs"],
        "java" => vec!["java"],
        "php" => vec!["php"],
        "ruby" => vec!["rb"],
        _ => vec![],
    }
}