// BACKEND_HEARTBEAT_SYSTEM.rs
// Connection health monitoring through periodic heartbeat messages

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use serde_json::json;
use tauri::{Runtime, Window};
use chrono;

// Heartbeat configuration
const HEARTBEAT_INTERVAL_MS: u64 = 2000; // 2 seconds between heartbeats
const MAX_MISSED_HEARTBEATS: usize = 3;  // Consider disconnected after 3 missed replies

/// Health monitoring status for Tauri window connections
struct ConnectionHealth {
    last_successful_heartbeat: Instant,
    disconnected: bool,
    missed_heartbeats: usize,
    frontend_version: Option<String>,
}

/// Start the heartbeat system to monitor frontend connectivity
pub fn start_heartbeat_system<R: Runtime>(window: Window<R>) {
    // Initialize connection health tracker
    let health = Arc::new(Mutex::new(ConnectionHealth {
        last_successful_heartbeat: Instant::now(),
        disconnected: false,
        missed_heartbeats: 0,
        frontend_version: None,
    }));
    
    // Clone references for the threads
    let health_sender = health.clone();
    let window_sender = window.clone();
    
    // Register event listener for heartbeat replies from frontend
    let health_receiver = health.clone();
    let _ = window.listen("frontend-heartbeat-reply", move |event| {
        let mut health = health_receiver.lock().unwrap();
        
        // Record successful heartbeat
        health.last_successful_heartbeat = Instant::now();
        health.missed_heartbeats = 0;
        
        // Store frontend version if provided
        if let Some(payload) = event.payload() {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
                if let Some(version) = data.get("version").and_then(|v| v.as_str()) {
                    health.frontend_version = Some(version.to_string());
                }
            }
        }
        
        // Log recovery from disconnection
        if health.disconnected {
            println!("Frontend connection restored");
            health.disconnected = false;
        }
    });
    
    // Spawn thread to send periodic heartbeats
    thread::spawn(move || {
        println!("Starting backend heartbeat system");
        
        loop {
            // Sleep for the heartbeat interval
            thread::sleep(Duration::from_millis(HEARTBEAT_INTERVAL_MS));
            
            // Check if window is still valid
            if window_sender.is_visible().unwrap_or(false) {
                // Create heartbeat payload with timestamp
                let heartbeat_data = json!({
                    "timestamp": chrono::Utc::now().timestamp_millis(),
                    "process_id": std::process::id(),
                    "uptime_ms": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis(),
                });
                
                // Send heartbeat event to frontend
                let result = window_sender.emit("backend-heartbeat", heartbeat_data);
                
                // Update health status
                let mut health = health_sender.lock().unwrap();
                
                if result.is_err() {
                    health.missed_heartbeats += 1;
                    
                    // Check if we've reached disconnection threshold
                    if health.missed_heartbeats >= MAX_MISSED_HEARTBEATS && !health.disconnected {
                        println!(
                            "Frontend connection lost after {} missed heartbeats. Waiting for reconnection...",
                            health.missed_heartbeats
                        );
                        health.disconnected = true;
                    }
                } else {
                    // Even on successful emit, we need to verify replies
                    let elapsed = health.last_successful_heartbeat.elapsed();
                    
                    // If no reply in 3x the interval, consider it disconnected
                    if elapsed > Duration::from_millis(HEARTBEAT_INTERVAL_MS * 3) {
                        health.missed_heartbeats += 1;
                        
                        if health.missed_heartbeats >= MAX_MISSED_HEARTBEATS && !health.disconnected {
                            println!(
                                "Frontend stopped responding to heartbeats ({:.2}s since last reply)",
                                elapsed.as_secs_f64()
                            );
                            health.disconnected = true;
                        }
                    }
                }
            }
        }
    });
}

/// Example frontend handler to include in JavaScript/TypeScript
/// 
/// ```typescript
/// // Frontend heartbeat reply handler
/// if (typeof window.__TAURI__ !== 'undefined') {
///   window.__TAURI__.event.listen('backend-heartbeat', (event) => {
///     // Reply to the heartbeat to confirm frontend is responsive
///     window.__TAURI__.event.emit('frontend-heartbeat-reply', {
///       timestamp: Date.now(),
///       version: process.env.REACT_APP_VERSION || '1.0.0',
///     });
///     
///     // Update last heartbeat timestamp
///     lastHeartbeatReceived = Date.now();
///   });
/// }
/// ```