//! Language installer implementations

use anyhow::Result;
use std::path::Path;

pub mod python_installer;
pub mod javascript_installer;
pub mod golang_installer;

pub trait LanguageInstaller {
    async fn install(&self, version: &str, install_path: &Path) -> Result<()>;
    async fn verify_installation(&self, install_path: &Path) -> Result<bool>;
}
