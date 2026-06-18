/// Error handler for error display feature.
///
/// Single Responsibility: Determine how to display errors based on category and code.
/// This handler is stateless; it's a pure function from error details to UI instructions.

use crate::infrastructure::error::{ErrorCategory, ErrorDetails};
use serde::{Deserialize, Serialize};

/// Instructions for frontend on how to display an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDisplay {
    /// Title for the error dialog
    pub title: String,

    /// User-friendly message (never shows stack traces or technical jargon)
    pub message: String,

    /// Recovery suggestions (actionable steps the user can take)
    pub recovery: String,

    /// Display style: "error", "warning", "info"
    pub style: String,

    /// Whether to show additional technical details (debug builds only)
    pub show_details: bool,

    /// Technical message (only for debug/logs)
    pub technical_message: Option<String>,
}

/// Convert an error into display instructions.
pub struct ErrorHandler;

impl ErrorHandler {
    /// Convert a backend error into frontend display instructions.
    pub fn handle_error(error: &ErrorDetails) -> ErrorDisplay {
        let (title, style, show_details) = match error.category {
            ErrorCategory::User => (
                "Can't proceed".to_string(),
                "error".to_string(),
                false, // Never show details for user errors
            ),
            ErrorCategory::System => (
                "Something went wrong".to_string(),
                "warning".to_string(),
                cfg!(debug_assertions), // Show details only in debug
            ),
            ErrorCategory::Algorithm => (
                "Processing failed".to_string(),
                "error".to_string(),
                cfg!(debug_assertions), // Show details only in debug
            ),
        };

        ErrorDisplay {
            title,
            message: error.user_message.clone(),
            recovery: error.recovery.clone(),
            style,
            show_details,
            technical_message: if show_details {
                Some(error.message.clone())
            } else {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::error::ErrorCode;

    #[test]
    fn test_user_error_handling() {
        let error = ErrorDetails::new(
            ErrorCode::MissingFile,
            "File IMG_001.CR3 not found",
            "Could not find the image file.",
            "Check the folder path and try again.",
        );

        let display = ErrorHandler::handle_error(&error);
        assert_eq!(display.style, "error");
        assert_eq!(display.message, "Could not find the image file.");
        assert!(display.recovery.contains("folder"));
        assert!(!display.show_details); // User errors never show details
    }

    #[test]
    fn test_system_error_handling() {
        let error = ErrorDetails::new(
            ErrorCode::DiskFull,
            "Disk full at /Volumes/media",
            "Not enough disk space.",
            "Free up space and try again.",
        );

        let display = ErrorHandler::handle_error(&error);
        assert_eq!(display.style, "warning");
        assert_eq!(display.message, "Not enough disk space.");
        assert!(display.recovery.contains("Free"));
        // show_details depends on cfg!(debug_assertions)
    }

    #[test]
    fn test_algorithm_error_handling() {
        let error = ErrorDetails::new(
            ErrorCode::UnresolvableConflict,
            "Conflict between IMG_001 and IMG_002: 0.5 vs 0.5",
            "Images have equal ratings.",
            "Manually compare and select one.",
        );

        let display = ErrorHandler::handle_error(&error);
        assert_eq!(display.style, "error");
        assert_eq!(display.message, "Images have equal ratings.");
        assert!(display.recovery.contains("select"));
    }
}
