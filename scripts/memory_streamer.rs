use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::broadcast;
use tokio::time::sleep;

use crate::memory_segment_parser::{MemorySegment, MemoryType};
use crate::memory_loader::MemoryLoader;
use crate::memory_checksum_cache::MemoryChecksumCache;

/// Stream type indicating the kind of memory update
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamType {
    /// Full memory update (initial load or forced refresh)
    Full,
    /// Delta update (only changed segments)
    Delta,
    /// Priority update (critical segments only)
    Priority,
    /// Background update (low priority, opportunistic)
    Background,
}

/// Stream status indicating the current state of the memory stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamStatus {
    /// Stream is active and delivering updates
    Active,
    /// Stream is paused but can be resumed
    Paused,
    /// Stream has completed successfully
    Completed,
    /// Stream has failed and needs attention
    Failed,
    /// Stream is in the process of reconnecting
    Reconnecting,
}

/// Memory update event sent through the stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdateEvent {
    /// Unique identifier for the update event
    pub event_id: String,
    /// Agent the update is for
    pub agent_name: String,
    /// Type of stream this update is part of
    pub stream_type: StreamType,
    /// Segments included in this update
    pub segments: Vec<MemorySegment>,
    /// Timestamp when update was created
    pub timestamp: u64,
    /// Sequence number within the current stream
    pub sequence_number: u32,
    /// Whether this is the last update in a sequence
    pub is_final: bool,
    /// Additional metadata for the update
    pub metadata: HashMap<String, String>,
}

/// Configuration for memory streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Maximum updates per second
    pub max_updates_per_second: u32,
    /// Maximum batch size (number of segments per update)
    pub max_batch_size: usize,
    /// Whether to enable background polling
    pub enable_polling: bool,
    /// Polling interval in seconds
    pub polling_interval_seconds: u64,
    /// Whether to enable file watching
    pub enable_file_watching: bool,
    /// Whether to stream tier 1 memory immediately
    pub stream_tier1_immediately: bool,
    /// Buffer size for the stream
    pub stream_buffer_size: usize,
    /// Maximum segments to keep in update history
    pub max_history_size: usize,
    /// Directories to watch for memory file changes
    pub watch_directories: Vec<String>,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            max_updates_per_second: 10,
            max_batch_size: 10,
            enable_polling: true,
            polling_interval_seconds: 60,
            enable_file_watching: true,
            stream_tier1_immediately: true,
            stream_buffer_size: 100,
            max_history_size: 50,
            watch_directories: vec!["AGENTS".to_string()],
        }
    }
}

/// Manages the streaming of memory updates to agents
pub struct MemoryStreamer {
    /// Configuration for the streamer
    config: StreamConfig,
    /// Reference to memory loader for fetching segments
    memory_loader: Arc<Mutex<MemoryLoader>>,
    /// Checksum cache for detecting changes
    checksum_cache: Arc<Mutex<MemoryChecksumCache>>,
    /// Last known state of each agent's memory
    agent_memory_state: DashMap<String, HashMap<String, String>>,
    /// Channels for each active agent
    agent_channels: DashMap<String, Sender<MemoryUpdateEvent>>,
    /// Stream status for each agent
    stream_status: DashMap<String, StreamStatus>,
    /// History of recent updates for replay
    update_history: Arc<RwLock<VecDeque<MemoryUpdateEvent>>>,
    /// Optional file watcher for real-time updates
    #[allow(dead_code)]
    file_watcher: Option<RecommendedWatcher>,
    /// Broadcast channel for system-wide updates
    system_update_channel: Arc<broadcast::Sender<MemoryUpdateEvent>>,
    /// Timestamp of the last time each agent requested an update
    last_update_request: DashMap<String, Instant>,
    /// Throttle counters for rate limiting
    update_counters: Arc<RwLock<HashMap<String, (Instant, u32)>>>,
}

