// Error type abstraction: codes, categories, and details
pub mod category;
pub mod code;

pub use category::ErrorCategory;
pub use code::ErrorCode;

use serde::{Deserialize, Serialize};

/// Structured error details for logging and frontend display.
///
/// Contains all information needed for:
/// - Logging: technical message, context, backtrace
/// - Frontend UI: user message, recovery suggestions
/// - Error tracking: category for automatic handling
///
/// Single Responsibility: Carries error information; doesn't determine UI treatment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error category: user, system, or algorithm (auto-determined by code)
    pub category: ErrorCategory,

    /// Error code for identification and filtering
    pub code: ErrorCode,

    /// Technical error message for logs and support
    pub message: String,

    /// User-friendly message for UI display (never technical jargon)
    pub user_message: String,

    /// Recovery suggestion for user (actionable steps)
    pub recovery: String,

    /// Operation context (optional; what were you doing when this failed?)
    pub context: serde_json::Value,

    /// Stack trace (optional; only for algorithm errors in debug builds)
    pub backtrace: Option<String>,
}

impl ErrorDetails {
    /// Create a new error with automatic category determination.
    ///
    /// Category is computed from the error code; caller doesn't choose it.
    /// This ensures consistent categorization across the codebase.
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

    /// Attach operation context for better diagnostics.
    ///
    /// Example: `error.with_context(json!({"file": "IMG_001.CR3", "size_mb": 42}))`
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    /// Attach stack trace for algorithm errors (debug builds only).
    pub fn with_backtrace(mut self, backtrace: String) -> Self {
        self.backtrace = Some(backtrace);
        self
    }
}

/// Result type for operations that can fail with structured errors.
///
/// Preferred over anyhow::Result or generic std::result because it
/// ensures all errors go through our categorization and logging system.
pub type PhotorgResult<T> = Result<T, ErrorDetails>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_details_new() {
        let error = ErrorDetails::new(
            ErrorCode::MissingFile,
            "File not found: IMG_001.CR3",
            "Could not find the image file.",
            "Check the folder path and try again.",
        );

        assert_eq!(error.code, ErrorCode::MissingFile);
        assert_eq!(error.category, ErrorCategory::User);
        assert_eq!(error.message, "File not found: IMG_001.CR3");
        assert_eq!(error.user_message, "Could not find the image file.");
        assert_eq!(error.recovery, "Check the folder path and try again.");
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
        assert!(json.contains("\"user_message\":\"Could not find the image file.\""));
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

    #[test]
    fn test_error_details_with_backtrace() {
        let error = ErrorDetails::new(
            ErrorCode::RaterFailure,
            "Rater panic",
            "Image processing failed.",
            "Try with a different image.",
        )
        .with_backtrace("thread panicked at line 42".to_string());

        assert!(error.backtrace.is_some());
        assert_eq!(
            error.backtrace.as_ref().unwrap(),
            "thread panicked at line 42"
        );
    }

    #[test]
    fn test_photorg_result_type_alias() {
        let ok: PhotorgResult<i32> = Ok(42);
        let err: PhotorgResult<i32> = Err(ErrorDetails::new(
            ErrorCode::Unknown,
            "test error",
            "test user message",
            "test recovery",
        ));

        assert!(ok.is_ok());
        assert!(err.is_err());
    }
}
