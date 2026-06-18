use serde::{Deserialize, Serialize};
use std::fmt;

use super::category::ErrorCategory; // Import from sibling module

/// Error code for structured error identification and categorization.
///
/// Each code automatically maps to an ErrorCategory, determining how the frontend
/// handles the error. Codes should be application-wide; features don't define their own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // User errors: user can fix these directly
    MissingFile,
    InvalidPath,
    UnsupportedFormat,
    InvalidInput,
    FileNotFound,

    // System errors: diagnostic; user can't fix but we can log for support
    PermissionDenied,
    DiskFull,
    DatabaseCorruption,
    OutOfMemory,
    IoError,

    // Algorithm errors: bugs or data conflicts requiring investigation
    UnresolvableConflict,
    RaterFailure,
    DataInvariantViolation,
    PairingFailure,

    // Generic fallback
    Unknown,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ErrorCode {
    /// Determine the category for this error code.
    ///
    /// Categorization is deterministic and implicit; the frontend receives
    /// the category along with the code and adjusts UI treatment accordingly.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_category_user() {
        assert_eq!(
            ErrorCode::MissingFile.category(),
            ErrorCategory::User
        );
        assert_eq!(
            ErrorCode::InvalidInput.category(),
            ErrorCategory::User
        );
    }

    #[test]
    fn test_error_code_category_system() {
        assert_eq!(
            ErrorCode::PermissionDenied.category(),
            ErrorCategory::System
        );
        assert_eq!(
            ErrorCode::DiskFull.category(),
            ErrorCategory::System
        );
    }

    #[test]
    fn test_error_code_category_algorithm() {
        assert_eq!(
            ErrorCode::UnresolvableConflict.category(),
            ErrorCategory::Algorithm
        );
        assert_eq!(
            ErrorCode::RaterFailure.category(),
            ErrorCategory::Algorithm
        );
    }

    #[test]
    fn test_error_code_unknown_defaults_to_system() {
        assert_eq!(
            ErrorCode::Unknown.category(),
            ErrorCategory::System
        );
    }
}
