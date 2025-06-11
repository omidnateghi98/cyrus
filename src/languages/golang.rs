//! Go language handler implementation

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct GolangHandler {
    config: LanguageConfig,
}

impl GolangHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("go".to_string(), "go".to_string());

        let config = LanguageConfig {
            name: "golang".to_string(),
            versions: vec![
                "1.19".to_string(),
                "1.20".to_string(),
                "1.21".to_string(),
                "1.22".to_string()
            ],
            default_version: "1.21".to_string(),
            package_managers: vec!["go mod".to_string()],
            default_package_manager: "go mod".to_string(),
            install_commands: vec!["go get".to_string()],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://go.dev/dl/go{}.windows-amd64.zip", version),
            Platform::MacOS => format!("https://go.dev/dl/go{}.darwin-amd64.tar.gz", version),
            Platform::Linux => format!("https://go.dev/dl/go{}.linux-amd64.tar.gz", version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for GolangHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("🐹 Installing Go {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        let download_url = self.get_download_url(version);
        let temp_file = install_path.join(format!("go-{}.archive", version));
        
        // Download Go
        downloader::download_file(&download_url, &temp_file).await
            .context("Failed to download Go")?;
        
        // Extract based on platform
        let platform = Platform::current();
        match platform {
            Platform::Linux | Platform::MacOS => {
                archive::extract_tar_gz(&temp_file, install_path)
                    .context("Failed to extract Go archive")?;
            },
            Platform::Windows => {
                archive::extract_zip(&temp_file, install_path)
                    .context("Failed to extract Go archive")?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Clean up
        std::fs::remove_file(&temp_file)?;
        
        println!("✅ Go {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Go environment for project at {:?}", project_path);
        
        // Initialize go.mod if it doesn't exist
        let go_mod = project_path.join("go.mod");
        if !go_mod.exists() {
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("example");
            
            let output = Command::new("go")
                .args(["mod", "init", project_name])
                .current_dir(project_path)
                .output()
                .context("Failed to initialize Go module")?;

            if !output.status.success() {
                anyhow::bail!("Failed to initialize Go module: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
            
            println!("📦 go.mod created");
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