impl MemoryStreamer {
    /// Create a new memory streamer with the given configuration
    pub fn new(
        config: StreamConfig, 
        memory_loader: Arc<Mutex<MemoryLoader>>,
        checksum_cache: Arc<Mutex<MemoryChecksumCache>>,
    ) -> Result<Self, Box<dyn Error>> {
        // Create broadcast channel for system-wide updates
        let (system_update_tx, _) = broadcast::channel(config.stream_buffer_size);

        // Create the streamer
        let mut streamer = Self {
            config,
            memory_loader,
            checksum_cache,
            agent_memory_state: DashMap::new(),
            agent_channels: DashMap::new(),
            stream_status: DashMap::new(),
            update_history: Arc::new(RwLock::new(VecDeque::with_capacity(50))),
            file_watcher: None,
            system_update_channel: Arc::new(system_update_tx),
            last_update_request: DashMap::new(),
            update_counters: Arc::new(RwLock::new(HashMap::new())),
        };

        // Set up file watching if enabled
        if streamer.config.enable_file_watching {
            streamer.setup_file_watcher()?;
        }

        // Set up background polling if enabled
        if streamer.config.enable_polling {
            streamer.setup_background_polling()?;
        }

        Ok(streamer)
    }

    /// Create a new memory streamer with default configuration
    pub fn default(
        memory_loader: Arc<Mutex<MemoryLoader>>,
        checksum_cache: Arc<Mutex<MemoryChecksumCache>>,
    ) -> Result<Self, Box<dyn Error>> {
        Self::new(StreamConfig::default(), memory_loader, checksum_cache)
    }

    /// Subscribe to memory updates for a specific agent
    pub async fn subscribe_to_agent(&self, agent_name: &str) -> Result<Receiver<MemoryUpdateEvent>, Box<dyn Error>> {
        // Check if already subscribed
        if self.agent_channels.contains_key(agent_name) {
            return Err(format!("Already subscribed to agent {}", agent_name).into());
        }

        // Create a channel for this agent
        let (tx, rx) = mpsc::channel(self.config.stream_buffer_size);
        self.agent_channels.insert(agent_name.to_string(), tx);
        self.stream_status.insert(agent_name.to_string(), StreamStatus::Active);
        self.last_update_request.insert(agent_name.to_string(), Instant::now());

        // If there's history, send it immediately
        if self.config.stream_tier1_immediately {
            self.send_initial_tier1_memory(agent_name).await?;
        }

        Ok(rx)
    }

    /// Unsubscribe from memory updates for a specific agent
    pub fn unsubscribe_from_agent(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        self.agent_channels.remove(agent_name);
        self.stream_status.remove(agent_name);
        Ok(())
    }

    /// Subscribe to system-wide memory updates
    pub fn subscribe_to_system_updates(&self) -> broadcast::Receiver<MemoryUpdateEvent> {
        self.system_update_channel.subscribe()
    }

    /// Request a memory update for a specific agent
    pub async fn request_update(&self, agent_name: &str, stream_type: StreamType) -> Result<(), Box<dyn Error>> {
        // Rate limit check
        if !self.check_rate_limit(agent_name) {
            return Err("Rate limit exceeded for this agent".into());
        }

        // Update the last request timestamp
        self.last_update_request.insert(agent_name.to_string(), Instant::now());

        // Process based on stream type
        match stream_type {
            StreamType::Full => {
                self.process_full_update(agent_name).await?;
            },
            StreamType::Delta => {
                self.process_delta_update(agent_name).await?;
            },
            StreamType::Priority => {
                self.process_priority_update(agent_name).await?;
            },
            StreamType::Background => {
                self.process_background_update(agent_name).await?;
            },
        }

        Ok(())
    }

    /// Pause the memory stream for a specific agent
    pub fn pause_stream(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(mut status) = self.stream_status.get_mut(agent_name) {
            *status = StreamStatus::Paused;
            Ok(())
        } else {
            Err(format!("No active stream for agent {}", agent_name).into())
        }
    }

    /// Resume the memory stream for a specific agent
    pub fn resume_stream(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(mut status) = self.stream_status.get_mut(agent_name) {
            *status = StreamStatus::Active;
            Ok(())
        } else {
            Err(format!("No active stream for agent {}", agent_name).into())
        }
    }

    /// Get the current status of an agent's memory stream
    pub fn get_stream_status(&self, agent_name: &str) -> Option<StreamStatus> {
        self.stream_status.get(agent_name).map(|s| *s)
    }

