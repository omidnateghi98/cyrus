//! Download utility for language installations

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio_stream::StreamExt;

pub async fn download_file(url: &str, destination: &Path) -> Result<()> {
    println!("🌐 Downloading from: {}", url);
    
    let client = reqwest::Client::new();
    let response = client.get(url).send().await
        .context("Failed to send download request")?;
    
    let total_size = response.content_length().unwrap_or(0);
    
    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    // Create destination file
    let mut file = File::create(destination)
        .context("Failed to create destination file")?;
    
    // Download with progress
    let mut stream = response.bytes_stream();
    let mut downloaded = 0u64;
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Error downloading chunk")?;
        file.write_all(&chunk)
            .context("Error writing to file")?;
        
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message("✅ Download completed");
    Ok(())
}

pub async fn download_with_retries(url: &str, destination: &Path, max_retries: u32) -> Result<()> {
    let mut attempts = 0;
    
    while attempts < max_retries {
        match download_file(url, destination).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                println!("⚠️  Download failed, retrying... ({}/{})", attempts, max_retries);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed to download after {} retries", max_retries))
}
