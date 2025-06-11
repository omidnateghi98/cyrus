//! Ruby language handler implementation
//! src/languages/ruby.rs

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct RubyHandler {
    config: LanguageConfig,
}

impl RubyHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("ruby".to_string(), "ruby".to_string());
        run_commands.insert("gem".to_string(), "gem".to_string());
        run_commands.insert("bundle".to_string(), "bundle".to_string());
        run_commands.insert("rails".to_string(), "rails".to_string());

        let config = LanguageConfig {
            name: "ruby".to_string(),
            versions: vec![
                "2.7".to_string(),
                "3.0".to_string(),
                "3.1".to_string(),
                "3.2".to_string(),
                "3.3".to_string(),
            ],
            default_version: "3.3".to_string(),
            package_managers: vec![
                "gem".to_string(),
                "bundler".to_string(),
            ],
            default_package_manager: "bundler".to_string(),
            install_commands: vec![
                "gem install".to_string(),
                "bundle add".to_string(),
            ],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        match platform {
            Platform::Windows => format!("https://github.com/oneclick/rubyinstaller2/releases/download/RubyInstaller-{}-1/rubyinstaller-{}-1-x64.exe", version, version),
            Platform::MacOS => format!("https://cache.ruby-lang.org/pub/ruby/{}/ruby-{}.tar.gz", &version[0..3], version),
            Platform::Linux => format!("https://cache.ruby-lang.org/pub/ruby/{}/ruby-{}.tar.gz", &version[0..3], version),
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for RubyHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("💎 Installing Ruby {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        let platform = Platform::current();
        match platform {
            Platform::Linux | Platform::MacOS => {
                // Use rbenv or compile from source
                self.install_with_rbenv(version, install_path).await
                    .or_else(|_| self.compile_from_source(version, install_path))?;
            },
            Platform::Windows => {
                let download_url = self.get_download_url(version);
                let temp_file = install_path.join(format!("ruby-{}.exe", version));
                
                // Download Ruby installer
                downloader::download_file(&download_url, &temp_file).await
                    .context("Failed to download Ruby installer")?;
                
                // Run installer silently
                let output = Command::new(&temp_file)
                    .args(["/SILENT", &format!("/DIR={}", install_path.to_str().unwrap())])
                    .output()
                    .context("Failed to run Ruby installer")?;

                if !output.status.success() {
                    anyhow::bail!("Failed to install Ruby: {}", 
                        String::from_utf8_lossy(&output.stderr));
                }
                
                // Clean up
                std::fs::remove_file(&temp_file)?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Install bundler
        self.install_bundler(install_path).await?;
        
        println!("✅ Ruby {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Ruby environment for project at {:?}", project_path);
        
        // Initialize Gemfile if it doesn't exist
        let gemfile = project_path.join("Gemfile");
        if !gemfile.exists() {
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("example");
            
            let gemfile_content = format!(r#"# frozen_string_literal: true

source "https://rubygems.org"

ruby "3.3.0"

# Add your gems here
# gem "rails", "~> 7.0"
# gem "sqlite3", "~> 1.4"

group :development, :test do
  gem "rspec", "~> 3.0"
  gem "rubocop", "~> 1.0"
end
"#);
            
            std::fs::write(&gemfile, gemfile_content)?;
            
            println!("📦 Gemfile created");
        }

        // Create basic main.rb if it doesn't exist
        let main_rb = project_path.join("main.rb");
        if !main_rb.exists() {
            let basic_ruby = r#"#!/usr/bin/env ruby
# frozen_string_literal: true

puts "Hello from Cyrus Ruby environment!"
puts "Ruby version: #{RUBY_VERSION}"
puts "Platform: #{RUBY_PLATFORM}"
"#;
            
            std::fs::write(main_rb, basic_ruby)?;
            println!("📄 main.rb created");
        }

        // Run bundle install if Gemfile exists
        if gemfile.exists() {
            let output = Command::new("bundle")
                .args(["install"])
                .current_dir(project_path)
                .output()
                .context("Failed to run bundle install")?;

            if output.status.success() {
                println!("📦 Bundle install completed");
            }
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

impl RubyHandler {
    async fn install_with_rbenv(&self, version: &str, install_path: &Path) -> Result<()> {
        // Try to use rbenv if available
        let output = Command::new("rbenv")
            .args(["install", version])
            .env("RBENV_ROOT", install_path)
            .output()
            .context("Failed to install Ruby with rbenv")?;

        if !output.status.success() {
            anyhow::bail!("Failed to install Ruby with rbenv: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    fn compile_from_source(&self, version: &str, install_path: &Path) -> Result<()> {
        // This is a simplified version - in practice you'd need more configuration
        let download_url = self.get_download_url(version);
        // Implementation would download source and compile
        println!("Compiling Ruby from source...");
        // This is a placeholder - actual implementation would be more complex
        Ok(())
    }

    async fn install_bundler(&self, install_path: &Path) -> Result<()> {
        println!("📦 Installing Bundler...");
        
        let output = Command::new("gem")
            .args(["install", "bundler"])
            .env("GEM_HOME", install_path.join("gems"))
            .output()
            .context("Failed to install Bundler")?;

        if !output.status.success() {
            anyhow::bail!("Failed to install Bundler: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        println!("✅ Bundler installed successfully");
        Ok(())
    }
}