    /// Send the initial tier 1 memory segments immediately
    async fn send_initial_tier1_memory(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get all tier 1 memory segments for this agent
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let segments = memory_loader.load_agent_tier1_memory(agent_name)?;
        drop(memory_loader); // Release lock

        if segments.is_empty() {
            return Ok(());
        }

        // Create an update event
        let event = MemoryUpdateEvent {
            event_id: generate_event_id(),
            agent_name: agent_name.to_string(),
            stream_type: StreamType::Priority,
            segments,
            timestamp: current_timestamp(),
            sequence_number: 1,
            is_final: true,
            metadata: HashMap::new(),
        };

        // Send the update
        self.send_update_to_agent(agent_name, event).await?;

        Ok(())
    }

    /// Process a full memory update for an agent
    async fn process_full_update(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get all memory segments for this agent
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let all_segments = memory_loader.load_agent_memory(agent_name, None)?;
        drop(memory_loader); // Release lock

        // Stream in batches to avoid overwhelming the channel
        let batch_size = self.config.max_batch_size;
        let total_batches = (all_segments.len() + batch_size - 1) / batch_size;

        for (i, chunk) in all_segments.chunks(batch_size).enumerate() {
            // Create batch update event
            let event = MemoryUpdateEvent {
                event_id: generate_event_id(),
                agent_name: agent_name.to_string(),
                stream_type: StreamType::Full,
                segments: chunk.to_vec(),
                timestamp: current_timestamp(),
                sequence_number: (i + 1) as u32,
                is_final: i == total_batches - 1,
                metadata: HashMap::new(),
            };

            // Send the update
            self.send_update_to_agent(agent_name, event.clone()).await?;

            // Short delay between batches to avoid flooding
            if i < total_batches - 1 {
                sleep(Duration::from_millis(100)).await;
            }
        }

        // Update the agent's memory state
        self.update_agent_memory_state(agent_name, &all_segments)?;

        Ok(())
    }

    /// Process a delta update for an agent (only changed segments)
    async fn process_delta_update(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get the current state from the checksum cache
        let checksum_cache = self.checksum_cache.lock().map_err(|e| format!("Failed to lock checksum cache: {}", e))?;
        let changed_segments = checksum_cache.get_changed_segments(agent_name)?;
        drop(checksum_cache); // Release lock

        if changed_segments.is_empty() {
            // No changes, send an empty final update
            let event = MemoryUpdateEvent {
                event_id: generate_event_id(),
                agent_name: agent_name.to_string(),
                stream_type: StreamType::Delta,
                segments: Vec::new(),
                timestamp: current_timestamp(),
                sequence_number: 1,
                is_final: true,
                metadata: HashMap::from([
                    ("status".to_string(), "no_changes".to_string()),
                ]),
            };

            self.send_update_to_agent(agent_name, event).await?;
            return Ok(());
        }

        // Stream in batches
        let batch_size = self.config.max_batch_size;
        let total_batches = (changed_segments.len() + batch_size - 1) / batch_size;

        for (i, chunk) in changed_segments.chunks(batch_size).enumerate() {
            // Create batch update event
            let event = MemoryUpdateEvent {
                event_id: generate_event_id(),
                agent_name: agent_name.to_string(),
                stream_type: StreamType::Delta,
                segments: chunk.to_vec(),
                timestamp: current_timestamp(),
                sequence_number: (i + 1) as u32,
                is_final: i == total_batches - 1,
                metadata: HashMap::from([
                    ("total_changes".to_string(), changed_segments.len().to_string()),
                ]),
            };

            // Send the update
            self.send_update_to_agent(agent_name, event.clone()).await?;

            // Short delay between batches
            if i < total_batches - 1 {
                sleep(Duration::from_millis(100)).await;
            }
        }

        // Update the agent's memory state
        self.update_agent_memory_state(agent_name, &changed_segments)?;

        Ok(())
    }

