use serde::{Deserialize, Serialize};
use std::fmt;

/// Error categories for structured error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorCategory {
    /// User-caused errors: invalid input, missing files, unsupported formats
    User,
    /// System errors: disk full, permissions, DB corruption
    System,
    /// Algorithm errors: rater failure, unresolvable conflicts
    Algorithm,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCategory::User => write!(f, "user"),
            ErrorCategory::System => write!(f, "system"),
            ErrorCategory::Algorithm => write!(f, "algorithm"),
        }
    }
}

/// Error code for structured error identification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // User errors
    MissingFile,
    InvalidPath,
    UnsupportedFormat,
    InvalidInput,
    FileNotFound,

    // System errors
    PermissionDenied,
    DiskFull,
    DatabaseCorruption,
    OutOfMemory,
    IoError,

    // Algorithm errors
    UnresolvableConflict,
    RaterFailure,
    DataInvariantViolation,
    PairingFailure,

    // Generic
    Unknown,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ErrorCode {
    pub fn category(&self) -> ErrorCategory {
        match self {
            ErrorCode::MissingFile
            | ErrorCode::InvalidPath
            | ErrorCode::UnsupportedFormat
            | ErrorCode::InvalidInput
            | ErrorCode::FileNotFound => ErrorCategory::User,

            ErrorCode::PermissionDenied
            | ErrorCode::DiskFull
            | ErrorCode::DatabaseCorruption
            | ErrorCode::OutOfMemory
            | ErrorCode::IoError => ErrorCategory::System,

            ErrorCode::UnresolvableConflict
            | ErrorCode::RaterFailure
            | ErrorCode::DataInvariantViolation
            | ErrorCode::PairingFailure => ErrorCategory::Algorithm,

            ErrorCode::Unknown => ErrorCategory::System,
        }
    }
}

/// Structured error details for logging and UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error category: user, system, or algorithm
    pub category: ErrorCategory,

    /// Error code for identification
    pub code: ErrorCode,

    /// Technical error message for logs
    pub message: String,

    /// User-friendly message for UI display
    pub user_message: String,

    /// Recovery suggestion for user
    pub recovery: String,

    /// Operation context (optional)
    pub context: serde_json::Value,

    /// Stack trace (optional, for algorithm errors)
    pub backtrace: Option<String>,
}

impl ErrorDetails {
    pub fn new(
        code: ErrorCode,
        message: impl Into<String>,
        user_message: impl Into<String>,
        recovery: impl Into<String>,
    ) -> Self {
        Self {
            category: code.category(),
            code,
            message: message.into(),
            user_message: user_message.into(),
            recovery: recovery.into(),
            context: serde_json::json!({}),
            backtrace: None,
        }
    }

    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    pub fn with_backtrace(mut self, backtrace: String) -> Self {
        self.backtrace = Some(backtrace);
        self
    }
}

/// Result type for operations that can fail with structured errors
pub type PhotorgResult<T> = Result<T, ErrorDetails>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_category_user() {
        assert_eq!(
            ErrorCode::MissingFile.category(),
            ErrorCategory::User
        );
    }

    #[test]
    fn test_error_code_category_system() {
        assert_eq!(
            ErrorCode::PermissionDenied.category(),
            ErrorCategory::System
        );
    }

    #[test]
    fn test_error_code_category_algorithm() {
        assert_eq!(
            ErrorCode::UnresolvableConflict.category(),
            ErrorCategory::Algorithm
        );
    }

    #[test]
    fn test_error_details_serialization() {
        let error = ErrorDetails::new(
            ErrorCode::MissingFile,
            "File not found: IMG_001.CR3",
            "Could not find the image file.",
            "Check the folder path and try again.",
        );

        let json = serde_json::to_string(&error).expect("Failed to serialize");
        assert!(json.contains("\"code\":\"MissingFile\""));
        assert!(json.contains("\"category\":\"user\""));
    }

    #[test]
    fn test_error_details_with_context() {
        let error = ErrorDetails::new(
            ErrorCode::IoError,
            "Permission denied",
            "Cannot write to file.",
            "Check file permissions.",
        )
        .with_context(serde_json::json!({
            "path": "/some/path/file.xmp"
        }));

        assert_eq!(error.context["path"], "/some/path/file.xmp");
    }
}
