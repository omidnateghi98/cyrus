// src/error.rs
//! Enhanced error handling for Cyrus with detailed error types and recovery

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum CyrusError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Language '{language}' is not supported")]
    UnsupportedLanguage { language: String },

    #[error("Version '{version}' is not available for '{language}'")]
    UnsupportedVersion { language: String, version: String },

    #[error("Package manager '{package_manager}' is not supported for '{language}'")]
    UnsupportedPackageManager { language: String, package_manager: String },

    #[error("Project not found in '{path}'")]
    ProjectNotFound { path: PathBuf },

    #[error("Language '{language}' version '{version}' is not installed")]
    LanguageNotInstalled { language: String, version: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Download failed for '{url}': {reason}")]
    DownloadFailed { url: String, reason: String },

    #[error("Installation failed for '{language}' version '{version}': {reason}")]
    InstallationFailed { language: String, version: String, reason: String },

    #[error("Command '{command}' failed with exit code {code}")]
    CommandFailed { command: String, code: Option<i32> },

    #[error("Template '{template}' not found")]
    TemplateNotFound { template: String },

    #[error("Validation error: {errors:?}")]
    Validation { errors: Vec<ValidationError> },

    #[error("Plugin error: {message}")]
    Plugin { message: String },

    #[error("Workspace error: {message}")]
    Workspace { message: String },

    #[error("Alias error: {message}")]
    Alias { message: String },

    #[error("Environment error: {message}")]
    Environment { message: String },

    #[error("Security error: {message}")]
    Security { message: String },

    #[error("Version conflict: {message}")]
    VersionConflict { message: String },

    #[error("Dependency error: {message}")]
    Dependency { message: String },
}

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Invalid version format: '{version}'")]
    InvalidVersion { version: String },

    #[error("Missing required field: '{field}'")]
    MissingField { field: String },

    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },

    #[error("Circular dependency detected: {path}")]
    CircularDependency { path: String },

    #[error("Incompatible versions: {details}")]
    IncompatibleVersions { details: String },
}

#[derive(Debug, Clone)]
pub enum ValidationWarning {
    #[error("Consider using TypeScript for better development experience")]
    SuggestTypescript,

    #[error("Version '{version}' is outdated, consider upgrading to '{latest}'")]
    OutdatedVersion { version: String, latest: String },

    #[error("Package manager '{pm}' might be slower than '{suggested}'")]
    SuboptimalPackageManager { pm: String, suggested: String },

    #[error("Missing common development dependencies")]
    MissingDevDependencies { suggestions: Vec<String> },

    #[error("Security vulnerability detected in dependency '{dep}'")]
    SecurityVulnerability { dep: String },

    #[error("Large number of dependencies ({count}) might slow down installation")]
    TooManyDependencies { count: usize },
}

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationWarning::SuggestTypescript => write!(f, "Consider using TypeScript for better development experience"),
            ValidationWarning::OutdatedVersion { version, latest } => write!(f, "Version '{}' is outdated, consider upgrading to '{}'", version, latest),
            ValidationWarning::SuboptimalPackageManager { pm, suggested } => write!(f, "Package manager '{}' might be slower than '{}'", pm, suggested),
            ValidationWarning::MissingDevDependencies { suggestions } => write!(f, "Missing common development dependencies: {}", suggestions.join(", ")),
            ValidationWarning::SecurityVulnerability { dep } => write!(f, "Security vulnerability detected in dependency '{}'", dep),
            ValidationWarning::TooManyDependencies { count } => write!(f, "Large number of dependencies ({}) might slow down installation", count),
        }
    }
}

pub type Result<T> = std::result::Result<T, CyrusError>;

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    Retry { max_attempts: u32, base_delay_ms: u64 },
    /// Fall back to an alternative approach
    Fallback { alternative: String },
    /// Use cached data if available
    UseCache,
    /// Prompt user for manual intervention
    Manual { message: String },
    /// Skip the operation and continue
    Skip,
    /// Abort the entire operation
    Abort,
}

