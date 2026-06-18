/// Feature Slices
///
/// Each module here represents a complete vertical slice:
/// - UI components (Vue)
/// - Business logic handlers
/// - Data models
/// - Tests
///
/// Features depend on infrastructure (error, logging) but infrastructure
/// never depends on features. This ensures clean separation of concerns.

pub mod error_display;
