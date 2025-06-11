//! Platform-specific utilities

use std::env;

#[derive(Debug, Clone)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

impl Platform {
    pub fn current() -> Self {
        match env::consts::OS {
            "windows" => Platform::Windows,
            "macos" => Platform::MacOS,
            "linux" => Platform::Linux,
            _ => Platform::Unknown,
        }
    }
    
    pub fn executable_extension(&self) -> &str {
        match self {
            Platform::Windows => ".exe",
            _ => "",
        }
    }
    
    pub fn library_extension(&self) -> &str {
        match self {
            Platform::Windows => ".dll",
            Platform::MacOS => ".dylib",
            _ => ".so",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Architecture {
    X64,
    X86,
    Arm64,
    Unknown,
}

impl Architecture {
    pub fn current() -> Self {
        match env::consts::ARCH {
            "x86_64" => Architecture::X64,
            "x86" => Architecture::X86,
            "aarch64" => Architecture::Arm64,
            _ => Architecture::Unknown,
        }
    }
}
