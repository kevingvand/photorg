/// Error Display Feature Slice
///
/// Responsible for:
/// - Converting backend errors to user-friendly frontend messages
/// - Determining appropriate UI treatment based on error category
/// - Providing composable error display components
///
/// This feature depends on infrastructure::error for error types.
/// The frontend depends on this module's exported types and composables.

pub mod handler;

pub use handler::ErrorHandler;
