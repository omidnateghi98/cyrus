//! Archive extraction utilities

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use tar::Archive;

pub fn extract_tar_gz(archive_path: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive_path)
        .context("Failed to open tar.gz archive")?;
    
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    
    archive.unpack(destination)
        .context("Failed to extract tar.gz archive")?;
    
    println!("📦 Extracted archive to {:?}", destination);
    Ok(())
}

pub fn extract_zip(archive_path: &Path, destination: &Path) -> Result<()> {
    // For now, use system unzip command
    // In a real implementation, you would use the zip crate
    
    let output = std::process::Command::new("unzip")
        .args(["-q", archive_path.to_str().unwrap(), "-d", destination.to_str().unwrap()])
        .output()
        .context("Failed to run unzip command")?;
    
    if !output.status.success() {
        anyhow::bail!("Failed to extract ZIP archive: {}", 
            String::from_utf8_lossy(&output.stderr));
    }
    
    println!("📦 Extracted ZIP archive to {:?}", destination);
    Ok(())
}

pub fn extract_archive(archive_path: &Path, destination: &Path) -> Result<()> {
    let extension = archive_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match extension {
        "gz" => extract_tar_gz(archive_path, destination),
        "zip" => extract_zip(archive_path, destination),
        _ => Err(anyhow::anyhow!("Unsupported archive format: {}", extension)),
    }
}
