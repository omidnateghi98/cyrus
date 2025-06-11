//! PHP language handler implementation
//! src/languages/php.rs

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct PhpHandler {
    config: LanguageConfig,
}

impl PhpHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("php".to_string(), "php".to_string());
        run_commands.insert("composer".to_string(), "composer".to_string());

        let config = LanguageConfig {
            name: "php".to_string(),
            versions: vec![
                "7.4".to_string(),
                "8.0".to_string(),
                "8.1".to_string(),
                "8.2".to_string(),
                "8.3".to_string(),
            ],
            default_version: "8.3".to_string(),
            package_managers: vec!["composer".to_string()],
            default_package_manager: "composer".to_string(),
            install_commands: vec!["composer require".to_string()],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://windows.php.net/downloads/releases/php-{}-Win32-vs16-x64.zip", version),
            Platform::MacOS => format!("https://formulae.brew.sh/api/formula/php@{}.json", version),
            Platform::Linux => format!("https://www.php.net/distributions/php-{}.tar.gz", version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for PhpHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("🐘 Installing PHP {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        let platform = Platform::current();
        match platform {
            Platform::Linux => {
                // For Linux, compile from source or use package manager
                let output = Command::new("apt-get")
                    .args(["update"])
                    .output()
                    .context("Failed to update package list")?;

                let output = Command::new("apt-get")
                    .args(["install", "-y", &format!("php{}", version)])
                    .output()
                    .context("Failed to install PHP")?;

                if !output.status.success() {
                    anyhow::bail!("Failed to install PHP: {}", 
                        String::from_utf8_lossy(&output.stderr));
                }
            },
            Platform::MacOS => {
                // Use Homebrew for macOS
                let output = Command::new("brew")
                    .args(["install", &format!("php@{}", version)])
                    .output()
                    .context("Failed to install PHP via Homebrew")?;

                if !output.status.success() {
                    anyhow::bail!("Failed to install PHP: {}", 
                        String::from_utf8_lossy(&output.stderr));
                }
            },
            Platform::Windows => {
                let download_url = self.get_download_url(version);
                let temp_file = install_path.join(format!("php-{}.zip", version));
                
                // Download PHP
                downloader::download_file(&download_url, &temp_file).await
                    .context("Failed to download PHP")?;
                
                // Extract
                archive::extract_zip(&temp_file, install_path)
                    .context("Failed to extract PHP archive")?;
                
                // Clean up
                std::fs::remove_file(&temp_file)?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Install Composer globally
        self.install_composer(install_path).await?;
        
        println!("✅ PHP {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up PHP environment for project at {:?}", project_path);
        
        // Initialize composer.json if it doesn't exist
        let composer_json = project_path.join("composer.json");
        if !composer_json.exists() {
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("example");
            
            let output = Command::new("composer")
                .args(["init", "--name", &format!("example/{}", project_name), "--no-interaction"])
                .current_dir(project_path)
                .output()
                .context("Failed to initialize Composer project")?;

            if !output.status.success() {
                anyhow::bail!("Failed to initialize Composer project: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
            
            println!("📦 composer.json created");
        }

        // Create basic index.php if it doesn't exist
        let index_php = project_path.join("index.php");
        if !index_php.exists() {
            let basic_php = r#"<?php
echo "Hello from Cyrus PHP environment!\n";
echo "PHP version: " . PHP_VERSION . "\n";
?>"#;
            
            std::fs::write(index_php, basic_php)?;
            println!("📄 index.php created");
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

impl PhpHandler {
    async fn install_composer(&self, install_path: &Path) -> Result<()> {
        println!("📦 Installing Composer...");
        
        let composer_installer = install_path.join("composer-setup.php");
        
        // Download Composer installer
        downloader::download_file(
            "https://getcomposer.org/installer",
            &composer_installer
        ).await.context("Failed to download Composer installer")?;
        
        // Run installer
        let output = Command::new("php")
            .args([
                composer_installer.to_str().unwrap(),
                "--install-dir", install_path.to_str().unwrap(),
                "--filename", "composer"
            ])
            .output()
            .context("Failed to install Composer")?;

        if !output.status.success() {
            anyhow::bail!("Failed to install Composer: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        // Clean up installer
        std::fs::remove_file(&composer_installer)?;
        
        println!("✅ Composer installed successfully");
        Ok(())
    }
}