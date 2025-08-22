// IMPROVED_EVENT_EMITTER.rs
// Improved event naming standardization and error handling

use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{Runtime, Window};

/// Event categories for better organization
pub enum EventCategory {
    Configuration,
    Process,
    Terminal,
    Session,
    System,
    Connection,
}

/// Standard event names for application-wide consistency
pub enum ServiceEvent<T: Serialize + Clone> {
    ConfigLoaded(T),
    ConfigUpdated(T),
    ProcessStarted(T),
    ProcessExited(T),
    ProcessOutput(T),
    SessionCreated(T),
    SessionRemoved(T),
    TerminalData(T),
    SystemError(T),
    ConnectionStateChanged(T),
}

/// Improved event emitter for consistent event naming and error handling
pub struct EventEmitter<R: Runtime> {
    window: Window<R>,
    event_prefixes: HashMap<EventCategory, &'static str>,
    event_counts: Arc<Mutex<HashMap<String, usize>>>,
}

impl<R: Runtime> EventEmitter<R> {
    /// Create a new event emitter
    pub fn new(window: Window<R>) -> Self {
        // Initialize event prefixes for consistent naming
        let mut prefixes = HashMap::new();
        prefixes.insert(EventCategory::Configuration, "config");
        prefixes.insert(EventCategory::Process, "process");
        prefixes.insert(EventCategory::Terminal, "terminal");
        prefixes.insert(EventCategory::Session, "session");
        prefixes.insert(EventCategory::System, "system");
        prefixes.insert(EventCategory::Connection, "connection");
        
        Self {
            window,
            event_prefixes: prefixes,
            event_counts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Get the category prefix for an event
    fn get_prefix(&self, category: &EventCategory) -> &'static str {
        self.event_prefixes.get(category).unwrap_or(&"event")
    }
    
    /// Sanitize event name to ensure it only contains valid characters
    fn sanitize_event_name(&self, name: &str) -> String {
        // Ensure event name only contains alphanumeric characters, `-`, `/`, `:` and `_`
        // These are the characters allowed by Tauri's event system
        name.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '/' || c == ':' || c == '_' {
                    c
                } else {
                    '-'
                }
            })
            .collect()
    }
    
    /// Emit an event with the standardized event naming convention
    pub fn emit<T: Serialize + Clone>(&self, category: EventCategory, name: &str, payload: T) -> Result<(), String> {
        // Create standardized event name with category prefix
        let prefix = self.get_prefix(&category);
        let full_name = format!("{}-{}", prefix, name);
        let safe_name = self.sanitize_event_name(&full_name);
        
        // Increment event counter for this event type
        {
            let mut counts = self.event_counts.lock().unwrap();
            let count = counts.entry(safe_name.clone()).or_insert(0);
            *count += 1;
        }
        
        // Emit the event
        self.window.emit(&safe_name, payload.clone())
            .map_err(|e| format!("Failed to emit event {}: {}", safe_name, e))
    }
    
    /// Emit a service event using standardized event formats
    pub fn emit_service_event<T: Serialize + Clone>(&self, event: ServiceEvent<T>) -> Result<(), String> {
        // Get the event name and category based on event type
        let (category, name, payload) = match &event {
            // Configuration events
            ServiceEvent::ConfigLoaded(data) => (EventCategory::Configuration, "loaded", data.clone()),
            ServiceEvent::ConfigUpdated(data) => (EventCategory::Configuration, "updated", data.clone()),
            
            // Process events
            ServiceEvent::ProcessStarted(data) => (EventCategory::Process, "started", data.clone()),
            ServiceEvent::ProcessExited(data) => (EventCategory::Process, "exited", data.clone()),
            ServiceEvent::ProcessOutput(data) => (EventCategory::Process, "output", data.clone()),
            
            // Session events
            ServiceEvent::SessionCreated(data) => (EventCategory::Session, "created", data.clone()),
            ServiceEvent::SessionRemoved(data) => (EventCategory::Session, "removed", data.clone()),
            
            // Terminal events
            ServiceEvent::TerminalData(data) => (EventCategory::Terminal, "data", data.clone()),
            
            // System events
            ServiceEvent::SystemError(data) => (EventCategory::System, "error", data.clone()),
            
            // Connection events
            ServiceEvent::ConnectionStateChanged(data) => (EventCategory::Connection, "state-changed", data.clone()),
        };
        
        // Emit the event with standardized naming
        self.emit(category, name, payload)
    }
    
    /// Get statistics on emitted events
    pub fn get_event_stats(&self) -> HashMap<String, usize> {
        self.event_counts.lock().unwrap().clone()
    }
}

/// Example usage
/// 
/// ```rust
/// // In your Tauri application setup
/// fn setup_event_emitter<R: Runtime>(window: Window<R>) -> EventEmitter<R> {
///     let emitter = EventEmitter::new(window);
///     
///     // Use the emitter for all event operations
///     emitter
/// }
/// 
/// // Example event emission
/// fn handle_process_start<R: Runtime>(
///     emitter: &EventEmitter<R>, 
///     process_data: ProcessData
/// ) -> Result<(), String> {
///     // Emit standardized event
///     emitter.emit_service_event(ServiceEvent::ProcessStarted(process_data))
/// }
/// ```
/// 
/// And in your frontend TypeScript:
/// 
/// ```typescript
/// // Consistent event name handling
/// const EVENT_PREFIXES = {
///   CONFIG: 'config',
///   PROCESS: 'process',
///   TERMINAL: 'terminal',
///   SESSION: 'session',
///   SYSTEM: 'system',
///   CONNECTION: 'connection'
/// };
/// 
/// // Helper to create standardized event names
/// const createEventName = (prefix: string, name: string): string => {
///   return `${prefix}-${name}`;
/// };
/// 
/// // Listen for process started events
/// if (typeof window.__TAURI__ !== 'undefined') {
///   window.__TAURI__.event.listen(
///     createEventName(EVENT_PREFIXES.PROCESS, 'started'), 
///     (event) => {
///       console.log('Process started:', event.payload);
///     }
///   );
/// }
/// ```