use thiserror::Error;

/// Main error type for the convcom application
#[derive(Error, Debug)]
pub enum ConvComError {
    /// Configuration related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Git operation errors
    #[error("Git error: {0}")]
    GitError(String),

    /// HTTP client initialization error
    #[error("HTTP client error: {0}")]
    HttpClientError(String),

    /// API request error
    #[error("API request failed: {0}")]
    ApiRequestError(String),

    /// API returned an error response
    #[error("API error {status_code}: {message}")]
    ApiError { status_code: u16, message: String },

    /// Failed to parse API response
    #[error("Failed to parse API response: {0}")]
    ResponseParseError(String),

    /// API returned empty response
    #[error("API returned empty response")]
    EmptyResponseError,

    /// Environment variable not found
    #[error("Environment variable not found: {0}")]
    EnvVarError(String),

    /// IO operation failed
    #[error("IO error: {0}")]
    IoError(String),

    /// Template processing error
    #[error("Template error: {0}")]
    TemplateError(String),

    /// No staged files found
    #[error("No staged files found")]
    NoStagedFilesError,

    /// Not in a git repository
    #[error("Not in a git repository")]
    NotGitRepoError,
}

impl From<std::io::Error> for ConvComError {
    fn from(error: std::io::Error) -> Self {
        ConvComError::IoError(error.to_string())
    }
}

impl From<git2::Error> for ConvComError {
    fn from(error: git2::Error) -> Self {
        ConvComError::GitError(error.to_string())
    }
}

impl From<dotenvy::Error> for ConvComError {
    fn from(error: dotenvy::Error) -> Self {
        ConvComError::EnvVarError(error.to_string())
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, ConvComError>;
