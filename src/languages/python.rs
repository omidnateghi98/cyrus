//! Python language handler implementation

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct PythonHandler {
    config: LanguageConfig,
}

impl PythonHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("python".to_string(), "python".to_string());
        run_commands.insert("pip".to_string(), "pip".to_string());
        run_commands.insert("poetry".to_string(), "poetry".to_string());
        run_commands.insert("pipenv".to_string(), "pipenv".to_string());

        let config = LanguageConfig {
            name: "python".to_string(),
            versions: vec![
                "3.8".to_string(),
                "3.9".to_string(), 
                "3.10".to_string(),
                "3.11".to_string(),
                "3.12".to_string()
            ],
            default_version: "3.11".to_string(),
            package_managers: vec![
                "pip".to_string(),
                "poetry".to_string(),
                "pipenv".to_string()
            ],
            default_package_manager: "pip".to_string(),
            install_commands: vec!["pip install".to_string()],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://www.python.org/ftp/python/{version}.0/python-{version}.0-embed-amd64.zip", version = version),
            Platform::MacOS => format!("https://www.python.org/ftp/python/{version}.0/python-{version}.0-macos11.pkg", version = version),
            Platform::Linux => format!("https://www.python.org/ftp/python/{version}.0/Python-{version}.0.tgz", version = version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for PythonHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("🐍 Installing Python {} to {:?}", version, install_path);
        
        // Create installation directory
        std::fs::create_dir_all(install_path)?;
        
        // Get download URL
        let download_url = self.get_download_url(version);
        let temp_file = install_path.join(format!("python-{}.archive", version));
        
        // Download Python
        downloader::download_file(&download_url, &temp_file).await
            .context("Failed to download Python")?;
        
        // Extract based on platform
        let platform = Platform::current();
        match platform {
            Platform::Linux => {
                archive::extract_tar_gz(&temp_file, install_path)
                    .context("Failed to extract Python archive")?;
            },
            Platform::Windows => {
                archive::extract_zip(&temp_file, install_path)
                    .context("Failed to extract Python archive")?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Clean up temporary file
        std::fs::remove_file(&temp_file)?;
        
        println!("✅ Python {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Python environment for project at {:?}", project_path);
        
        // Create virtual environment
        let venv_path = project_path.join(".venv");
        
        if !venv_path.exists() {
            let output = Command::new("python")
                .args(["-m", "venv", venv_path.to_str().unwrap()])
                .current_dir(project_path)
                .output()
                .context("Failed to create virtual environment")?;

            if !output.status.success() {
                anyhow::bail!("Failed to create virtual environment: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
            
            println!("📦 Virtual environment created at .venv");
        }

        Ok(())
    }

    async fn run_command(&self, command: &str, args: &[String]) -> Result<()> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        
        let status = cmd.status().context("Failed to execute command")?;
        
        if !status.success() {
            anyhow::bail!("Command failed with exit code: {:?}", status.code());
        }
        
        Ok(())
    }

    fn get_config(&self) -> &LanguageConfig {
        &self.config
    }
}
