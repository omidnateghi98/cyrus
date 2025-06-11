//! Java language handler implementation
//! src/languages/java.rs

use super::{LanguageConfig, LanguageHandler};
use crate::utils::{downloader, archive, platform::Platform};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

pub struct JavaHandler {
    config: LanguageConfig,
}

impl JavaHandler {
    pub fn new() -> Self {
        let mut run_commands = std::collections::HashMap::new();
        run_commands.insert("java".to_string(), "java".to_string());
        run_commands.insert("javac".to_string(), "javac".to_string());
        run_commands.insert("mvn".to_string(), "mvn".to_string());
        run_commands.insert("gradle".to_string(), "gradle".to_string());

        let config = LanguageConfig {
            name: "java".to_string(),
            versions: vec![
                "8".to_string(),
                "11".to_string(),
                "17".to_string(),
                "21".to_string(),
            ],
            default_version: "21".to_string(),
            package_managers: vec![
                "maven".to_string(),
                "gradle".to_string(),
            ],
            default_package_manager: "maven".to_string(),
            install_commands: vec![
                "mvn install".to_string(),
                "gradle build".to_string(),
            ],
            run_commands,
        };

        Self { config }
    }

    fn get_download_url(&self, version: &str) -> String {
        let platform = Platform::current();
        let arch = crate::utils::platform::Architecture::current();
        
        match platform {
            Platform::Windows => match arch {
                crate::utils::platform::Architecture::X64 => 
                    format!("https://download.oracle.com/java/{}/latest/jdk-{}_windows-x64_bin.zip", version, version),
                _ => panic!("Unsupported architecture for Windows"),
            },
            Platform::MacOS => match arch {
                crate::utils::platform::Architecture::X64 => 
                    format!("https://download.oracle.com/java/{}/latest/jdk-{}_macos-x64_bin.tar.gz", version, version),
                crate::utils::platform::Architecture::Arm64 => 
                    format!("https://download.oracle.com/java/{}/latest/jdk-{}_macos-aarch64_bin.tar.gz", version, version),
                _ => panic!("Unsupported architecture for macOS"),
            },
            Platform::Linux => match arch {
                crate::utils::platform::Architecture::X64 => 
                    format!("https://download.oracle.com/java/{}/latest/jdk-{}_linux-x64_bin.tar.gz", version, version),
                crate::utils::platform::Architecture::Arm64 => 
                    format!("https://download.oracle.com/java/{}/latest/jdk-{}_linux-aarch64_bin.tar.gz", version, version),
                _ => panic!("Unsupported architecture for Linux"),
            },
            _ => panic!("Unsupported platform"),
        }
    }
}

#[async_trait]
impl LanguageHandler for JavaHandler {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()> {
        println!("☕ Installing Java {} to {:?}", version, install_path);
        
        std::fs::create_dir_all(install_path)?;
        
        let download_url = self.get_download_url(version);
        let temp_file = install_path.join(format!("jdk-{}.archive", version));
        
        // Download JDK
        downloader::download_file(&download_url, &temp_file).await
            .context("Failed to download JDK")?;
        
        // Extract based on platform
        let platform = Platform::current();
        match platform {
            Platform::Linux | Platform::MacOS => {
                archive::extract_tar_gz(&temp_file, install_path)
                    .context("Failed to extract JDK archive")?;
            },
            Platform::Windows => {
                archive::extract_zip(&temp_file, install_path)
                    .context("Failed to extract JDK archive")?;
            },
            _ => {
                return Err(anyhow::anyhow!("Platform installation not implemented"));
            }
        }
        
        // Clean up
        std::fs::remove_file(&temp_file)?;
        
        println!("✅ Java {} installed successfully", version);
        Ok(())
    }

    async fn setup_environment(&self, project_path: &Path) -> Result<()> {
        println!("🔧 Setting up Java environment for project at {:?}", project_path);
        
        // Check for existing build files
        let has_maven = project_path.join("pom.xml").exists();
        let has_gradle = project_path.join("build.gradle").exists() || project_path.join("build.gradle.kts").exists();
        
        if !has_maven && !has_gradle {
            // Create a basic Maven project structure
            let project_name = project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("example");
            
            // Create directory structure
            std::fs::create_dir_all(project_path.join("src/main/java"))?;
            std::fs::create_dir_all(project_path.join("src/test/java"))?;
            
            // Create basic pom.xml
            let pom_xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 
         http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    
    <groupId>com.example</groupId>
    <artifactId>{}</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>
    
    <properties>
        <maven.compiler.source>21</maven.compiler.source>
        <maven.compiler.target>21</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>
    
    <dependencies>
        <dependency>
            <groupId>junit</groupId>
            <artifactId>junit</artifactId>
            <version>4.13.2</version>
            <scope>test</scope>
        </dependency>
    </dependencies>
    
    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.11.0</version>
            </plugin>
        </plugins>
    </build>
</project>"#, project_name);
            
            std::fs::write(project_path.join("pom.xml"), pom_xml)?;
            
            // Create a basic Main.java
            let main_java = format!(r#"package com.example;

public class Main {{
    public static void main(String[] args) {{
        System.out.println("Hello from Cyrus Java environment!");
        System.out.println("Java version: " + System.getProperty("java.version"));
    }}
}}"#);
            
            std::fs::write(project_path.join("src/main/java/Main.java"), main_java)?;
            
            println!("📦 Maven project structure created");
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