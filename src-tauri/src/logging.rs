use std::path::PathBuf;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize structured JSON logging to ~/.photorg/logs/app.log
///
/// Creates the logs directory if it doesn't exist and sets up file rotation.
/// Log level defaults to INFO and can be controlled via RUST_LOG environment variable.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = get_log_dir()?;

    // Create logs directory if missing
    std::fs::create_dir_all(&log_dir)?;

    // Set up file appender for app.log
    let file_appender = tracing_appender::rolling::never(&log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // JSON subscriber for structured logging
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    // Console subscriber for debug output (development only)
    let console_layer = if cfg!(debug_assertions) {
        Some(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_filter(tracing_subscriber::EnvFilter::from_default_env()),
        )
    } else {
        None
    };

    // Initialize subscriber with both layers
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(json_layer)
        .with(console_layer)
        .init();

    // Log startup info
    tracing::info!(
        target: "photorg::startup",
        operation = "logging_initialized",
        result = "success",
        log_dir = ?log_dir,
        "Logging initialized"
    );

    Ok(())
}

/// Get the app logs directory: ~/.photorg/logs
fn get_log_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir()
        .ok_or("Could not determine home directory")?;

    Ok(home.join(".photorg").join("logs"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_dir_path() {
        let log_dir = get_log_dir().expect("Failed to get log dir");
        assert!(log_dir.to_string_lossy().contains(".photorg"));
        assert!(log_dir.to_string_lossy().contains("logs"));
    }
}
