// src/workspace/mod.rs
//! Workspace management for multi-project development

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::{CyrusError, Result as CyrusResult};
use crate::core::Project;
use futures::future;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub description: String,
    pub root_path: PathBuf,
    pub members: Vec<WorkspaceMember>,
    pub shared_config: WorkspaceConfig,
    pub dependencies: WorkspaceDependencies,
    pub scripts: HashMap<String, WorkspaceScript>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: PathBuf,
    pub language: String,
    pub enabled: bool,
    pub build_order: Option<u32>,
    pub dependencies: Vec<String>, // Other workspace members this depends on
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceConfig {
    pub default_language_versions: HashMap<String, String>,
    pub shared_environment: HashMap<String, String>,
    pub common_scripts: HashMap<String, String>,
    pub build_parallel: bool,
    pub max_parallel_jobs: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceDependencies {
    pub shared_dependencies: HashMap<String, Vec<String>>, // language -> dependencies
    pub shared_dev_dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceScript {
    pub command: String,
    pub description: String,
    pub run_in_members: Vec<String>, // Empty means all members
    pub run_parallel: bool,
    pub continue_on_error: bool,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            default_language_versions: HashMap::new(),
            shared_environment: HashMap::new(),
            common_scripts: HashMap::new(),
            build_parallel: true,
            max_parallel_jobs: Some(num_cpus::get() as u32),
        }
    }
}

impl Default for WorkspaceDependencies {
    fn default() -> Self {
        Self {
            shared_dependencies: HashMap::new(),
            shared_dev_dependencies: HashMap::new(),
        }
    }
}

