use serde::{Deserialize, Serialize};
use std::fmt;

/// Error categories for structured error reporting.
///
/// Categorization determines how errors are handled by the frontend:
/// - **User**: Actionable by end user (invalid input, missing files)
/// - **System**: Diagnostic (disk full, permissions)
/// - **Algorithm**: Requires debugging (rater failure, conflicts)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_category_display() {
        assert_eq!(ErrorCategory::User.to_string(), "user");
        assert_eq!(ErrorCategory::System.to_string(), "system");
        assert_eq!(ErrorCategory::Algorithm.to_string(), "algorithm");
    }

    #[test]
    fn test_error_category_serialization() {
        let category = ErrorCategory::User;
        let json = serde_json::to_string(&category).expect("Failed to serialize");
        assert_eq!(json, "\"user\"");
    }
}
