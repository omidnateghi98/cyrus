//! JavaScript/Node.js language handler implementation

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct JavaScriptHandler {
    config: LanguageConfig,
}

impl JavaScriptHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("node".to_string(), "node".to_string());
        run_commands.insert("npm".to_string(), "npm".to_string());
        run_commands.insert("yarn".to_string(), "yarn".to_string());
        run_commands.insert("pnpm".to_string(), "pnpm".to_string());
        run_commands.insert("bun".to_string(), "bun".to_string());

        let config = LanguageConfig {
            name: "javascript".to_string(),
            versions: vec![
                "16".to_string(),
                "18".to_string(),
                "20".to_string(),
                "21".to_string()
            ],
            default_version: "20".to_string(),
            package_managers: vec![
                "npm".to_string(),
                "yarn".to_string(),
                "pnpm".to_string(),
                "bun".to_string()
            ],
            default_package_manager: "npm".to_string(),
            install_commands: vec![
                "npm install".to_string(),
                "yarn add".to_string(),
                "pnpm add".to_string(),
                "bun add".to_string()
            ],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://nodejs.org/dist/v{}.0.0/node-v{}.0.0-win-x64.zip", version, version),
            Platform::MacOS => format!("https://nodejs.org/dist/v{}.0.0/node-v{}.0.0-darwin-x64.tar.gz", version, version),
            Platform::Linux => format!("https://nodejs.org/dist/v{}.0.0/node-v{}.0.0-linux-x64.tar.xz", version, version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for JavaScriptHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("🟢 Installing Node.js {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        let download_url = self.get_download_url(version);
        let temp_file = install_path.join(format!("nodejs-{}.archive", version));
        
        // Download Node.js
        downloader::download_file(&download_url, &temp_file).await
            .context("Failed to download Node.js")?;
        
        // Extract based on platform
        let platform = Platform::current();
        match platform {
            Platform::Linux | Platform::MacOS => {
                archive::extract_tar_gz(&temp_file, install_path)
                    .context("Failed to extract Node.js archive")?;
            },
            Platform::Windows => {
                archive::extract_zip(&temp_file, install_path)
                    .context("Failed to extract Node.js archive")?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Clean up
        std::fs::remove_file(&temp_file)?;
        
        println!("✅ Node.js {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Node.js environment for project at {:?}", project_path);
        
        // Initialize package.json if it doesn't exist
        let package_json = project_path.join("package.json");
        if !package_json.exists() {
            let output = Command::new("npm")
                .args(["init", "-y"])
                .current_dir(project_path)
                .output()
                .context("Failed to initialize npm project")?;

            if !output.status.success() {
                anyhow::bail!("Failed to initialize npm project: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
            
            println!("📦 package.json created");
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