pub struct WorkspaceManager {
    current_workspace: Option<Workspace>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            current_workspace: None,
        }
    }
    
    /// Initialize a new workspace
    pub fn init_workspace(
        &mut self,
        name: String,
        description: String,
        path: PathBuf,
    ) -> CyrusResult<()> {
        if path.exists() && path.read_dir()?.next().is_some() {
            return Err(CyrusError::Workspace {
                message: "Directory is not empty".to_string(),
            });
        }
        
        std::fs::create_dir_all(&path)?;
        
        let workspace = Workspace {
            name: name.clone(),
            description,
            root_path: path.clone(),
            members: Vec::new(),
            shared_config: WorkspaceConfig::default(),
            dependencies: WorkspaceDependencies::default(),
            scripts: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        // Save workspace configuration
        self.save_workspace(&workspace, &path)?;
        self.current_workspace = Some(workspace);
        
        println!("✅ Workspace '{}' initialized at {:?}", name, path);
        Ok(())
    }
    
    /// Load an existing workspace
    pub fn load_workspace(&mut self, path: &Path) -> CyrusResult<()> {
        let workspace_file = path.join("cyrus-workspace.toml");
        if !workspace_file.exists() {
            return Err(CyrusError::Workspace {
                message: "No workspace configuration found".to_string(),
            });
        }
        
        let content = std::fs::read_to_string(&workspace_file)?;
        let workspace: Workspace = toml::from_str(&content)
            .map_err(|e| CyrusError::Workspace {
                message: format!("Failed to parse workspace config: {}", e),
            })?;
        
        self.current_workspace = Some(workspace);
        Ok(())
    }
    
    /// Add a project to the workspace
    pub async fn add_member(
        &mut self,
        name: String,
        path: PathBuf,
        language: Option<String>,
        create_project: bool,
    ) -> CyrusResult<()> {
        let workspace = self.get_current_workspace_mut()?;
        
        // Check if member already exists
        if workspace.members.iter().any(|m| m.name == name) {
            return Err(CyrusError::Workspace {
                message: format!("Member '{}' already exists", name),
            });
        }
        
        let member_path = workspace.root_path.join(&path);
        
        if create_project {
            // Create the project directory
            std::fs::create_dir_all(&member_path)?;
            
            // Initialize as Cyrus project if language is specified
            if let Some(lang) = &language {
                let project = Project::new(
                    name.clone(),
                    lang.clone(),
                    workspace.shared_config.default_language_versions
                        .get(lang)
                        .cloned()
                        .unwrap_or_else(|| "latest".to_string()),
                    "npm".to_string(), // Default, will be updated based on language
                );
                
                project.save_to_file(&member_path.join("cyrus.toml"))?;
            }
        } else if !member_path.exists() {
            return Err(CyrusError::Workspace {
                message: format!("Project path does not exist: {:?}", member_path),
            });
        }
        
        // Detect language if not specified
        let detected_language = language.unwrap_or_else(|| {
            self.detect_project_language(&member_path)
                .unwrap_or_else(|| "unknown".to_string())
        });
        
        let member = WorkspaceMember {
            name: name.clone(),
            path,
            language: detected_language,
            enabled: true,
            build_order: None,
            dependencies: Vec::new(),
        };
        
        workspace.members.push(member);
        workspace.updated_at = chrono::Utc::now();
        
        self.save_workspace(workspace, &workspace.root_path.clone())?;
        
        println!("✅ Added member '{}' to workspace", name);
        Ok(())
    }
    
    /// Remove a member from the workspace
    pub fn remove_member(&mut self, name: &str, delete_files: bool) -> CyrusResult<()> {
        let workspace = self.get_current_workspace_mut()?;
        
        let member_index = workspace.members.iter()
            .position(|m| m.name == name)
            .ok_or_else(|| CyrusError::Workspace {
                message: format!("Member '{}' not found", name),
            })?;
        
        let member = workspace.members.remove(member_index);
        
        if delete_files {
            let member_path = workspace.root_path.join(&member.path);
            if member_path.exists() {
                std::fs::remove_dir_all(&member_path)?;
                println!("🗑️  Deleted files for member '{}'", name);
            }
        }
        
        workspace.updated_at = chrono::Utc::now();
        self.save_workspace(workspace, &workspace.root_path.clone())?;
        
        println!("✅ Removed member '{}' from workspace", name);
        Ok(())
    }
    
    /// List all workspace members
    pub fn list_members(&self) -> CyrusResult<Vec<&WorkspaceMember>> {
        let workspace = self.get_current_workspace()?;
        Ok(workspace.members.iter().collect())
    }
    
    /// Run a command in all or specific workspace members
    pub async fn run_in_workspace(
        &self,
        command: &str,
        args: &[String],
        members: Option<Vec<String>>,
        parallel: bool,
    ) -> CyrusResult<()> {
        let workspace = self.get_current_workspace()?;
        
        let target_members: Vec<&WorkspaceMember> = if let Some(member_names) = members {
            workspace.members.iter()
                .filter(|m| member_names.contains(&m.name) && m.enabled)
                .collect()
        } else {
            workspace.members.iter()
                .filter(|m| m.enabled)
                .collect()
        };
        
        if target_members.is_empty() {
            println!("⚠️  No enabled members found");
            return Ok(());
        }
        
        println!("🚀 Running '{}' in {} members", command, target_members.len());
        
        if parallel {
            self.run_parallel(&target_members, command, args).await?;
        } else {
            self.run_sequential(&target_members, command, args).await?;
        }
        
        Ok(())
    }
    
    /// Build all workspace members in dependency order
    pub async fn build_workspace(&self, parallel: bool) -> CyrusResult<()> {
        let workspace = self.get_current_workspace()?;
        
        // Sort members by build order and dependencies
        let build_order = self.calculate_build_order(&workspace.members)?;
        
        println!("🔨 Building workspace with {} members", build_order.len());
        
        for batch in build_order {
            if parallel && batch.len() > 1 {
                println!("🔄 Building {} projects in parallel", batch.len());
                self.run_parallel(&batch, "build", &[]).await?;
            } else {
                for member in batch {
                    println!("🔨 Building {}", member.name);
                    self.run_in_member(member, "build", &[]).await?;
                }
            }
        }
        
        println!("✅ Workspace build completed");
        Ok(())
    }
    
    /// Test all workspace members
    pub async fn test_workspace(&self, parallel: bool) -> CyrusResult<()> {
        let workspace = self.get_current_workspace()?;
        let enabled_members: Vec<&WorkspaceMember> = workspace.members.iter()
            .filter(|m| m.enabled)
            .collect();
        
        println!("🧪 Testing workspace with {} members", enabled_members.len());
        
        if parallel {
            self.run_parallel(&enabled_members, "test", &[]).await?;
        } else {
            self.run_sequential(&enabled_members, "test", &[]).await?;
        }
        
        println!("✅ Workspace tests completed");
        Ok(())
    }
    
    /// Add a workspace script
    pub fn add_script(
        &mut self,
        name: String,
        script: WorkspaceScript,
    ) -> CyrusResult<()> {
        let workspace = self.get_current_workspace_mut()?;
        workspace.scripts.insert(name.clone(), script);
        workspace.updated_at = chrono::Utc::now();
        
        self.save_workspace(workspace, &workspace.root_path.clone())?;
        println!("✅ Added workspace script '{}'", name);
        Ok(())
    }
    
    /// Run a workspace script
    pub async fn run_script(&self, script_name: &str) -> CyrusResult<()> {
        let workspace = self.get_current_workspace()?;
        
        let script = workspace.scripts.get(script_name)
            .ok_or_else(|| CyrusError::Workspace {
                message: format!("Script '{}' not found", script_name),
            })?;
        
        let target_members: Vec<&WorkspaceMember> = if script.run_in_members.is_empty() {
            workspace.members.iter().filter(|m| m.enabled).collect()
        } else {
            workspace.members.iter()
                .filter(|m| script.run_in_members.contains(&m.name) && m.enabled)
                .collect()
        };
        
        println!("🚀 Running script '{}' in {} members", script_name, target_members.len());
        
        let command_parts: Vec<&str> = script.command.split_whitespace().collect();
        let command = command_parts[0];
        let args: Vec<String> = command_parts[1..].iter().map(|s| s.to_string()).collect();
        
        if script.run_parallel {
            self.run_parallel_with_error_handling(&target_members, command, &args, script.continue_on_error).await?;
        } else {
            self.run_sequential_with_error_handling(&target_members, command, &args, script.continue_on_error).await?;
        }
        
        Ok(())
    }
    
    /// Get workspace status
    pub fn get_status(&self) -> CyrusResult<WorkspaceStatus> {
        let workspace = self.get_current_workspace()?;
        
        let mut member_statuses = Vec::new();
        for member in &workspace.members {
            let member_path = workspace.root_path.join(&member.path);
            let status = MemberStatus {
                name: member.name.clone(),
                language: member.language.clone(),
                enabled: member.enabled,
                exists: member_path.exists(),
                has_cyrus_config: member_path.join("cyrus.toml").exists(),
                last_modified: self.get_last_modified(&member_path)?,
            };
            member_statuses.push(status);
        }
        
        Ok(WorkspaceStatus {
            name: workspace.name.clone(),
            root_path: workspace.root_path.clone(),
            total_members: workspace.members.len(),
            enabled_members: workspace.members.iter().filter(|m| m.enabled).count(),
            member_statuses,
        })
    }
    
    // Helper methods
    
    fn get_current_workspace(&self) -> CyrusResult<&Workspace> {
        self.current_workspace.as_ref()
            .ok_or_else(|| CyrusError::Workspace {
                message: "No workspace loaded".to_string(),
            })
    }
    
    fn get_current_workspace_mut(&mut self) -> CyrusResult<&mut Workspace> {
        self.current_workspace.as_mut()
            .ok_or_else(|| CyrusError::Workspace {
                message: "No workspace loaded".to_string(),
            })
    }
    
    fn save_workspace(&self, workspace: &Workspace, path: &Path) -> CyrusResult<()> {
        let workspace_file = path.join("cyrus-workspace.toml");
        let content = toml::to_string_pretty(workspace)
            .map_err(|e| CyrusError::Workspace {
                message: format!("Failed to serialize workspace config: {}", e),
            })?;
        
        std::fs::write(workspace_file, content)?;
        Ok(())
    }
    
    fn detect_project_language(&self, path: &Path) -> Option<String> {
        // Check for common language files
        if path.join("package.json").exists() {
            Some("javascript".to_string())
        } else if path.join("Cargo.toml").exists() {
            Some("rust".to_string())
        } else if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
            Some("python".to_string())
        } else if path.join("go.mod").exists() {
            Some("golang".to_string())
        } else if path.join("pom.xml").exists() || path.join("build.gradle").exists() {
            Some("java".to_string())
        } else if path.join("composer.json").exists() {
            Some("php".to_string())
        } else if path.join("Gemfile").exists() {
            Some("ruby".to_string())
        } else {
            None
        }
    }
    
    async fn run_parallel(
        &self,
        members: &[&WorkspaceMember],
        command: &str,
        args: &[String],
    ) -> CyrusResult<()> {
        let futures: Vec<_> = members.iter()
            .map(|member| self.run_in_member(member, command, args))
            .collect();
        
        let results = futures::future::join_all(futures).await;
        
        // Check for errors
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                eprintln!("❌ Failed in member '{}': {}", members[i].name, e);
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    async fn run_sequential(
        &self,
        members: &[&WorkspaceMember],
        command: &str,
        args: &[String],
    ) -> CyrusResult<()> {
        for member in members {
            println!("▶️  Running in {}", member.name);
            self.run_in_member(member, command, args).await?;
        }
        Ok(())
    }
    
    async fn run_parallel_with_error_handling(
        &self,
        members: &[&WorkspaceMember],
        command: &str,
        args: &[String],
        continue_on_error: bool,
    ) -> CyrusResult<()> {
        let futures: Vec<_> = members.iter()
            .map(|member| self.run_in_member(member, command, args))
            .collect();
        
        let results = futures::future::join_all(futures).await;
        
        let mut has_errors = false;
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                eprintln!("❌ Failed in member '{}': {}", members[i].name, e);
                has_errors = true;
                if !continue_on_error {
                    return Err(e);
                }
            }
        }
        
        if has_errors && !continue_on_error {
            return Err(CyrusError::Workspace {
                message: "Some workspace operations failed".to_string(),
            });
        }
        
        Ok(())
    }
    
    async fn run_sequential_with_error_handling(
        &self,
        members: &[&WorkspaceMember],
        command: &str,
        args: &[String],
        continue_on_error: bool,
    ) -> CyrusResult<()> {
        let mut has_errors = false;
        
        for member in members {
            println!("▶️  Running in {}", member.name);
            if let Err(e) = self.run_in_member(member, command, args).await {
                eprintln!("❌ Failed in member '{}': {}", member.name, e);
                has_errors = true;
                if !continue_on_error {
                    return Err(e);
                }
            }
        }
        
        if has_errors && !continue_on_error {
            return Err(CyrusError::Workspace {
                message: "Some workspace operations failed".to_string(),
            });
        }
        
        Ok(())
    }
    
    async fn run_in_member(
        &self,
        member: &WorkspaceMember,
        command: &str,
        args: &[String],
    ) -> CyrusResult<()> {
        let workspace = self.get_current_workspace()?;
        let member_path = workspace.root_path.join(&member.path);
        
        // Check if it's a Cyrus project
        if member_path.join("cyrus.toml").exists() {
            // Use cyrus run
            let mut cyrus_cmd = tokio::process::Command::new("cyrus");
            cyrus_cmd.arg("run")
                .arg(command)
                .args(args)
                .current_dir(&member_path);
            
            let status = cyrus_cmd.status().await?;
            if !status.success() {
                return Err(CyrusError::CommandFailed {
                    command: format!("cyrus run {} {}", command, args.join(" ")),
                    code: status.code(),
                });
            }
        } else {
            // Run command directly
            let mut cmd = tokio::process::Command::new(command);
            cmd.args(args)
                .current_dir(&member_path);
            
            let status = cmd.status().await?;
            if !status.success() {
                return Err(CyrusError::CommandFailed {
                    command: format!("{} {}", command, args.join(" ")),
                    code: status.code(),
                });
            }
        }
        
        Ok(())
    }
    
    fn calculate_build_order(&self, members: &[WorkspaceMember]) -> CyrusResult<Vec<Vec<&WorkspaceMember>>> {
        let mut build_order = Vec::new();
        let mut remaining: Vec<&WorkspaceMember> = members.iter().filter(|m| m.enabled).collect();
        let mut processed = std::collections::HashSet::new();
        
        while !remaining.is_empty() {
            let mut current_batch = Vec::new();
            let mut batch_processed = false;
            
            // Find members with no unresolved dependencies
            for (i, member) in remaining.iter().enumerate() {
                let dependencies_resolved = member.dependencies.iter()
                    .all(|dep| processed.contains(dep));
                
                if dependencies_resolved {
                    current_batch.push(*member);
                    processed.insert(member.name.clone());
                    batch_processed = true;
                }
            }
            
            if !batch_processed {
                return Err(CyrusError::Workspace {
                    message: "Circular dependency detected in workspace members".to_string(),
                });
            }
            
            // Remove processed members
            remaining.retain(|member| !processed.contains(&member.name));
            
            if !current_batch.is_empty() {
                build_order.push(current_batch);
            }
        }
        
        Ok(build_order)
    }
    
    fn get_last_modified(&self, path: &Path) -> CyrusResult<Option<chrono::DateTime<chrono::Utc>>> {
        if !path.exists() {
            return Ok(None);
        }
        
        let metadata = std::fs::metadata(path)?;
        let modified = metadata.modified()?;
        let datetime = chrono::DateTime::<chrono::Utc>::from(modified);
        Ok(Some(datetime))
    }
    
    /// Export workspace configuration
    pub fn export_workspace(&self) -> CyrusResult<String> {
        let workspace = self.get_current_workspace()?;
        toml::to_string_pretty(workspace)
            .map_err(|e| CyrusError::Workspace {
                message: format!("Failed to export workspace: {}", e),
            })
    }
    
    /// Import workspace configuration
    pub fn import_workspace(&mut self, config: &str, path: PathBuf) -> CyrusResult<()> {
        let mut workspace: Workspace = toml::from_str(config)
            .map_err(|e| CyrusError::Workspace {
                message: format!("Failed to parse workspace config: {}", e),
            })?;
        
        workspace.root_path = path.clone();
        workspace.updated_at = chrono::Utc::now();
        
        self.save_workspace(&workspace, &path)?;
        self.current_workspace = Some(workspace);
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct WorkspaceStatus {
    pub name: String,
    pub root_path: PathBuf,
    pub total_members: usize,
    pub enabled_members: usize,
    pub member_statuses: Vec<MemberStatus>,
}

#[derive(Debug)]
pub struct MemberStatus {
    pub name: String,
    pub language: String,
    pub enabled: bool,
    pub exists: bool,
    pub has_cyrus_config: bool,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Workspace commands implementation
pub struct WorkspaceCommands;

impl WorkspaceCommands {
    pub async fn init(
        name: String,
        description: Option<String>,
        path: Option<PathBuf>,
    ) -> CyrusResult<()> {
        let workspace_path = path.unwrap_or_else(|| std::env::current_dir().unwrap().join(&name));
        let desc = description.unwrap_or_else(|| format!("{} workspace", name));
        
        let mut manager = WorkspaceManager::new();
        manager.init_workspace(name, desc, workspace_path)?;
        
        Ok(())
    }
    
    pub async fn add_member(
        workspace_path: PathBuf,
        name: String,
        project_path: PathBuf,
        language: Option<String>,
        create: bool,
    ) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        manager.add_member(name, project_path, language, create).await?;
        
        Ok(())
    }
    
    pub async fn list_members(workspace_path: PathBuf) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        
        let members = manager.list_members()?;
        
        if members.is_empty() {
            println!("📭 No members in workspace");
            return Ok(());
        }
        
        println!("📋 Workspace Members:");
        for member in members {
            let status = if member.enabled { "✅" } else { "❌" };
            println!("  {} {} ({})", status, member.name, member.language);
            println!("    Path: {:?}", member.path);
            if !member.dependencies.is_empty() {
                println!("    Dependencies: {}", member.dependencies.join(", "));
            }
        }
        
        Ok(())
    }
    
    pub async fn run_command(
        workspace_path: PathBuf,
        command: String,
        args: Vec<String>,
        members: Option<Vec<String>>,
        parallel: bool,
    ) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        manager.run_in_workspace(&command, &args, members, parallel).await?;
        
        Ok(())
    }
    
    pub async fn build(workspace_path: PathBuf, parallel: bool) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        manager.build_workspace(parallel).await?;
        
        Ok(())
    }
    
    pub async fn test(workspace_path: PathBuf, parallel: bool) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        manager.test_workspace(parallel).await?;
        
        Ok(())
    }
    
    pub async fn status(workspace_path: PathBuf) -> CyrusResult<()> {
        let mut manager = WorkspaceManager::new();
        manager.load_workspace(&workspace_path)?;
        
        let status = manager.get_status()?;
        
        println!("📊 Workspace Status: {}", status.name);
        println!("📁 Root: {:?}", status.root_path);
        println!("📦 Members: {} total, {} enabled", status.total_members, status.enabled_members);
        println!();
        
        for member_status in &status.member_statuses {
            let enabled_icon = if member_status.enabled { "✅" } else { "❌" };
            let exists_icon = if member_status.exists { "📁" } else { "❓" };
            let config_icon = if member_status.has_cyrus_config { "⚙️" } else { "  " };
            
            println!("  {} {} {} {} ({})", 
                enabled_icon, exists_icon, config_icon, 
                member_status.name, member_status.language);
            
            if let Some(modified) = member_status.last_modified {
                println!("    Last modified: {}", modified.format("%Y-%m-%d %H:%M:%S UTC"));
            }
        }
        
        Ok(())
    }
}