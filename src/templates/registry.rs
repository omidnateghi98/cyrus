// src/templates/registry.rs
use anyhow::Result;
use crate::templates::{TemplateInfo, ProjectTemplate, TemplateSource, TemplateCategory, DifficultyLevel};

pub struct TemplateRegistry {
    // In a real implementation, this might cache templates or connect to a remote registry
    cache: std::collections::HashMap<String, ProjectTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    pub async fn get_template(&self, name: &str) -> Result<Option<ProjectTemplate>> {
        // Check cache first
        if let Some(template) = self.cache.get(name) {
            return Ok(Some(template.clone()));
        }

        // In a real implementation, this would:
        // 1. Check local template registry
        // 2. Query remote template registry (like npm registry but for templates)
        // 3. Download and cache the template

        // For now, return None (template not found in registry)
        Ok(None)
    }

    pub async fn list_templates(&self) -> Result<Vec<TemplateInfo>> {
        // In a real implementation, this would return templates from:
        // 1. Local registry cache
        // 2. Remote registry metadata

        // For now, return empty list since we don't have a real registry yet
        Ok(vec![])
    }

    pub async fn search_templates(&self, query: &str) -> Result<Vec<TemplateInfo>> {
        let all_templates = self.list_templates().await?;

        let query_lower = query.to_lowercase();
        Ok(all_templates.into_iter()
            .filter(|template| {
                template.name.to_lowercase().contains(&query_lower) ||
                    template.description.to_lowercase().contains(&query_lower)
            })
            .collect())
    }

    pub async fn add_template(&mut self, template: ProjectTemplate) -> Result<()> {
        // Add template to local cache/registry
        self.cache.insert(template.name.clone(), template);
        Ok(())
    }

    pub async fn remove_template(&mut self, name: &str) -> Result<bool> {
        Ok(self.cache.remove(name).is_some())
    }

    pub async fn update_template(&mut self, template: ProjectTemplate) -> Result<()> {
        // Update existing template
        self.cache.insert(template.name.clone(), template);
        Ok(())
    }

    pub fn get_template_count(&self) -> usize {
        self.cache.len()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    // Helper method to create sample registry templates
    pub fn with_sample_templates() -> Self {
        let mut registry = Self::new();

        // Add some sample community templates
        let sample_templates = vec![
            TemplateInfo {
                name: "community-react-native".to_string(),
                description: "React Native mobile app template".to_string(),
                category: TemplateCategory::Mobile,
                difficulty: DifficultyLevel::Intermediate,
                language: "javascript".to_string(),
                source: TemplateSource::Registry,
            },
            TemplateInfo {
                name: "community-django-rest".to_string(),
                description: "Django REST API template".to_string(),
                category: TemplateCategory::Api,
                difficulty: DifficultyLevel::Advanced,
                language: "python".to_string(),
                source: TemplateSource::Registry,
            },
        ];

        // In a real implementation, these would be converted to full ProjectTemplate objects
        // and added to the registry

        registry
    }
}