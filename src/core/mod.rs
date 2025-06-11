//! Core functionality for Cyrus
//! 
//! This module contains the main CyrusCore struct and fundamental
//! operations for the language management system.

use anyhow::{Context, Result};
use std::path::PathBuf;
use dirs;

pub mod environment;
pub mod project;

pub use environment::Environment;
pub use project::Project;

/// Main Cyrus core structure
#[derive(Debug)]
pub struct CyrusCore {
    pub home_dir: PathBuf,
    pub cyrus_dir: PathBuf,
    pub config_dir: PathBuf,
    pub languages_dir: PathBuf,
}

impl CyrusCore {
    /// Initialize a new Cyrus core instance
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .context("Unable to determine home directory")?;
        
        let cyrus_dir = home_dir.join(".cyrus");
        let config_dir = cyrus_dir.join("config");
        let languages_dir = cyrus_dir.join("languages");

        // Create directories if they don't exist
        std::fs::create_dir_all(&cyrus_dir)?;
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&languages_dir)?;

        Ok(Self {
            home_dir,
            cyrus_dir,
            config_dir,
            languages_dir,
        })
    }

    /// Get the path for a specific language installation
    pub fn language_path(&self, language: &str, version: &str) -> PathBuf {
        self.languages_dir.join(language).join(version)
    }

    /// Check if a language version is installed
    pub fn is_language_installed(&self, language: &str, version: &str) -> bool {
        self.language_path(language, version).exists()
    }
}
