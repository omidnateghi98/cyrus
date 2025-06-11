//! Enhanced project configuration with alias support
//! src/core/project.rs

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub language: String,
    pub version: String,
    pub package_manager: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub scripts: std::collections::HashMap<String, String>,
    pub environment: std::collections::HashMap<String, String>,
    
    // New fields for enhanced functionality
    #[serde(default)]
    pub enable_aliases: bool,
    
    #[serde(default)]
    pub custom_aliases: std::collections::HashMap<String, String>,
}

impl Project {
    pub fn new(
        name: String,
        language: String,
        version: String,
        package_manager: String,
    ) -> Self {
        let mut scripts = std::collections::HashMap::new();
        let mut custom_aliases = std::collections::HashMap::new();
        
        // Add default scripts based on language and package manager
        match language.as_str() {
            "javascript" => {
                scripts.insert("start".to_string(), "npm start".to_string());
                scripts.insert("dev".to_string(), "npm run dev".to_string());
                scripts.insert("build".to_string(), "npm run build".to_string());
                scripts.insert("test".to_string(), "npm test".to_string());
                
                // Add package manager specific aliases
                match package_manager.as_str() {
                    "yarn" => {
                        custom_aliases.insert("dev".to_string(), "yarn dev".to_string());
                        custom_aliases.insert("start".to_string(), "yarn start".to_string());
                        custom_aliases.insert("build".to_string(), "yarn build".to_string());
                        custom_aliases.insert("test".to_string(), "yarn test".to_string());
                    },
                    "pnpm" => {
                        custom_aliases.insert("dev".to_string(), "pnpm dev".to_string());
                        custom_aliases.insert("start".to_string(), "pnpm start".to_string());
                        custom_aliases.insert("build".to_string(), "pnpm build".to_string());
                        custom_aliases.insert("test".to_string(), "pnpm test".to_string());
                    },
                    "bun" => {
                        custom_aliases.insert("dev".to_string(), "bun run dev".to_string());
                        custom_aliases.insert("start".to_string(), "bun run start".to_string());
                        custom_aliases.insert("build".to_string(), "bun run build".to_string());
                        custom_aliases.insert("test".to_string(), "bun test".to_string());
                    },
                    _ => {} // npm is default
                }
            },
            "python" => {
                scripts.insert("start".to_string(), "python main.py".to_string());
                scripts.insert("test".to_string(), "pytest".to_string());
                scripts.insert("lint".to_string(), "flake8".to_string());
                scripts.insert("format".to_string(), "black .".to_string());
                
                match package_manager.as_str() {
                    "poetry" => {
                        custom_aliases.insert("install".to_string(), "poetry install".to_string());
                        custom_aliases.insert("add".to_string(), "poetry add".to_string());
                        custom_aliases.insert("test".to_string(), "poetry run pytest".to_string());
                        custom_aliases.insert("run".to_string(), "poetry run".to_string());
                    },
                    "pipenv" => {
                        custom_aliases.insert("install".to_string(), "pipenv install".to_string());
                        custom_aliases.insert("shell".to_string(), "pipenv shell".to_string());
                        custom_aliases.insert("run".to_string(), "pipenv run".to_string());
                    },
                    _ => {} // pip is default
                }
            },
            "golang" => {
                scripts.insert("build".to_string(), "go build".to_string());
                scripts.insert("run".to_string(), "go run main.go".to_string());
                scripts.insert("test".to_string(), "go test".to_string());
                scripts.insert("mod".to_string(), "go mod tidy".to_string());
            },
            "rust" => {
                scripts.insert("build".to_string(), "cargo build".to_string());
                scripts.insert("run".to_string(), "cargo run".to_string());
                scripts.insert("test".to_string(), "cargo test".to_string());
                scripts.insert("check".to_string(), "cargo check".to_string());
                scripts.insert("clippy".to_string(), "cargo clippy".to_string());
                scripts.insert("fmt".to_string(), "cargo fmt".to_string());
            },
            "java" => {
                scripts.insert("compile".to_string(), "javac *.java".to_string());
                scripts.insert("run".to_string(), "java Main".to_string());
                scripts.insert("test".to_string(), "mvn test".to_string());
                scripts.insert("build".to_string(), "mvn clean compile".to_string());
                
                custom_aliases.insert("mvn".to_string(), "mvn".to_string());
                custom_aliases.insert("gradle".to_string(), "gradle".to_string());
            },
            "php" => {
                scripts.insert("serve".to_string(), "php -S localhost:8000".to_string());
                scripts.insert("test".to_string(), "phpunit".to_string());
                scripts.insert("composer".to_string(), "composer".to_string());
                
                custom_aliases.insert("composer".to_string(), "composer".to_string());
            },
            "ruby" => {
                scripts.insert("run".to_string(), "ruby main.rb".to_string());
                scripts.insert("test".to_string(), "rspec".to_string());
                scripts.insert("bundle".to_string(), "bundle".to_string());
                
                custom_aliases.insert("gem".to_string(), "gem".to_string());
                custom_aliases.insert("bundle".to_string(), "bundle".to_string());
            },
            _ => {}
        }

        Self {
            name,
            language,
            version,
            package_manager,
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            scripts,
            environment: std::collections::HashMap::new(),
            enable_aliases: true, // Enable by default
            custom_aliases,
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read cyrus.toml file")?;
        
        toml::from_str(&content)
            .context("Failed to parse cyrus.toml file")
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize project configuration")?;
        
        fs::write(path, content)
            .context("Failed to write cyrus.toml file")
    }

    pub fn find_project_root() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        
        loop {
            if current.join("cyrus.toml").exists() {
                return Some(current);
            }
            
            if !current.pop() {
                break;
            }
        }
        
        None
    }