    /// Process a priority update for an agent (only high-priority segments)
    async fn process_priority_update(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get all tier 1 and identity segments
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let mut priority_segments = memory_loader.load_agent_tier1_memory(agent_name)?;
        
        // Add identity segments if not already included
        let identity_segments = memory_loader.load_agent_memory_by_type(agent_name, MemoryType::Identity)?;
        drop(memory_loader); // Release lock
        
        // Merge identity segments that aren't already in the priority list
        let existing_ids: std::collections::HashSet<String> = priority_segments.iter()
            .map(|s| s.id.clone())
            .collect();
            
        for segment in identity_segments {
            if !existing_ids.contains(&segment.id) {
                priority_segments.push(segment);
            }
        }

        // Send in a single batch (priority updates are typically small)
        let event = MemoryUpdateEvent {
            event_id: generate_event_id(),
            agent_name: agent_name.to_string(),
            stream_type: StreamType::Priority,
            segments: priority_segments.clone(),
            timestamp: current_timestamp(),
            sequence_number: 1,
            is_final: true,
            metadata: HashMap::new(),
        };

        // Send the update
        self.send_update_to_agent(agent_name, event).await?;

        // Update the agent's memory state (only for these segments)
        self.update_agent_memory_state(agent_name, &priority_segments)?;

        Ok(())
    }

    /// Process a background update for an agent (low priority, run when idle)
    async fn process_background_update(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get segments that haven't been accessed recently but might be needed
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let background_segments = memory_loader.load_agent_background_memory(agent_name)?;
        drop(memory_loader); // Release lock

        if background_segments.is_empty() {
            return Ok(());
        }

        // Stream in small batches with longer delays to minimize impact
        let batch_size = self.config.max_batch_size / 2; // Smaller batches for background
        let total_batches = (background_segments.len() + batch_size - 1) / batch_size;

        for (i, chunk) in background_segments.chunks(batch_size).enumerate() {
            // Create batch update event
            let event = MemoryUpdateEvent {
                event_id: generate_event_id(),
                agent_name: agent_name.to_string(),
                stream_type: StreamType::Background,
                segments: chunk.to_vec(),
                timestamp: current_timestamp(),
                sequence_number: (i + 1) as u32,
                is_final: i == total_batches - 1,
                metadata: HashMap::from([
                    ("background".to_string(), "true".to_string()),
                ]),
            };

            // Send the update
            self.send_update_to_agent(agent_name, event.clone()).await?;

            // Longer delay between background batches
            if i < total_batches - 1 {
                sleep(Duration::from_millis(500)).await;
            }
        }

        // Update the agent's memory state
        self.update_agent_memory_state(agent_name, &background_segments)?;

        Ok(())
    }

    /// Send an update to a specific agent
    async fn send_update_to_agent(&self, agent_name: &str, event: MemoryUpdateEvent) -> Result<(), Box<dyn Error>> {
        // Check if the agent's stream is active
        if let Some(status) = self.stream_status.get(agent_name) {
            if *status != StreamStatus::Active {
                return Err(format!("Stream for agent {} is not active (status: {:?})", agent_name, *status).into());
            }
        } else {
            return Err(format!("No stream found for agent {}", agent_name).into());
        }

        // Send the update to the agent's channel
        if let Some(tx) = self.agent_channels.get(agent_name) {
            tx.send(event.clone()).await.map_err(|e| format!("Failed to send update: {}", e))?;
        } else {
            return Err(format!("No channel found for agent {}", agent_name).into());
        }

        // Add to history for replay capabilities
        {
            let mut history = self.update_history.write().map_err(|e| format!("Failed to lock update history: {}", e))?;
            history.push_back(event.clone());
            
            // Trim history if needed
            while history.len() > self.config.max_history_size {
                history.pop_front();
            }
        }

        // Broadcast to system channel for system-wide subscribers
        let _ = self.system_update_channel.send(event);

        Ok(())
    }

    /// Update the agent's memory state with the provided segments
    fn update_agent_memory_state(&self, agent_name: &str, segments: &[MemorySegment]) -> Result<(), Box<dyn Error>> {
        let mut state = self.agent_memory_state.entry(agent_name.to_string()).or_insert_with(HashMap::new);
        
        // Update checksums for each segment
        for segment in segments {
            // Create a simple checksum for the content
            let checksum = calculate_content_hash(&segment.content);
            state.insert(segment.id.clone(), checksum);
        }

        Ok(())
    }

