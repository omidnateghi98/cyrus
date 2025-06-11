//! Rust language handler implementation
//! src/languages/rust.rs

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct RustHandler {
    config: LanguageConfig,
}

impl RustHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("cargo".to_string(), "cargo".to_string());
        run_commands.insert("rustc".to_string(), "rustc".to_string());
        run_commands.insert("rustup".to_string(), "rustup".to_string());

        let config = LanguageConfig {
            name: "rust".to_string(),
            versions: vec![
                "1.70".to_string(),
                "1.71".to_string(),
                "1.72".to_string(),
                "1.73".to_string(),
                "1.74".to_string(),
                "1.75".to_string(),
            ],
            default_version: "1.75".to_string(),
            package_managers: vec!["cargo".to_string()],
            default_package_manager: "cargo".to_string(),
            install_commands: vec!["cargo add".to_string()],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://forge.rust-lang.org/infra/channel-layout.html#channel-layout-{}-x86_64-pc-windows-msvc.tar.gz", version),
            Platform::MacOS => format!("https://forge.rust-lang.org/infra/channel-layout.html#channel-layout-{}-x86_64-apple-darwin.tar.gz", version),
            Platform::Linux => format!("https://forge.rust-lang.org/infra/channel-layout.html#channel-layout-{}-x86_64-unknown-linux-gnu.tar.gz", version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for RustHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("🦀 Installing Rust {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        // For Rust, we'll use rustup for installation
        let output = Command::new("curl")
            .args(["--proto", "=https", "--tlsv1.2", "-sSf", "https://sh.rustup.rs"])
            .output()
            .context("Failed to download rustup installer")?;

        if !output.status.success() {
            anyhow::bail!("Failed to download rustup installer: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        // Install specific version using rustup
        let install_script = String::from_utf8(output.stdout)?;
        std::fs::write(install_path.join("rustup-init.sh"), install_script)?;

        let output = Command::new("sh")
            .args([
                install_path.join("rustup-init.sh").to_str().unwrap(),
                "-y",
                "--default-toolchain", version,
                "--profile", "default"
            ])
            .env("RUSTUP_HOME", install_path.join("rustup"))
            .env("CARGO_HOME", install_path.join("cargo"))
            .output()
            .context("Failed to install Rust")?;

        if !output.status.success() {
            anyhow::bail!("Failed to install Rust: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        println!("✅ Rust {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Rust environment for project at {:?}", project_path);
        
        // Initialize Cargo.toml if it doesn't exist
        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("example");
            
            let output = Command::new("cargo")
                .args(["init", "--name", project_name, "."])
                .current_dir(project_path)
                .output()
                .context("Failed to initialize Cargo project")?;

            if !output.status.success() {
                anyhow::bail!("Failed to initialize Cargo project: {}", 
                    String::from_utf8_lossy(&output.stderr));
            }
            
            println!("📦 Cargo.toml created");
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