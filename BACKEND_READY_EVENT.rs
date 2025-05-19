// BACKEND_READY_EVENT.rs
// Implementation of backend-ready event with delay to ensure frontend is ready

use chrono;
use serde_json::json;
use std::thread;
use std::time::Duration;
use tauri::{Runtime, Window};

/// Adds the backend-ready event emission to main.rs
/// Should be called after the window is created and before the run() call
pub fn emit_backend_ready<R: Runtime>(window: Window<R>) {
    // Collect runtime information to send with the event
    let runtime_info = json!({
        "timestamp": chrono::Utc::now().timestamp_millis(),
        "version": env!("CARGO_PKG_VERSION"),
        "mode": if cfg!(debug_assertions) { "development" } else { "production" },
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "process_id": std::process::id(),
    });

    // Clone the window handle to use in the thread
    let startup_window = window.clone();
    
    println!("Preparing to emit backend-ready event...");
    
    // Emit event in a separate thread with delay to ensure frontend has time to setup listeners
    thread::spawn(move || {
        // Initial delay to allow frontend event listeners to be registered
        let delay_ms = 500;
        
        println!("Waiting {}ms before emitting backend-ready event...", delay_ms);
        thread::sleep(Duration::from_millis(delay_ms));
        
        // Emit the event with runtime information
        match startup_window.emit("backend-ready", runtime_info.clone()) {
            Ok(_) => println!("Successfully emitted backend-ready event"),
            Err(e) => println!("Failed to emit backend-ready event: {}", e),
        }
        
        // Schedule a second emission a bit later in case frontend wasn't ready yet
        // This ensures frontend will receive the event even if it took longer to initialize
        let retry_window = startup_window.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1500)); // Wait 1.5s more
            
            match retry_window.emit("backend-ready", runtime_info) {
                Ok(_) => println!("Emitted backup backend-ready event"),
                Err(_) => (), // Silently ignore errors on backup emission
            }
        });
    });
}

/// Function to be added to main.rs for proper event emission at startup
pub fn setup_backend_ready_emission<R: Runtime>(window: Window<R>) {
    // Emit the backend-ready event
    emit_backend_ready(window.clone());
    
    // Start the heartbeat system if needed
    // This would typically be initialized here as well
}

/// Example of how to call this from main.rs
/// 
/// ```rust
/// #[tauri::command]
/// fn main() {
///     tauri::Builder::default()
///         .setup(|app| {
///             let main_window = app.get_window("main").unwrap();
///             
///             // Setup backend ready emission system
///             setup_backend_ready_emission(main_window.clone());
///             
///             Ok(())
///         })
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```