    /// Check if a command should be aliased
    pub fn get_aliased_command(&self, command: &str) -> Option<String> {
        if !self.enable_aliases {
            return None;
        }

        // First check custom aliases
        if let Some(alias) = self.custom_aliases.get(command) {
            return Some(alias.clone());
        }

        // Then check scripts
        if let Some(script) = self.scripts.get(command) {
            return Some(script.clone());
        }

        None
    }

    /// Get the full command with package manager prefix if needed
    pub fn resolve_command(&self, command: &str, args: &[String]) -> (String, Vec<String>) {
        // Check if it's an aliased command
        if let Some(aliased) = self.get_aliased_command(command) {
            let parts: Vec<&str> = aliased.split_whitespace().collect();
            if let Some(first) = parts.first() {
                let mut new_args = parts[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>();
                new_args.extend_from_slice(args);
                return (first.to_string(), new_args);
            }
        }

        // Check if it's a package manager command that needs prefixing
        let package_manager_commands = match self.language.as_str() {
            "javascript" => match self.package_manager.as_str() {
                "npm" => vec!["install", "run", "start", "test", "build"],
                "yarn" => vec!["add", "run", "start", "test", "build", "dev"],
                "pnpm" => vec!["add", "run", "start", "test", "build", "dev"],
                "bun" => vec!["add", "run", "start", "test", "build", "dev"],
                _ => vec![],
            },
            "python" => match self.package_manager.as_str() {
                "poetry" => vec!["install", "add", "run", "shell"],
                "pipenv" => vec!["install", "shell", "run"],
                _ => vec![],
            },
            _ => vec![],
        };

        if package_manager_commands.contains(&command) {
            match self.package_manager.as_str() {
                "bun" if command == "run" => {
                    // Special case for bun run
                    let mut new_args = vec!["run".to_string()];
                    new_args.extend_from_slice(args);
                    return ("bun".to_string(), new_args);
                },
                pm => {
                    let mut new_args = vec![command.to_string()];
                    new_args.extend_from_slice(args);
                    return (pm.to_string(), new_args);
                }
            }
        }

        // Default: return as-is
        (command.to_string(), args.to_vec())
    }

    /// Add a custom alias
    pub fn add_alias(&mut self, alias: String, command: String) {
        self.custom_aliases.insert(alias, command);
    }

    /// Remove a custom alias
    pub fn remove_alias(&mut self, alias: &str) {
        self.custom_aliases.remove(alias);
    }

    /// Toggle alias functionality
    pub fn toggle_aliases(&mut self) {
        self.enable_aliases = !self.enable_aliases;
    }
}