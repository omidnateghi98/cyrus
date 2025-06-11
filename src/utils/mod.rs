//! Utility functions for Cyrus

pub mod downloader;
pub mod archive;
pub mod platform;

use anyhow::Result;
use std::path::Path;

pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn is_executable(path: &Path) -> bool {
    path.exists() && path.is_file()
}
