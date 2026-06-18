// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod features;
pub mod infrastructure;

// Re-export commonly used types for convenience
pub use infrastructure::{ErrorCategory, ErrorCode, ErrorDetails, PhotorgResult};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging infrastructure first
    if let Err(e) = infrastructure::logging::init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
