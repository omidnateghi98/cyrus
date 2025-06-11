//! Language-specific implementations

pub mod python;
pub mod javascript;
pub mod golang;

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

pub fn get_language_handler(language: &str) -> Option<Box<dyn LanguageHandler + Send + Sync>> {
    match language {
        "python" => Some(Box::new(python::PythonHandler::new())),
        "javascript" => Some(Box::new(javascript::JavaScriptHandler::new())),
        "golang" => Some(Box::new(golang::GolangHandler::new())),
        _ => None,
    }
}
