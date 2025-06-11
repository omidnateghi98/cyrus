// src/templates/mod.rs
//! Advanced project template system with Git integration and custom templates

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use handlebars::Handlebars;
use crate::error::CyrusError;

pub mod builtin;
pub mod git;
pub mod registry;

use builtin::*;
use git::GitTemplateSource;
use registry::TemplateRegistry;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub language: String,
    pub language_version: String,
    pub package_manager: String,
    
    /// Template metadata
    pub metadata: TemplateMetadata,
    
    /// Files to create with their content templates
    pub files: HashMap<String, String>,
    
    /// Dependencies to install
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    
    /// Scripts to add to project
    pub scripts: HashMap<String, String>,
    
    /// Custom aliases for this template
    pub aliases: HashMap<String, String>,
    
    /// Environment variables
    pub environment: HashMap<String, String>,
    
    /// Commands to run after project creation
    pub post_install_commands: Vec<PostInstallCommand>,
    
    /// Template variables and their defaults
    pub variables: HashMap<String, TemplateVariable>,
    
    /// Conditional features
    pub features: Vec<TemplateFeature>,
    
    /// Template hooks
    pub hooks: TemplateHooks,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateMetadata {
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub difficulty: DifficultyLevel,
    pub min_cyrus_version: String,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TemplateCategory {
    Web,
    Api,
    Cli,
    Library,
    Desktop,
    Mobile,
    Game,
    DataScience,
    MachineLearning,
    Blockchain,
    IoT,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateVariable {
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub variable_type: VariableType,
    pub validation: Option<String>, // Regex pattern
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Choice(Vec<String>),
    Path,
    Url,
    Email,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateFeature {
    pub name: String,
    pub description: String,
    pub enabled_by_default: bool,
    pub dependencies: Vec<String>,
    pub files: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
    pub post_install_commands: Vec<PostInstallCommand>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostInstallCommand {
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub condition: Option<String>, // JavaScript expression
    pub ignore_failure: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateHooks {
    pub pre_create: Vec<String>,
    pub post_create: Vec<String>,
    pub pre_install: Vec<String>,
    pub post_install: Vec<String>,
}

impl Default for TemplateHooks {
    fn default() -> Self {
        Self {
            pre_create: vec![],
            post_create: vec![],
            pre_install: vec![],
            post_install: vec![],
        }
    }
}

pub struct TemplateManager {
    registry: TemplateRegistry,
    handlebars: Handlebars<'static>,
    git_source: GitTemplateSource,
}

impl TemplateManager {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // Register custom helpers
        handlebars.register_helper("upper", Box::new(upper_helper));
        handlebars.register_helper("lower", Box::new(lower_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("current_year", Box::new(current_year_helper));
        
        Ok(Self {
            registry: TemplateRegistry::new(),
            handlebars,
            git_source: GitTemplateSource::new(),
        })
    }
    
    pub async fn create_project(
        &self,
        template_name: &str,
        project_name: &str,
        project_path: &Path,
        variables: HashMap<String, String>,
        features: Vec<String>,
    ) -> Result<()> {
        // Get template
        let template = self.get_template(template_name).await?;
        
        // Validate inputs
        self.validate_template_inputs(&template, &variables)?;
        
        // Create project directory
        std::fs::create_dir_all(project_path)?;
        
        // Prepare template context
        let mut context = self.create_template_context(project_name, &variables)?;
        
        // Execute pre-create hooks
        self.execute_hooks(&template.hooks.pre_create, project_path, &context).await?;
        
        // Create base files
        self.create_files_from_template(&template, project_path, &context).await?;
        
        // Apply selected features
        for feature_name in &features {
            self.apply_feature(&template, feature_name, project_path, &context).await?;
        }
        
        // Create cyrus.toml
        self.create_cyrus_config(&template, project_name, project_path, &features).await?;
        
        // Execute post-create hooks
        self.execute_hooks(&template.hooks.post_create, project_path, &context).await?;
        
        // Run post-install commands
        for command in &template.post_install_commands {
            self.execute_post_install_command(command, project_path, &context).await?;
        }
        
        // Apply enabled features' post-install commands
        for feature_name in &features {
            if let Some(feature) = template.features.iter().find(|f| f.name == *feature_name) {
                for command in &feature.post_install_commands {
                    self.execute_post_install_command(command, project_path, &context).await?;
                }
            }
        }
        
        println!("✅ Created {} project: {}", template_name, project_name);
        Ok(())
    }
    
    async fn get_template(&self, name: &str) -> Result<ProjectTemplate> {
        // Try built-in templates first
        if let Some(template) = self.get_builtin_template(name) {
            return Ok(template);
        }
        
        // Try registry
        if let Some(template) = self.registry.get_template(name).await? {
            return Ok(template);
        }
        
        // Try Git source
        if name.starts_with("git:") || name.contains("://") {
            return self.git_source.fetch_template(name).await;
        }
        
        Err(CyrusError::TemplateNotFound { 
            template: name.to_string() 
        })
    }
    
    fn get_builtin_template(&self, name: &str) -> Option<ProjectTemplate> {
        match name {
            "react-typescript" => Some(create_react_typescript_template()),
            "rust-cli" => Some(create_rust_cli_template()),
            "python-api" => Some(create_python_api_template()),
            "node-express" => Some(create_node_express_template()),
            "vue-typescript" => Some(create_vue_typescript_template()),
            "rust-web" => Some(create_rust_web_template()),
            "python-ml" => Some(create_python_ml_template()),
            "go-api" => Some(create_go_api_template()),
            "java-spring" => Some(create_java_spring_template()),
            "python-cli" => Some(create_python_cli_template()),
            _ => None,
        }
    }
    
    fn create_template_context(
        &self,
        project_name: &str,
        variables: &HashMap<String, String>,
    ) -> Result<serde_json::Value> {
        let mut context = serde_json::Map::new();
        
        // Built-in variables
        context.insert("project_name".to_string(), serde_json::Value::String(project_name.to_string()));
        context.insert("project_name_snake".to_string(), serde_json::Value::String(to_snake_case(project_name)));
        context.insert("project_name_camel".to_string(), serde_json::Value::String(to_camel_case(project_name)));
        context.insert("project_name_pascal".to_string(), serde_json::Value::String(to_pascal_case(project_name)));
        context.insert("project_name_kebab".to_string(), serde_json::Value::String(to_kebab_case(project_name)));
        context.insert("current_year".to_string(), serde_json::Value::String(chrono::Utc::now().year().to_string()));
        context.insert("author".to_string(), serde_json::Value::String(self.get_git_user().unwrap_or_else(|| "Developer".to_string())));
        context.insert("email".to_string(), serde_json::Value::String(self.get_git_email().unwrap_or_else(|| "developer@example.com".to_string())));
        
        // User variables
        for (key, value) in variables {
            context.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        
        Ok(serde_json::Value::Object(context))
    }
    
    async fn create_files_from_template(
        &self,
        template: &ProjectTemplate,
        project_path: &Path,
        context: &serde_json::Value,
    ) -> Result<()> {
        for (file_path, content_template) in &template.files {
            let rendered_path = self.handlebars.render_template(file_path, context)?;
            let rendered_content = self.handlebars.render_template(content_template, context)?;
            
            let full_path = project_path.join(&rendered_path);
            
            // Create parent directories
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            std::fs::write(full_path, rendered_content)?;
        }
        
        Ok(())
    }
    
    async fn apply_feature(
        &self,
        template: &ProjectTemplate,
        feature_name: &str,
        project_path: &Path,
        context: &serde_json::Value,
    ) -> Result<()> {
        let feature = template.features.iter()
            .find(|f| f.name == feature_name)
            .ok_or_else(|| CyrusError::Config { 
                message: format!("Feature '{}' not found in template", feature_name) 
            })?;
        
        // Create feature files
        for (file_path, content_template) in &feature.files {
            let rendered_path = self.handlebars.render_template(file_path, context)?;
            let rendered_content = self.handlebars.render_template(content_template, context)?;
            
            let full_path = project_path.join(&rendered_path);
            
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            std::fs::write(full_path, rendered_content)?;
        }
        
        Ok(())
    }
    
    async fn create_cyrus_config(
        &self,
        template: &ProjectTemplate,
        project_name: &str,
        project_path: &Path,
        features: &[String],
    ) -> Result<()> {
        let mut project_config = crate::core::Project::new(
            project_name.to_string(),
            template.language.clone(),
            template.language_version.clone(),
            template.package_manager.clone(),
        );
        
        project_config.dependencies = template.dependencies.clone();
        project_config.dev_dependencies = template.dev_dependencies.clone();
        project_config.scripts = template.scripts.clone();
        project_config.custom_aliases = template.aliases.clone();
        project_config.environment = template.environment.clone();
        
        // Add feature dependencies
        for feature_name in features {
            if let Some(feature) = template.features.iter().find(|f| f.name == *feature_name) {
                project_config.dependencies.extend_from_slice(&feature.dependencies);
                for (script_name, script_cmd) in &feature.scripts {
                    project_config.scripts.insert(script_name.clone(), script_cmd.clone());
                }
            }
        }
        
        project_config.save_to_file(&project_path.join("cyrus.toml"))?;
        Ok(())
    }
    
    async fn execute_hooks(
        &self,
        hooks: &[String],
        project_path: &Path,
        context: &serde_json::Value,
    ) -> Result<()> {
        for hook in hooks {
            let rendered_hook = self.handlebars.render_template(hook, context)?;
            let parts: Vec<&str> = rendered_hook.split_whitespace().collect();
            
            if let Some(command) = parts.first() {
                let args = &parts[1..];
                let output = tokio::process::Command::new(command)
                    .args(args)
                    .current_dir(project_path)
                    .output()
                    .await?;
                
                if !output.status.success() {
                    return Err(CyrusError::CommandFailed {
                        command: rendered_hook,
                        code: output.status.code(),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    async fn execute_post_install_command(
        &self,
        command: &PostInstallCommand,
        project_path: &Path,
        context: &serde_json::Value,
    ) -> Result<()> {
        let rendered_command = self.handlebars.render_template(&command.command, context)?;
        let mut rendered_args = Vec::new();
        
        for arg in &command.args {
            rendered_args.push(self.handlebars.render_template(arg, context)?);
        }
        
        let working_dir = if let Some(wd) = &command.working_directory {
            project_path.join(self.handlebars.render_template(wd, context)?)
        } else {
            project_path.to_path_buf()
        };
        
        let output = tokio::process::Command::new(rendered_command.clone())
            .args(&rendered_args)
            .current_dir(&working_dir)
            .output()
            .await?;
        
        if !output.status.success() && !command.ignore_failure {
            return Err(CyrusError::CommandFailed {
                command: format!("{} {}", rendered_command, rendered_args.join(" ")),
                code: output.status.code(),
            });
        }
        
        Ok(())
    }
    
    fn validate_template_inputs(
        &self,
        template: &ProjectTemplate,
        variables: &HashMap<String, String>,
    ) -> Result<()> {
        for (var_name, var_def) in &template.variables {
            if var_def.required && !variables.contains_key(var_name) {
                return Err(CyrusError::Config {
                    message: format!("Required variable '{}' not provided", var_name),
                });
            }
            
            if let Some(value) = variables.get(var_name) {
                self.validate_variable_value(var_name, value, var_def)?;
            }
        }
        
        Ok(())
    }
    
    fn validate_variable_value(
        &self,
        name: &str,
        value: &str,
        definition: &TemplateVariable,
    ) -> Result<()> {
        match &definition.variable_type {
            VariableType::String => {
                // Basic string validation
                if value.is_empty() && definition.required {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' cannot be empty", name),
                    });
                }
            },
            VariableType::Number => {
                if value.parse::<f64>().is_err() {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' must be a number", name),
                    });
                }
            },
            VariableType::Boolean => {
                if !matches!(value.to_lowercase().as_str(), "true" | "false" | "yes" | "no" | "1" | "0") {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' must be a boolean value", name),
                    });
                }
            },
            VariableType::Choice(choices) => {
                if !choices.contains(&value.to_string()) {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' must be one of: {}", name, choices.join(", ")),
                    });
                }
            },
            VariableType::Path => {
                // Basic path validation
                if value.contains("..") || value.starts_with('/') {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' contains invalid path characters", name),
                    });
                }
            },
            VariableType::Url => {
                if url::Url::parse(value).is_err() {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' must be a valid URL", name),
                    });
                }
            },
            VariableType::Email => {
                if !value.contains('@') || !value.contains('.') {
                    return Err(CyrusError::Config {
                        message: format!("Variable '{}' must be a valid email address", name),
                    });
                }
            },
        }
        
        // Regex validation if provided
        if let Some(pattern) = &definition.validation {
            let regex = regex::Regex::new(pattern)
                .map_err(|_| CyrusError::Config {
                    message: format!("Invalid regex pattern for variable '{}'", name),
                })?;
            
            if !regex.is_match(value) {
                return Err(CyrusError::Config {
                    message: format!("Variable '{}' does not match required pattern", name),
                });
            }
        }
        
        Ok(())
    }
    
    pub async fn list_templates(&self) -> Result<Vec<TemplateInfo>> {
        let mut templates = Vec::new();
        
        // Built-in templates
        let builtin_names = vec![
            "react-typescript", "rust-cli", "python-api", "node-express",
            "vue-typescript", "rust-web", "python-ml", "go-api", 
            "java-spring", "python-cli"
        ];
        
        for name in builtin_names {
            if let Some(template) = self.get_builtin_template(name) {
                templates.push(TemplateInfo {
                    name: template.name,
                    description: template.description,
                    category: template.metadata.category,
                    difficulty: template.metadata.difficulty,
                    language: template.language,
                    source: TemplateSource::Builtin,
                });
            }
        }
        
        // Registry templates
        let registry_templates = self.registry.list_templates().await?;
        templates.extend(registry_templates);
        
        Ok(templates)
    }
    
    pub async fn search_templates(&self, query: &str) -> Result<Vec<TemplateInfo>> {
        let all_templates = self.list_templates().await?;
        
        let query_lower = query.to_lowercase();
        Ok(all_templates.into_iter()
            .filter(|template| {
                template.name.to_lowercase().contains(&query_lower) ||
                template.description.to_lowercase().contains(&query_lower) ||
                template.language.to_lowercase().contains(&query_lower)
            })
            .collect())
    }
    
    fn get_git_user(&self) -> Option<String> {
        std::process::Command::new("git")
            .args(["config", "--global", "user.name"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
    }
    
    fn get_git_email(&self) -> Option<String> {
        std::process::Command::new("git")
            .args(["config", "--global", "user.email"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub difficulty: DifficultyLevel,
    pub language: String,
    pub source: TemplateSource,
}

#[derive(Debug, Clone)]
pub enum TemplateSource {
    Builtin,
    Registry,
    Git(String),
    Local(PathBuf),
}

// Helper functions for template rendering
fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
        .replace('-', "_")
        .replace(' ', "_")
}

fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = s.split(&['-', '_', ' '][..]).collect();
    let mut result = String::new();
    
    for (i, word) in words.iter().enumerate() {
        if i == 0 {
            result.push_str(&word.to_lowercase());
        } else {
            result.push_str(&capitalize_first(word));
        }
    }
    
    result
}

fn to_pascal_case(s: &str) -> String {
    let words: Vec<&str> = s.split(&['-', '_', ' '][..]).collect();
    words.iter()
        .map(|word| capitalize_first(word))
        .collect::<String>()
}

fn to_kebab_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!("-{}", c.to_lowercase())
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
        .replace('_', "-")
        .replace(' ', "-")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Handlebars helpers
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

fn upper_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"upper\""))?;
    
    let rendered = param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?
        .to_uppercase();
    
    out.write(&rendered)?;
    Ok(())
}

fn lower_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"lower\""))?;
    
    let rendered = param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?
        .to_lowercase();
    
    out.write(&rendered)?;
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"snake_case\""))?;
    
    let rendered = to_snake_case(param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?);
    
    out.write(&rendered)?;
    Ok(())
}

fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"camel_case\""))?;
    
    let rendered = to_camel_case(param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?);
    
    out.write(&rendered)?;
    Ok(())
}

fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"pascal_case\""))?;
    
    let rendered = to_pascal_case(param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?);
    
    out.write(&rendered)?;
    Ok(())
}

fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Param not found for helper \"kebab_case\""))?;
    
    let rendered = to_kebab_case(param.value().as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?);
    
    out.write(&rendered)?;
    Ok(())
}

fn current_year_helper(
    _: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let year = chrono::Utc::now().year().to_string();
    out.write(&year)?;
    Ok(())
}