/// Shared infrastructure: error handling, logging, and auditing.
///
/// This module is intentionally kept separate from feature slices.
/// It provides cross-cutting concerns that multiple features depend on:
/// - **error**: Error abstraction, codes, and categorization
/// - **logging**: Structured JSON logging setup
/// - **audit**: Append-only audit trail for decisions and exports
///
/// The infrastructure is initialized once at app startup and then used
/// transparently throughout the application.
/// Features should never initialize these modules themselves.

pub mod audit;
pub mod error;
pub mod logging;

// Re-export public types for convenience
pub use audit::AuditEntry;
pub use error::{ErrorCategory, ErrorCode, ErrorDetails, PhotorgResult};
