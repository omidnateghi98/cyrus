// src/templates/git.rs
use anyhow::Result;
use crate::templates::ProjectTemplate;
use std::path::Path;

pub struct GitTemplateSource;

impl GitTemplateSource {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_template(&self, url: &str) -> Result<ProjectTemplate> {
        // Parse the Git URL
        let clean_url = if url.starts_with("git:") {
            url.strip_prefix("git:").unwrap_or(url)
        } else {
            url
        };

        // Create temporary directory for cloning
        let temp_dir = tempfile::tempdir()?;

        // Clone the repository
        let output = tokio::process::Command::new("git")
            .args(["clone", clean_url, temp_dir.path().to_str().unwrap()])
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to clone Git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Look for template configuration
        let template_file = temp_dir.path().join("cyrus-template.toml");
        if !template_file.exists() {
            anyhow::bail!("No cyrus-template.toml found in Git repository");
        }

        // Load template configuration
        let content = std::fs::read_to_string(&template_file)?;
        let template: ProjectTemplate = toml::from_str(&content)?;

        // TODO: Load template files from the repository
        // This would involve reading all the template files and populating
        // the template.files HashMap

        Ok(template)
    }

    async fn clone_repository(&self, url: &str, target_dir: &Path) -> Result<()> {
        let output = tokio::process::Command::new("git")
            .args(["clone", url, target_dir.to_str().unwrap()])
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to clone repository: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }
}