    /// Set up file watching for real-time updates
    fn setup_file_watcher(&mut self) -> Result<(), Box<dyn Error>> {
        // Create a recommended watcher with default configuration
        let (tx, rx) = std::sync::mpsc::channel();
        let watcher = RecommendedWatcher::new(tx, Config::default())?;

        // Clone what we need for the background task
        let update_channel = self.system_update_channel.clone();
        let checksum_cache = self.checksum_cache.clone();
        let memory_loader = self.memory_loader.clone();
        let watch_directories = self.config.watch_directories.clone();

        // Set up the watcher in a background thread
        std::thread::spawn(move || {
            // Watch each configured directory
            for dir in &watch_directories {
                match watcher.watch(Path::new(dir), RecursiveMode::Recursive) {
                    Ok(_) => println!("Watching directory: {}", dir),
                    Err(e) => eprintln!("Failed to watch directory {}: {}", dir, e),
                }
            }

            // Process filesystem events
            for event in rx {
                match event {
                    Ok(event) => {
                        // Process file changes
                        if let Some(path) = event.paths.first() {
                            // Only process markdown files
                            if let Some(ext) = path.extension() {
                                if ext.to_str() == Some("md") {
                                    // Identify which agent's file changed
                                    if let Some(agent_name) = extract_agent_name_from_path(path) {
                                        // Update the checksum cache
                                        if let Ok(mut cache) = checksum_cache.lock() {
                                            if let Err(e) = cache.update_file_checksum(path) {
                                                eprintln!("Failed to update checksum: {}", e);
                                            }
                                        }

                                        // Reload the file and broadcast changes
                                        if let Ok(loader) = memory_loader.lock() {
                                            match loader.load_segments_from_file(path) {
                                                Ok(segments) => {
                                                    // Create an update event
                                                    let event = MemoryUpdateEvent {
                                                        event_id: generate_event_id(),
                                                        agent_name: agent_name.to_string(),
                                                        stream_type: StreamType::Delta,
                                                        segments,
                                                        timestamp: current_timestamp(),
                                                        sequence_number: 1,
                                                        is_final: true,
                                                        metadata: HashMap::from([
                                                            ("source".to_string(), "file_watcher".to_string()),
                                                            ("file".to_string(), path.to_string_lossy().to_string()),
                                                        ]),
                                                    };

                                                    // Broadcast the update
                                                    let _ = update_channel.send(event);
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to load segments from file {}: {}", 
                                                        path.display(), e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("File watch error: {}", e),
                }
            }
        });

        Ok(())
    }

    /// Set up background polling for changes
    fn setup_background_polling(&self) -> Result<(), Box<dyn Error>> {
        // Clone what we need for the background task
        let checksum_cache = self.checksum_cache.clone();
        let agent_channels = self.agent_channels.clone();
        let system_update_channel = self.system_update_channel.clone();
        let polling_interval = Duration::from_secs(self.config.polling_interval_seconds);

        // Clone self for the background task
        let self_clone = self.clone();

        // Start the background polling task
        tokio::spawn(async move {
            loop {
                // Sleep for the polling interval
                sleep(polling_interval).await;

                // Update checksums for all agents
                if let Ok(mut cache) = checksum_cache.lock() {
                    if let Err(e) = cache.update_all_checksums() {
                        eprintln!("Failed to update checksums: {}", e);
                    }
                }

                // Check for changes in each agent's memory
                for agent_entry in agent_channels.iter() {
                    let agent_name = agent_entry.key();
                    
                    // Process delta updates for active agents
                    if let Err(e) = self_clone.process_delta_update(&agent_name).await {
                        eprintln!("Failed to process delta update for {}: {}", agent_name, e);
                    }
                }

                // Periodic notification to system-wide subscribers
                let system_event = MemoryUpdateEvent {
                    event_id: generate_event_id(),
                    agent_name: "system".to_string(),
                    stream_type: StreamType::Background,
                    segments: Vec::new(),
                    timestamp: current_timestamp(),
                    sequence_number: 0,
                    is_final: true,
                    metadata: HashMap::from([
                        ("type".to_string(), "polling_heartbeat".to_string()),
                    ]),
                };
                
                let _ = system_update_channel.send(system_event);
            }
        });

        Ok(())
    }

    /// Check if the agent has exceeded the rate limit
    fn check_rate_limit(&self, agent_name: &str) -> bool {
        let now = Instant::now();
        let mut counters = self.update_counters.write().unwrap();
        
        // Get or create the counter for this agent
        let counter = counters.entry(agent_name.to_string())
            .or_insert_with(|| (now, 0));
            
        // Reset counter if it's been more than a second
        if now.duration_since(counter.0) > Duration::from_secs(1) {
            *counter = (now, 1);
            return true;
        }
        
        // Increment counter and check against limit
        counter.1 += 1;
        counter.1 <= self.config.max_updates_per_second
    }

    /// Persist the current state to disk for recovery
    pub fn persist_state(&self, state_file: &Path) -> Result<(), Box<dyn Error>> {
        // Prepare state for persistence
        #[derive(Serialize)]
        struct PersistedState {
            agent_memory_state: HashMap<String, HashMap<String, String>>,
            stream_status: HashMap<String, StreamStatus>,
            last_update_timestamps: HashMap<String, u64>,
        }
        
        let mut persisted_state = PersistedState {
            agent_memory_state: HashMap::new(),
            stream_status: HashMap::new(),
            last_update_timestamps: HashMap::new(),
        };
        
        // Collect agent memory states
        for entry in self.agent_memory_state.iter() {
            persisted_state.agent_memory_state.insert(entry.key().clone(), entry.value().clone());
        }
        
        // Collect stream statuses
        for entry in self.stream_status.iter() {
            persisted_state.stream_status.insert(entry.key().clone(), *entry.value());
        }
        
        // Collect last update timestamps
        for entry in self.last_update_request.iter() {
            persisted_state.last_update_timestamps.insert(
                entry.key().clone(), 
                entry.value().elapsed().as_secs()
            );
        }
        
        // Create the parent directory if it doesn't exist
        if let Some(parent) = state_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write the state to disk
        let file = File::create(state_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &persisted_state)?;
        
        Ok(())
    }

    /// Load persisted state from disk
    pub fn load_state(&self, state_file: &Path) -> Result<(), Box<dyn Error>> {
        // Check if the file exists
        if !state_file.exists() {
            return Err(format!("State file does not exist: {}", state_file.display()).into());
        }
        
        // Read the state from disk
        let file = File::open(state_file)?;
        let reader = BufReader::new(file);
        
        #[derive(Deserialize)]
        struct PersistedState {
            agent_memory_state: HashMap<String, HashMap<String, String>>,
            stream_status: HashMap<String, StreamStatus>,
            last_update_timestamps: HashMap<String, u64>,
        }
        
        let persisted_state: PersistedState = serde_json::from_reader(reader)?;
        
        // Restore agent memory states
        for (agent, state) in persisted_state.agent_memory_state {
            self.agent_memory_state.insert(agent, state);
        }
        
        // Restore stream statuses
        for (agent, status) in persisted_state.stream_status {
            self.stream_status.insert(agent, status);
        }
        
        // Restore last update timestamps
        let now = Instant::now();
        for (agent, seconds_ago) in persisted_state.last_update_timestamps {
            if let Some(timestamp) = now.checked_sub(Duration::from_secs(seconds_ago)) {
                self.last_update_request.insert(agent, timestamp);
            }
        }
        
        Ok(())
    }

    /// Get memory update history for an agent
    pub fn get_update_history(&self, agent_name: &str) -> Result<Vec<MemoryUpdateEvent>, Box<dyn Error>> {
        let history = self.update_history.read().map_err(|e| format!("Failed to lock update history: {}", e))?;
        
        let agent_history: Vec<MemoryUpdateEvent> = history.iter()
            .filter(|event| event.agent_name == agent_name)
            .cloned()
            .collect();
            
        Ok(agent_history)
    }

    /// Replay memory updates for an agent
    pub async fn replay_updates(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Get update history for this agent
        let agent_history = self.get_update_history(agent_name)?;
        
        // If there's no history, do a full update instead
        if agent_history.is_empty() {
            return self.process_full_update(agent_name).await;
        }
        
        // Send each update in sequence
        for event in agent_history {
            self.send_update_to_agent(agent_name, event).await?;
            sleep(Duration::from_millis(50)).await;
        }
        
        Ok(())
    }
}

// Allow cloning for the streamer to enable background tasks
impl Clone for MemoryStreamer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            memory_loader: self.memory_loader.clone(),
            checksum_cache: self.checksum_cache.clone(),
            agent_memory_state: self.agent_memory_state.clone(),
            agent_channels: self.agent_channels.clone(),
            stream_status: self.stream_status.clone(),
            update_history: self.update_history.clone(),
            file_watcher: None, // Don't clone the watcher
            system_update_channel: self.system_update_channel.clone(),
            last_update_request: self.last_update_request.clone(),
            update_counters: self.update_counters.clone(),
        }
    }
}

/// Generate a unique event ID
fn generate_event_id() -> String {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let random_part: u64 = rng.gen();
    format!("evt-{:x}-{}", random_part, current_timestamp())
}

/// Get the current timestamp in milliseconds
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis() as u64
}

/// Calculate a hash of content for change detection
fn calculate_content_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

/// Extract agent name from a file path
fn extract_agent_name_from_path(path: &Path) -> Option<String> {
    // Convert to string for easier processing
    let path_str = path.to_string_lossy();
    
    // Look for patterns like AGENTS/AgentName/ or agents/AgentName/
    let re = regex::Regex::new(r"(?i)AGENTS[/\\]([^/\\]+)[/\\]").unwrap();
    
    if let Some(caps) = re.captures(&path_str) {
        if let Some(agent_name) = caps.get(1) {
            return Some(agent_name.as_str().to_string());
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    
    #[tokio::test]
    async fn test_memory_streamer_basics() {
        // Create temporary directory for test files
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test agent directories and files
        let agent_dir = temp_path.join("AGENTS").join("TestAgent");
        fs::create_dir_all(&agent_dir).unwrap();
        
        let memory_file = agent_dir.join("MEMORY.md");
        let mut file = File::create(&memory_file).unwrap();
        writeln!(file, "# Test Agent Memory\n\nThis is a test memory file.").unwrap();
        
        // Create memory loader and checksum cache
        let memory_loader = Arc::new(Mutex::new(MemoryLoader::default()));
        let checksum_cache = Arc::new(Mutex::new(MemoryChecksumCache::default()));
        
        // Create streamer with custom config
        let mut config = StreamConfig::default();
        config.watch_directories = vec![temp_path.join("AGENTS").to_string_lossy().to_string()];
        
        let streamer = MemoryStreamer::new(config, memory_loader, checksum_cache).unwrap();
        
        // Subscribe to an agent
        let _rx = streamer.subscribe_to_agent("TestAgent").await.unwrap();
        
        // Verify the agent has an active stream
        assert_eq!(streamer.get_stream_status("TestAgent"), Some(StreamStatus::Active));
        
        // Test pausing and resuming
        streamer.pause_stream("TestAgent").unwrap();
        assert_eq!(streamer.get_stream_status("TestAgent"), Some(StreamStatus::Paused));
        
        streamer.resume_stream("TestAgent").unwrap();
        assert_eq!(streamer.get_stream_status("TestAgent"), Some(StreamStatus::Active));
        
        // Unsubscribe
        streamer.unsubscribe_from_agent("TestAgent").unwrap();
        assert_eq!(streamer.get_stream_status("TestAgent"), None);
    }
    
    #[test]
    fn test_extract_agent_name() {
        let test_cases = vec![
            ("/Users/user/AGENTS/TestAgent/MEMORY.md", Some("TestAgent")),
            ("/Users/user/AGENTS/TestAgent/Sessions/Test/file.md", Some("TestAgent")),
            ("/Users/user/agents/TestAgent/MEMORY.md", Some("TestAgent")), // Case insensitive
            ("/Users/user/Projects/SomeOtherPath/file.md", None),
            ("/AGENTS/TestAgent/file.md", Some("TestAgent")),
            ("C:\\AGENTS\\TestAgent\\file.md", Some("TestAgent")), // Windows path
        ];
        
        for (path_str, expected) in test_cases {
            let path = PathBuf::from(path_str);
            let result = extract_agent_name_from_path(&path);
            assert_eq!(result, expected.map(String::from));
        }
    }
}