pub struct ErrorContext {
    pub operation: String,
    pub recovery_strategies: Vec<RecoveryStrategy>,
    pub user_facing_message: Option<String>,
    pub technical_details: Option<String>,
}

impl CyrusError {
    pub fn with_context(self, context: ErrorContext) -> ContextualError {
        ContextualError {
            error: self,
            context,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            CyrusError::Network { .. } | 
            CyrusError::DownloadFailed { .. } |
            CyrusError::CommandFailed { .. }
        )
    }

    pub fn suggest_recovery(&self) -> Vec<RecoveryStrategy> {
        match self {
            CyrusError::Network { .. } | CyrusError::DownloadFailed { .. } => {
                vec![
                    RecoveryStrategy::Retry { max_attempts: 3, base_delay_ms: 1000 },
                    RecoveryStrategy::UseCache,
                ]
            },
            CyrusError::CommandFailed { .. } => {
                vec![
                    RecoveryStrategy::Retry { max_attempts: 2, base_delay_ms: 500 },
                    RecoveryStrategy::Manual { message: "Please check if the command exists and try again".to_string() },
                ]
            },
            CyrusError::UnsupportedLanguage { language } => {
                vec![
                    RecoveryStrategy::Fallback { alternative: format!("Try using a supported language instead of '{}'", language) },
                    RecoveryStrategy::Manual { message: "Check 'cyrus languages' for supported languages".to_string() },
                ]
            },
            _ => vec![RecoveryStrategy::Abort],
        }
    }
}

#[derive(Debug)]
pub struct ContextualError {
    pub error: CyrusError,
    pub context: ErrorContext,
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(user_msg) = &self.context.user_facing_message {
            write!(f, "{}", user_msg)?;
        } else {
            write!(f, "{}", self.error)?;
        }

        if let Some(technical) = &self.context.technical_details {
            write!(f, "\nTechnical details: {}", technical)?;
        }

        Ok(())
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Helper macros for error handling
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err)
    };
}

/// Error recovery utilities
pub struct ErrorRecovery;

impl ErrorRecovery {
    pub async fn with_retry<F, T, E>(
        operation: F,
        strategy: RecoveryStrategy,
    ) -> std::result::Result<T, E>
    where
        F: Fn() -> std::result::Result<T, E>,
        E: std::fmt::Display,
    {
        match strategy {
            RecoveryStrategy::Retry { max_attempts, base_delay_ms } => {
                let mut attempts = 0;
                loop {
                    match operation() {
                        Ok(result) => return Ok(result),
                        Err(e) => {
                            attempts += 1;
                            if attempts >= max_attempts {
                                return Err(e);
                            }
                            let delay = base_delay_ms * 2_u64.pow(attempts - 1);
                            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                        }
                    }
                }
            }
            _ => operation(),
        }
    }

    pub fn format_user_friendly_error(error: &CyrusError) -> String {
        match error {
            CyrusError::UnsupportedLanguage { language } => {
                format!("❌ '{}' is not supported. Run 'cyrus languages' to see available languages.", language)
            },
            CyrusError::LanguageNotInstalled { language, version } => {
                format!("❌ {} {} is not installed. Run 'cyrus install {}{}'", language, version, language, version)
            },
            CyrusError::ProjectNotFound { path } => {
                format!("❌ No Cyrus project found in '{}'. Run 'cyrus init' to create one.", path.display())
            },
            CyrusError::Network { message } => {
                format!("🌐 Network error: {}. Please check your internet connection.", message)
            },
            CyrusError::CommandFailed { command, code } => {
                format!("⚠️  Command '{}' failed{}. Check the output above for details.", 
                    command, 
                    if let Some(c) = code { format!(" with exit code {}", c) } else { String::new() }
                )
            },
            _ => format!("❌ {}", error),
        }
    }
}