use std::collections::{HashMap, HashSet, BTreeMap};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use dashmap::DashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tantivy::{Index, Document, schema::*, IndexWriter};

use crate::memory_segment_parser::{MemorySegment, MemoryType};
use crate::memory_loader::MemoryLoader;

/// Configuration for memory indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Directory where indexes will be stored
    pub index_directory: String,
    /// Whether to index all memory by default
    pub index_all_memory: bool,
    /// Maximum documents per index
    pub max_docs_per_index: usize,
    /// Memory types to index
    pub indexed_memory_types: Vec<MemoryType>,
    /// Whether to enable context-based indexing
    pub enable_context_indexing: bool,
    /// Whether to store full content in the index
    pub store_full_content: bool,
    /// Whether to enable incremental updating
    pub enable_incremental_updates: bool,
    /// How many days to keep segment history
    pub segment_history_days: u32,
    /// Index reindexing interval in days
    pub reindex_interval_days: u32,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            index_directory: "indexes".to_string(),
            index_all_memory: true,
            max_docs_per_index: 100000,
            indexed_memory_types: vec![
                MemoryType::Semantic,
                MemoryType::Episodic,
                MemoryType::Procedural,
                MemoryType::Relational,
                MemoryType::Identity,
            ],
            enable_context_indexing: true,
            store_full_content: true,
            enable_incremental_updates: true,
            segment_history_days: 30,
            reindex_interval_days: 7,
        }
    }
}

/// Represents the type of memory index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndexType {
    /// Agent-specific memory index
    Agent,
    /// System-wide memory index
    System,
    /// Context-specific memory index
    Context,
    /// Time-based memory index
    Temporal,
    /// Memory type-specific index
    MemoryType,
    /// Relation-based memory index
    Relational,
    /// Hierarchical concept index
    Hierarchical,
}

/// Metadata about an index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    /// Unique identifier for the index
    pub id: String,
    /// Type of index
    pub index_type: IndexType,
    /// Name or scope of the index
    pub name: String,
    /// When the index was created
    pub created_timestamp: u64,
    /// When the index was last updated
    pub last_updated_timestamp: u64,
    /// Number of documents in the index
    pub document_count: usize,
    /// Additional custom metadata
    pub custom_metadata: HashMap<String, String>,
    /// Path to the index on disk
    pub path: PathBuf,
}

/// Hierarchical structure for organizing memory concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchicalNode {
    /// Node identifier
    pub id: String,
    /// Node name
    pub name: String,
    /// Node description
    pub description: String,
    /// Parent node ID (if any)
    pub parent_id: Option<String>,
    /// Child node IDs
    pub child_ids: Vec<String>,
    /// Related segment IDs
    pub segment_ids: Vec<String>,
    /// Node depth in the hierarchy (0 = root)
    pub depth: usize,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Search query for memory indexes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    /// Free text query
    pub query_text: String,
    /// Filter by agent name
    pub agent_name: Option<String>,
    /// Filter by memory type
    pub memory_type: Option<MemoryType>,
    /// Filter by time range (start timestamp)
    pub time_start: Option<u64>,
    /// Filter by time range (end timestamp)
    pub time_end: Option<u64>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Maximum results to return
    pub limit: usize,
    /// Context for contextual search
    pub context: Option<String>,
    /// Whether to include related segments
    pub include_related: bool,
    /// Custom filters
    pub custom_filters: HashMap<String, String>,
}

impl Default for MemoryQuery {
    fn default() -> Self {
        Self {
            query_text: String::new(),
            agent_name: None,
            memory_type: None,
            time_start: None,
            time_end: None,
            tags: Vec::new(),
            limit: 10,
            context: None,
            include_related: false,
            custom_filters: HashMap::new(),
        }
    }
}

/// Result of a memory search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Matching memory segments
    pub segments: Vec<MemorySegment>,
    /// Search score for each segment
    pub scores: HashMap<String, f32>,
    /// Search highlights for each segment
    pub highlights: HashMap<String, Vec<String>>,
    /// Total number of matches (may be more than returned)
    pub total_matches: usize,
    /// Search execution time in milliseconds
    pub execution_time_ms: u64,
    /// Index used for the search
    pub index_used: String,
}

/// Memory indexing system for the Collaborative Intelligence platform
pub struct MemoryIndexes {
    /// Configuration for indexing
    config: IndexConfig,
    /// Reference to memory loader for retrieving segments
    memory_loader: Arc<Mutex<MemoryLoader>>,
    /// Available indexes with metadata
    indexes: DashMap<String, IndexMetadata>,
    /// Tantivy indexes for full-text search
    search_indexes: DashMap<String, Arc<Index>>,
    /// Index writers for updating indexes
    index_writers: DashMap<String, Arc<Mutex<IndexWriter>>>,
    /// Hierarchical concept model
    concept_hierarchy: Arc<RwLock<HashMap<String, HierarchicalNode>>>,
    /// Agent-specific hierarchies
    agent_hierarchies: DashMap<String, Arc<RwLock<HashMap<String, HierarchicalNode>>>>,
    /// Relational indexes for connecting related memory
    relational_indexes: DashMap<String, HashMap<String, HashSet<String>>>,
    /// Cache of recently searched segments
    search_cache: DashMap<String, (Instant, SearchResult)>,
}

impl MemoryIndexes {
    /// Create a new memory indexes instance with the given configuration
    pub fn new(config: IndexConfig, memory_loader: Arc<Mutex<MemoryLoader>>) -> Result<Self, Box<dyn Error>> {
        // Create index directory if it doesn't exist
        fs::create_dir_all(&config.index_directory)?;
        
        // Initialize the indexes system
        let indexes = Self {
            config,
            memory_loader,
            indexes: DashMap::new(),
            search_indexes: DashMap::new(),
            index_writers: DashMap::new(),
            concept_hierarchy: Arc::new(RwLock::new(HashMap::new())),
            agent_hierarchies: DashMap::new(),
            relational_indexes: DashMap::new(),
            search_cache: DashMap::new(),
        };
        
        // Load existing indexes
        indexes.load_existing_indexes()?;
        
        // Initialize base hierarchical structure
        indexes.initialize_base_hierarchy()?;
        
        Ok(indexes)
    }

    /// Create a new memory indexes instance with default configuration
    pub fn default(memory_loader: Arc<Mutex<MemoryLoader>>) -> Result<Self, Box<dyn Error>> {
        Self::new(IndexConfig::default(), memory_loader)
    }

    /// Load existing indexes from disk
    fn load_existing_indexes(&self) -> Result<(), Box<dyn Error>> {
        // Check if index directory exists
        let index_dir = Path::new(&self.config.index_directory);
        if !index_dir.exists() {
            return Ok(());
        }
        
        // Load index metadata files
        for entry in fs::read_dir(index_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Look for metadata files
            if path.is_dir() {
                let metadata_path = path.join("metadata.json");
                if metadata_path.exists() {
                    // Read and parse metadata
                    let metadata_str = fs::read_to_string(metadata_path)?;
                    let metadata: IndexMetadata = serde_json::from_str(&metadata_str)?;
                    
                    // Open the index
                    match self.open_index(&metadata) {
                        Ok(_) => {
                            // Store metadata
                            self.indexes.insert(metadata.id.clone(), metadata);
                        },
                        Err(e) => {
                            eprintln!("Failed to open index {}: {}", metadata.id, e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Open an existing index
    fn open_index(&self, metadata: &IndexMetadata) -> Result<(), Box<dyn Error>> {
        // Open the Tantivy index
        let index = Index::open_in_dir(&metadata.path)?;
        
        // Create a writer for updates
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        // Store the index and writer
        self.search_indexes.insert(metadata.id.clone(), Arc::new(index));
        self.index_writers.insert(metadata.id.clone(), Arc::new(Mutex::new(writer)));
        
        Ok(())
    }

    /// Initialize the base hierarchical structure
    fn initialize_base_hierarchy(&self) -> Result<(), Box<dyn Error>> {
        let mut hierarchy = self.concept_hierarchy.write().map_err(|e| format!("Failed to lock hierarchy: {}", e))?;
        
        // Only initialize if empty
        if !hierarchy.is_empty() {
            return Ok(());
        }
        
        // Create root node
        let root = HierarchicalNode {
            id: "root".to_string(),
            name: "Root".to_string(),
            description: "Root of the memory hierarchy".to_string(),
            parent_id: None,
            child_ids: Vec::new(),
            segment_ids: Vec::new(),
            depth: 0,
            metadata: HashMap::new(),
        };
        
        // Add memory type nodes
        let memory_types = [
            ("identity", "Identity Memory", "Core agent identity information"),
            ("semantic", "Semantic Memory", "Factual knowledge and concepts"),
            ("episodic", "Episodic Memory", "Experience-based memory"),
            ("procedural", "Procedural Memory", "Skill-based memory and processes"),
            ("relational", "Relational Memory", "Relationships between concepts"),
        ];
        
        let mut root_children = Vec::new();
        
        for (id_suffix, name, description) in memory_types {
            let id = format!("type_{}", id_suffix);
            let node = HierarchicalNode {
                id: id.clone(),
                name: name.to_string(),
                description: description.to_string(),
                parent_id: Some("root".to_string()),
                child_ids: Vec::new(),
                segment_ids: Vec::new(),
                depth: 1,
                metadata: HashMap::new(),
            };
            
            hierarchy.insert(id.clone(), node);
            root_children.push(id);
        }
        
        // Update root with children
        let mut root = root;
        root.child_ids = root_children;
        hierarchy.insert(root.id.clone(), root);
        
        Ok(())
    }

    /// Create a new agent-specific memory index
    pub fn create_agent_index(&self, agent_name: &str) -> Result<String, Box<dyn Error>> {
        // Generate a unique index ID
        let index_id = format!("agent_{}_{}", agent_name, current_timestamp());
        
        // Create index directory
        let index_dir = Path::new(&self.config.index_directory).join(&index_id);
        fs::create_dir_all(&index_dir)?;
        
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Define fields
        let id = schema_builder.add_text_field("id", TEXT | STORED);
        let agent = schema_builder.add_text_field("agent", TEXT | STORED);
        let memory_type = schema_builder.add_text_field("memory_type", TEXT | STORED);
        let tier = schema_builder.add_u64_field("tier", INDEXED | STORED);
        let content = schema_builder.add_text_field("content", TEXT | STORED);
        let tags = schema_builder.add_text_field("tags", TEXT | STORED);
        let created = schema_builder.add_date_field("created", INDEXED | STORED);
        let updated = schema_builder.add_date_field("updated", INDEXED | STORED);
        
        let schema = schema_builder.build();
        
        // Create the index
        let index = Index::create_in_dir(&index_dir, schema)?;
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        // Create metadata
        let metadata = IndexMetadata {
            id: index_id.clone(),
            index_type: IndexType::Agent,
            name: agent_name.to_string(),
            created_timestamp: current_timestamp(),
            last_updated_timestamp: current_timestamp(),
            document_count: 0,
            custom_metadata: HashMap::from([
                ("schema_version".to_string(), "1".to_string()),
                ("agent".to_string(), agent_name.to_string()),
            ]),
            path: index_dir,
        };
        
        // Save metadata
        self.save_index_metadata(&metadata)?;
        
        // Store the index
        self.search_indexes.insert(index_id.clone(), Arc::new(index));
        self.index_writers.insert(index_id.clone(), Arc::new(Mutex::new(writer)));
        self.indexes.insert(index_id.clone(), metadata);
        
        // Now initialize the agent's hierarchy
        self.initialize_agent_hierarchy(agent_name)?;
        
        Ok(index_id)
    }

    /// Initialize an agent's hierarchical structure
    fn initialize_agent_hierarchy(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Create a new hierarchy or get existing one
        let hierarchy = self.agent_hierarchies
            .entry(agent_name.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(HashMap::new())));
        
        // Lock for writing
        let mut hierarchy = hierarchy.write().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
        
        // Only initialize if empty
        if !hierarchy.is_empty() {
            return Ok(());
        }
        
        // Create root node for the agent
        let root = HierarchicalNode {
            id: format!("{}_root", agent_name),
            name: format!("{} Root", agent_name),
            description: format!("Root of {}'s memory hierarchy", agent_name),
            parent_id: None,
            child_ids: Vec::new(),
            segment_ids: Vec::new(),
            depth: 0,
            metadata: HashMap::from([
                ("agent".to_string(), agent_name.to_string()),
            ]),
        };
        
        // Add memory type nodes
        let memory_types = [
            ("identity", "Identity Memory", "Core identity information"),
            ("semantic", "Semantic Memory", "Factual knowledge and concepts"),
            ("episodic", "Episodic Memory", "Experience-based memory"),
            ("procedural", "Procedural Memory", "Skill-based memory and processes"),
            ("relational", "Relational Memory", "Relationships between concepts"),
        ];
        
        let mut root_children = Vec::new();
        
        for (id_suffix, name, description) in memory_types {
            let id = format!("{}_{}", agent_name, id_suffix);
            let node = HierarchicalNode {
                id: id.clone(),
                name: name.to_string(),
                description: description.to_string(),
                parent_id: Some(format!("{}_root", agent_name)),
                child_ids: Vec::new(),
                segment_ids: Vec::new(),
                depth: 1,
                metadata: HashMap::from([
                    ("agent".to_string(), agent_name.to_string()),
                    ("type".to_string(), id_suffix.to_string()),
                ]),
            };
            
            hierarchy.insert(id.clone(), node);
            root_children.push(id);
        }
        
        // Update root with children
        let mut root = root;
        root.child_ids = root_children;
        hierarchy.insert(root.id.clone(), root);
        
        Ok(())
    }

    /// Save index metadata to disk
    fn save_index_metadata(&self, metadata: &IndexMetadata) -> Result<(), Box<dyn Error>> {
        let metadata_path = metadata.path.join("metadata.json");
        let file = File::create(metadata_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, metadata)?;
        Ok(())
    }

    /// Create a system-wide memory index
    pub fn create_system_index(&self) -> Result<String, Box<dyn Error>> {
        // Generate a unique index ID
        let index_id = format!("system_{}", current_timestamp());
        
        // Create index directory
        let index_dir = Path::new(&self.config.index_directory).join(&index_id);
        fs::create_dir_all(&index_dir)?;
        
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Define fields (same as agent index but with additional system-specific fields)
        let id = schema_builder.add_text_field("id", TEXT | STORED);
        let agent = schema_builder.add_text_field("agent", TEXT | STORED);
        let memory_type = schema_builder.add_text_field("memory_type", TEXT | STORED);
        let tier = schema_builder.add_u64_field("tier", INDEXED | STORED);
        let content = schema_builder.add_text_field("content", TEXT | STORED);
        let tags = schema_builder.add_text_field("tags", TEXT | STORED);
        let created = schema_builder.add_date_field("created", INDEXED | STORED);
        let updated = schema_builder.add_date_field("updated", INDEXED | STORED);
        let system_tags = schema_builder.add_text_field("system_tags", TEXT | STORED);
        let is_shared = schema_builder.add_bool_field("is_shared", INDEXED | STORED);
        
        let schema = schema_builder.build();
        
        // Create the index
        let index = Index::create_in_dir(&index_dir, schema)?;
        let writer = index.writer(100_000_000)?; // 100MB buffer for system index
        
        // Create metadata
        let metadata = IndexMetadata {
            id: index_id.clone(),
            index_type: IndexType::System,
            name: "System".to_string(),
            created_timestamp: current_timestamp(),
            last_updated_timestamp: current_timestamp(),
            document_count: 0,
            custom_metadata: HashMap::from([
                ("schema_version".to_string(), "1".to_string()),
                ("scope".to_string(), "global".to_string()),
            ]),
            path: index_dir,
        };
        
        // Save metadata
        self.save_index_metadata(&metadata)?;
        
        // Store the index
        self.search_indexes.insert(index_id.clone(), Arc::new(index));
        self.index_writers.insert(index_id.clone(), Arc::new(Mutex::new(writer)));
        self.indexes.insert(index_id.clone(), metadata);
        
        Ok(index_id)
    }

    /// Index all memory for an agent
    pub fn index_agent_memory(&self, agent_name: &str) -> Result<usize, Box<dyn Error>> {
        // Find or create an index for this agent
        let index_id = self.get_agent_index_id(agent_name)?;
        
        // Get all memory segments for the agent
        let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
        let segments = memory_loader.load_agent_memory(agent_name, None)?;
        drop(memory_loader); // Release lock
        
        // Index the segments
        let count = self.index_segments(index_id, &segments)?;
        
        // Update the agent's hierarchy with these segments
        self.update_agent_hierarchy(agent_name, &segments)?;
        
        Ok(count)
    }

    /// Get the index ID for an agent, creating one if it doesn't exist
    fn get_agent_index_id(&self, agent_name: &str) -> Result<String, Box<dyn Error>> {
        // Look for an existing index for this agent
        for entry in self.indexes.iter() {
            let metadata = entry.value();
            if metadata.index_type == IndexType::Agent && metadata.name == agent_name {
                return Ok(metadata.id.clone());
            }
        }
        
        // No existing index, create a new one
        self.create_agent_index(agent_name)
    }

    /// Index a collection of memory segments
    fn index_segments(&self, index_id: String, segments: &[MemorySegment]) -> Result<usize, Box<dyn Error>> {
        // Get the index and writer
        let index_arc = self.search_indexes.get(&index_id).ok_or_else(|| format!("Index not found: {}", index_id))?;
        let index = &*index_arc;
        
        let writer_arc = self.index_writers.get(&index_id).ok_or_else(|| format!("Writer not found: {}", index_id))?;
        let mut writer = writer_arc.lock().map_err(|e| format!("Failed to lock writer: {}", e))?;
        
        // Get the schema
        let schema = index.schema();
        
        // Get field accessors
        let id_field = schema.get_field("id").unwrap();
        let agent_field = schema.get_field("agent").unwrap();
        let memory_type_field = schema.get_field("memory_type").unwrap();
        let tier_field = schema.get_field("tier").unwrap();
        let content_field = schema.get_field("content").unwrap();
        let tags_field = schema.get_field("tags").unwrap();
        let created_field = schema.get_field("created").unwrap();
        let updated_field = schema.get_field("updated").unwrap();
        
        // Batch index the segments
        let now = tantivy::DateTime::from_timestamp_secs(current_timestamp() as i64 / 1000);
        
        for segment in segments {
            // Extract tags from metadata
            let tags = segment.metadata.get("tags").cloned().unwrap_or_default();
            
            // Create a document
            let mut doc = Document::new();
            doc.add_text(id_field, &segment.id);
            doc.add_text(agent_field, segment.metadata.get("agent").unwrap_or(&String::new()));
            doc.add_text(memory_type_field, format!("{:?}", segment.memory_type));
            doc.add_u64(tier_field, segment.tier as u64);
            doc.add_text(content_field, &segment.content);
            doc.add_text(tags_field, &tags);
            doc.add_date(created_field, now);
            doc.add_date(updated_field, now);
            
            // Add the document to the index
            writer.add_document(doc)?;
        }
        
        // Commit changes
        writer.commit()?;
        
        // Update metadata
        if let Some(mut metadata) = self.indexes.get_mut(&index_id) {
            metadata.document_count += segments.len();
            metadata.last_updated_timestamp = current_timestamp();
            self.save_index_metadata(&metadata)?;
        }
        
        Ok(segments.len())
    }

    /// Update an agent's hierarchical structure with new segments
    fn update_agent_hierarchy(&self, agent_name: &str, segments: &[MemorySegment]) -> Result<(), Box<dyn Error>> {
        // Get or create the agent's hierarchy
        if !self.agent_hierarchies.contains_key(agent_name) {
            self.initialize_agent_hierarchy(agent_name)?;
        }
        
        let hierarchy_arc = self.agent_hierarchies.get(agent_name).ok_or_else(|| format!("Hierarchy not found for agent: {}", agent_name))?;
        let mut hierarchy = hierarchy_arc.write().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
        
        // Process each segment and add to the appropriate node
        for segment in segments {
            // Determine which node this segment belongs to
            let node_id = match segment.memory_type {
                MemoryType::Identity => format!("{}_identity", agent_name),
                MemoryType::Semantic => format!("{}_semantic", agent_name),
                MemoryType::Episodic => format!("{}_episodic", agent_name),
                MemoryType::Procedural => format!("{}_procedural", agent_name),
                MemoryType::Relational => format!("{}_relational", agent_name),
            };
            
            // Add segment to the node
            if let Some(mut node) = hierarchy.get_mut(&node_id) {
                if !node.segment_ids.contains(&segment.id) {
                    node.segment_ids.push(segment.id.clone());
                }
            }
            
            // Additionally, try to extract concepts and create/update concept nodes
            self.extract_and_update_concepts(agent_name, segment, &mut hierarchy)?;
        }
        
        Ok(())
    }

    /// Extract concepts from a segment and update the hierarchy
    fn extract_and_update_concepts(
        &self, 
        agent_name: &str, 
        segment: &MemorySegment, 
        hierarchy: &mut HashMap<String, HierarchicalNode>
    ) -> Result<(), Box<dyn Error>> {
        // Skip certain memory types that don't benefit from concept extraction
        if segment.memory_type == MemoryType::Identity {
            return Ok(());
        }
        
        // Basic concept extraction - find headings in the content
        let lines: Vec<&str> = segment.content.lines().collect();
        let mut concepts = Vec::new();
        
        for line in lines {
            // Look for markdown headings
            if line.starts_with("# ") {
                let concept = line[2..].trim().to_string();
                if !concept.is_empty() {
                    concepts.push((concept, 1)); // Level 1 heading
                }
            } else if line.starts_with("## ") {
                let concept = line[3..].trim().to_string();
                if !concept.is_empty() {
                    concepts.push((concept, 2)); // Level 2 heading
                }
            } else if line.starts_with("### ") {
                let concept = line[4..].trim().to_string();
                if !concept.is_empty() {
                    concepts.push((concept, 3)); // Level 3 heading
                }
            }
        }
        
        // Create or update concept nodes
        let memory_type_node_id = match segment.memory_type {
            MemoryType::Semantic => format!("{}_semantic", agent_name),
            MemoryType::Episodic => format!("{}_episodic", agent_name),
            MemoryType::Procedural => format!("{}_procedural", agent_name),
            MemoryType::Relational => format!("{}_relational", agent_name),
            _ => format!("{}_root", agent_name),
        };
        
        // Process extracted concepts
        for (concept, level) in concepts {
            let concept_id = format!("{}_{}", agent_name, sanitize_for_id(&concept));
            
            // Create a new node if it doesn't exist
            if !hierarchy.contains_key(&concept_id) {
                let parent_id = if level == 1 {
                    // Top-level concepts are children of the memory type node
                    Some(memory_type_node_id.clone())
                } else {
                    // Look for parent concept based on structure
                    None // For simplicity, making all concepts children of memory type
                };
                
                let node = HierarchicalNode {
                    id: concept_id.clone(),
                    name: concept.clone(),
                    description: format!("Concept extracted from segment {}", segment.id),
                    parent_id,
                    child_ids: Vec::new(),
                    segment_ids: vec![segment.id.clone()],
                    depth: level as usize,
                    metadata: HashMap::from([
                        ("agent".to_string(), agent_name.to_string()),
                        ("type".to_string(), "concept".to_string()),
                        ("memory_type".to_string(), format!("{:?}", segment.memory_type)),
                    ]),
                };
                
                // Add to hierarchy
                hierarchy.insert(concept_id.clone(), node);
                
                // Update parent's children list
                if let Some(parent_id) = &parent_id {
                    if let Some(mut parent) = hierarchy.get_mut(parent_id) {
                        if !parent.child_ids.contains(&concept_id) {
                            parent.child_ids.push(concept_id.clone());
                        }
                    }
                }
            } else {
                // Update existing node
                if let Some(mut node) = hierarchy.get_mut(&concept_id) {
                    if !node.segment_ids.contains(&segment.id) {
                        node.segment_ids.push(segment.id.clone());
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Search for memory segments matching a query
    pub fn search(&self, query: &MemoryQuery) -> Result<SearchResult, Box<dyn Error>> {
        // Check cache first
        let cache_key = format!("{:?}", query);
        if let Some(cached) = self.search_cache.get(&cache_key) {
            // Use cache if not expired (5 seconds TTL)
            if cached.0.elapsed().as_secs() < 5 {
                return Ok(cached.1.clone());
            }
        }
        
        // Determine which index to use
        let index_id = if let Some(agent_name) = &query.agent_name {
            // Use agent-specific index
            self.get_agent_index_id(agent_name)?
        } else {
            // Use system-wide index
            self.get_system_index_id()?
        };
        
        // Start timing the search
        let start_time = Instant::now();
        
        // Get the index
        let index_arc = self.search_indexes.get(&index_id).ok_or_else(|| format!("Index not found: {}", index_id))?;
        let index = &*index_arc;
        
        // Create a searcher
        let reader = index.reader()?;
        let searcher = reader.searcher();
        
        // Parse query
        let schema = index.schema();
        let content_field = schema.get_field("content").unwrap();
        let memory_type_field = schema.get_field("memory_type").unwrap();
        let agent_field = schema.get_field("agent").unwrap();
        let id_field = schema.get_field("id").unwrap();
        
        // Build query
        let mut query_parser = QueryParser::for_index(index, vec![content_field]);
        
        let mut query_terms = vec![query.query_text.clone()];
        
        // Add filters
        let mut filters = Vec::new();
        
        // Agent filter
        if let Some(agent_name) = &query.agent_name {
            filters.push(format!("agent:{}", agent_name));
        }
        
        // Memory type filter
        if let Some(memory_type) = &query.memory_type {
            filters.push(format!("memory_type:{:?}", memory_type));
        }
        
        // Tag filters
        for tag in &query.tags {
            filters.push(format!("tags:{}", tag));
        }
        
        // Time range filters
        if let Some(start) = query.time_start {
            filters.push(format!("created:>={}", start));
        }
        
        if let Some(end) = query.time_end {
            filters.push(format!("created:<={}", end));
        }
        
        // Join filters with main query
        if !filters.is_empty() {
            query_terms.push(format!("+({})", filters.join(" ")));
        }
        
        // Build final query string
        let query_str = query_terms.join(" ");
        let parsed_query = query_parser.parse_query(&query_str)?;
        
        // Execute search
        let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(query.limit))?;
        
        // Collect results
        let mut segments = Vec::new();
        let mut scores = HashMap::new();
        let mut highlights = HashMap::new();
        
        // Helper for highlighting
        let highlighter = SnippetGenerator::create(&searcher, &parsed_query, content_field)?;
        
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            
            // Extract fields
            let id = retrieved_doc.get_first(id_field).and_then(|f| f.as_text()).unwrap_or("");
            let agent = retrieved_doc.get_first(agent_field).and_then(|f| f.as_text()).unwrap_or("");
            let memory_type_str = retrieved_doc.get_first(memory_type_field).and_then(|f| f.as_text()).unwrap_or("");
            let content = retrieved_doc.get_first(content_field).and_then(|f| f.as_text()).unwrap_or("");
            
            // Parse memory type
            let memory_type = match memory_type_str {
                "Identity" => MemoryType::Identity,
                "Semantic" => MemoryType::Semantic,
                "Episodic" => MemoryType::Episodic,
                "Procedural" => MemoryType::Procedural,
                "Relational" => MemoryType::Relational,
                _ => MemoryType::Semantic, // Default to semantic
            };
            
            // Create a segment from the document
            let segment = MemorySegment {
                id: id.to_string(),
                memory_type,
                tier: 1, // Default tier
                content: content.to_string(),
                metadata: HashMap::from([
                    ("agent".to_string(), agent.to_string()),
                ]),
                activation_score: score as f64,
                last_accessed_timestamp: Some(current_timestamp()),
                related_segments: Vec::new(),
            };
            
            // Add to results
            segments.push(segment);
            scores.insert(id.to_string(), score);
            
            // Generate highlights
            let snippet = highlighter.snippet(content);
            highlights.insert(id.to_string(), snippet.fragments().iter().map(|f| f.to_string()).collect());
        }
        
        // Include related segments if requested
        if query.include_related && !segments.is_empty() {
            let related = self.find_related_segments(&segments)?;
            segments.extend(related);
        }
        
        // Create result
        let result = SearchResult {
            segments,
            scores,
            highlights,
            total_matches: top_docs.len(),
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            index_used: index_id,
        };
        
        // Cache the result
        self.search_cache.insert(cache_key, (Instant::now(), result.clone()));
        
        Ok(result)
    }

    /// Find related segments for a collection of segments
    fn find_related_segments(&self, segments: &[MemorySegment]) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        let mut related_ids = HashSet::new();
        
        // Collect IDs from segment relations
        for segment in segments {
            for related in &segment.related_segments {
                related_ids.insert(related.clone());
            }
        }
        
        // Also check relational indexes
        for segment in segments {
            if let Some(relations) = self.relational_indexes.get(&segment.id) {
                for related_id in relations.keys() {
                    related_ids.insert(related_id.clone());
                }
            }
        }
        
        // Remove IDs we already have
        for segment in segments {
            related_ids.remove(&segment.id);
        }
        
        // Fetch the related segments
        let mut related_segments = Vec::new();
        
        if !related_ids.is_empty() {
            // Try to load the segments from memory loader
            let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
            
            for id in related_ids {
                if let Ok(segment) = memory_loader.load_segment_by_id(&id) {
                    related_segments.push(segment);
                }
            }
        }
        
        Ok(related_segments)
    }

    /// Get the system index ID, creating one if it doesn't exist
    fn get_system_index_id(&self) -> Result<String, Box<dyn Error>> {
        // Look for an existing system index
        for entry in self.indexes.iter() {
            let metadata = entry.value();
            if metadata.index_type == IndexType::System {
                return Ok(metadata.id.clone());
            }
        }
        
        // No existing index, create a new one
        self.create_system_index()
    }

    /// Create a hierarchical index for conceptual organization
    pub fn create_hierarchical_index(&self) -> Result<String, Box<dyn Error>> {
        // Generate a unique index ID
        let index_id = format!("hierarchical_{}", current_timestamp());
        
        // Create index directory
        let index_dir = Path::new(&self.config.index_directory).join(&index_id);
        fs::create_dir_all(&index_dir)?;
        
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Define fields
        let id = schema_builder.add_text_field("id", TEXT | STORED);
        let name = schema_builder.add_text_field("name", TEXT | STORED);
        let path = schema_builder.add_text_field("path", TEXT | STORED);
        let level = schema_builder.add_u64_field("level", INDEXED | STORED);
        let parent = schema_builder.add_text_field("parent", TEXT | STORED);
        let children = schema_builder.add_text_field("children", TEXT | STORED);
        let segments = schema_builder.add_text_field("segments", TEXT | STORED);
        let description = schema_builder.add_text_field("description", TEXT | STORED);
        
        let schema = schema_builder.build();
        
        // Create the index
        let index = Index::create_in_dir(&index_dir, schema)?;
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        // Create metadata
        let metadata = IndexMetadata {
            id: index_id.clone(),
            index_type: IndexType::Hierarchical,
            name: "Concept Hierarchy".to_string(),
            created_timestamp: current_timestamp(),
            last_updated_timestamp: current_timestamp(),
            document_count: 0,
            custom_metadata: HashMap::from([
                ("schema_version".to_string(), "1".to_string()),
                ("type".to_string(), "hierarchical".to_string()),
            ]),
            path: index_dir,
        };
        
        // Save metadata
        self.save_index_metadata(&metadata)?;
        
        // Store the index
        self.search_indexes.insert(index_id.clone(), Arc::new(index));
        self.index_writers.insert(index_id.clone(), Arc::new(Mutex::new(writer)));
        self.indexes.insert(index_id.clone(), metadata);
        
        Ok(index_id)
    }

    /// Build and index the global concept hierarchy
    pub fn build_global_hierarchy(&self) -> Result<(), Box<dyn Error>> {
        // Create a hierarchical index
        let index_id = self.create_hierarchical_index()?;
        
        // Get the index and writer
        let index_arc = self.search_indexes.get(&index_id).ok_or_else(|| format!("Index not found: {}", index_id))?;
        let index = &*index_arc;
        
        let writer_arc = self.index_writers.get(&index_id).ok_or_else(|| format!("Writer not found: {}", index_id))?;
        let mut writer = writer_arc.lock().map_err(|e| format!("Failed to lock writer: {}", e))?;
        
        // Get the schema
        let schema = index.schema();
        
        // Get field accessors
        let id_field = schema.get_field("id").unwrap();
        let name_field = schema.get_field("name").unwrap();
        let path_field = schema.get_field("path").unwrap();
        let level_field = schema.get_field("level").unwrap();
        let parent_field = schema.get_field("parent").unwrap();
        let children_field = schema.get_field("children").unwrap();
        let segments_field = schema.get_field("segments").unwrap();
        let description_field = schema.get_field("description").unwrap();
        
        // Integrate hierarchy nodes from all agents
        let mut global_nodes = HashMap::new();
        
        // Start with the root hierarchy
        {
            let root_hierarchy = self.concept_hierarchy.read().map_err(|e| format!("Failed to lock root hierarchy: {}", e))?;
            for (id, node) in root_hierarchy.iter() {
                global_nodes.insert(id.clone(), node.clone());
            }
        }
        
        // Process each agent's hierarchy
        for entry in self.agent_hierarchies.iter() {
            let agent_name = entry.key();
            let hierarchy_arc = entry.value();
            
            let agent_hierarchy = hierarchy_arc.read().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
            for (id, node) in agent_hierarchy.iter() {
                // Skip root and basic type nodes - they're redundant
                if id.ends_with("_root") || 
                   id.ends_with("_identity") || 
                   id.ends_with("_semantic") || 
                   id.ends_with("_episodic") || 
                   id.ends_with("_procedural") || 
                   id.ends_with("_relational") {
                    continue;
                }
                
                // Add to global nodes
                global_nodes.insert(id.clone(), node.clone());
            }
        }
        
        // Index all nodes
        let mut indexed_count = 0;
        
        for (id, node) in global_nodes {
            // Find the full path to this node
            let path = self.calculate_node_path(&global_nodes, &node);
            
            // Create a document
            let mut doc = Document::new();
            doc.add_text(id_field, &node.id);
            doc.add_text(name_field, &node.name);
            doc.add_text(path_field, &path);
            doc.add_u64(level_field, node.depth as u64);
            
            if let Some(parent_id) = &node.parent_id {
                doc.add_text(parent_field, parent_id);
            }
            
            if !node.child_ids.is_empty() {
                doc.add_text(children_field, node.child_ids.join(","));
            }
            
            if !node.segment_ids.is_empty() {
                doc.add_text(segments_field, node.segment_ids.join(","));
            }
            
            doc.add_text(description_field, &node.description);
            
            // Add to index
            writer.add_document(doc)?;
            indexed_count += 1;
        }
        
        // Commit changes
        writer.commit()?;
        
        // Update metadata
        if let Some(mut metadata) = self.indexes.get_mut(&index_id) {
            metadata.document_count = indexed_count;
            metadata.last_updated_timestamp = current_timestamp();
            self.save_index_metadata(&metadata)?;
        }
        
        Ok(())
    }

    /// Calculate the full path to a node in the hierarchy
    fn calculate_node_path(&self, nodes: &HashMap<String, HierarchicalNode>, node: &HierarchicalNode) -> String {
        let mut path_parts = Vec::new();
        let mut current = Some(node);
        
        while let Some(n) = current {
            path_parts.push(n.name.clone());
            
            current = if let Some(parent_id) = &n.parent_id {
                nodes.get(parent_id)
            } else {
                None
            };
        }
        
        // Reverse to get root-to-leaf order
        path_parts.reverse();
        path_parts.join(" > ")
    }

    /// Merge agent-specific hierarchies into the global hierarchy
    pub fn merge_agent_hierarchies(&self) -> Result<(), Box<dyn Error>> {
        // Get the global hierarchy for writing
        let mut global_hierarchy = self.concept_hierarchy.write().map_err(|e| format!("Failed to lock global hierarchy: {}", e))?;
        
        // Process each agent's hierarchy
        for entry in self.agent_hierarchies.iter() {
            let agent_name = entry.key();
            let hierarchy_arc = entry.value();
            
            let agent_hierarchy = hierarchy_arc.read().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
            
            // Look for concept nodes to merge
            for (id, node) in agent_hierarchy.iter() {
                // Skip root and basic type nodes
                if id.ends_with("_root") || 
                   id.ends_with("_identity") || 
                   id.ends_with("_semantic") || 
                   id.ends_with("_episodic") || 
                   id.ends_with("_procedural") || 
                   id.ends_with("_relational") {
                    continue;
                }
                
                // Generate a global concept ID
                let global_id = format!("concept_{}", sanitize_for_id(&node.name));
                
                if let Some(mut global_node) = global_hierarchy.get_mut(&global_id) {
                    // Merge with existing node
                    for segment_id in &node.segment_ids {
                        if !global_node.segment_ids.contains(segment_id) {
                            global_node.segment_ids.push(segment_id.clone());
                        }
                    }
                    
                    // Update metadata
                    global_node.metadata.insert(format!("agent_{}", agent_name), "true".to_string());
                } else {
                    // Create a new global node
                    let global_node = HierarchicalNode {
                        id: global_id.clone(),
                        name: node.name.clone(),
                        description: format!("Concept from agent {}: {}", agent_name, node.description),
                        parent_id: Some("semantic".to_string()), // Default to semantic parent
                        child_ids: Vec::new(),
                        segment_ids: node.segment_ids.clone(),
                        depth: 2, // Level 2 under semantic
                        metadata: HashMap::from([
                            ("source_agent".to_string(), agent_name.to_string()),
                            ("source_node".to_string(), id.clone()),
                            ("type".to_string(), "concept".to_string()),
                        ]),
                    };
                    
                    // Add to global hierarchy
                    global_hierarchy.insert(global_id.clone(), global_node);
                    
                    // Update parent's children list
                    if let Some(mut parent) = global_hierarchy.get_mut("semantic") {
                        if !parent.child_ids.contains(&global_id) {
                            parent.child_ids.push(global_id);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Create relational indexes for memory segments
    pub fn create_relational_indexes(&self) -> Result<usize, Box<dyn Error>> {
        // Process each agent
        let mut total_relations = 0;
        
        // Get list of agents from the existing agent indexes
        let agent_names: Vec<String> = self.indexes.iter()
            .filter_map(|entry| {
                let metadata = entry.value();
                if metadata.index_type == IndexType::Agent {
                    Some(metadata.name.clone())
                } else {
                    None
                }
            })
            .collect();
        
        // Process each agent
        for agent_name in agent_names {
            // Get all segments for this agent
            let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
            let segments = memory_loader.load_agent_memory(&agent_name, None)?;
            drop(memory_loader);
            
            // Build relations
            let agent_relations = self.build_segment_relations(&segments)?;
            total_relations += agent_relations;
            
            // Store in hierarchy
            if let Some(hierarchy_arc) = self.agent_hierarchies.get(&agent_name) {
                let mut hierarchy = hierarchy_arc.write().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
                
                // Update relational type node with these relations
                let relational_node_id = format!("{}_relational", agent_name);
                if let Some(mut node) = hierarchy.get_mut(&relational_node_id) {
                    // Store relation counts in metadata
                    node.metadata.insert("relation_count".to_string(), agent_relations.to_string());
                }
            }
        }
        
        Ok(total_relations)
    }

    /// Build relations between memory segments
    fn build_segment_relations(&self, segments: &[MemorySegment]) -> Result<usize, Box<dyn Error>> {
        let mut relation_count = 0;
        
        // First, extract explicit relations from segment.related_segments
        for segment in segments {
            // Create a relation set for this segment if it doesn't exist
            let relations = self.relational_indexes.entry(segment.id.clone())
                .or_insert_with(HashMap::new);
                
            // Add explicit relations
            for related_id in &segment.related_segments {
                relations.insert(related_id.clone(), HashSet::from([
                    "explicit".to_string(),
                ]));
                relation_count += 1;
            }
        }
        
        // Next, look for implicit relations based on content similarity
        // This is a more complex implementation that would analyze content
        // For a real implementation, this could use embeddings or more advanced techniques
        
        // For demonstration purposes, we'll use a simple keyword-based approach
        let mut keyword_index: HashMap<String, Vec<String>> = HashMap::new();
        
        // Build keyword index
        for segment in segments {
            // Extract keywords from content
            let keywords = extract_keywords(&segment.content);
            
            // Add to index
            for keyword in keywords {
                keyword_index.entry(keyword)
                    .or_insert_with(Vec::new)
                    .push(segment.id.clone());
            }
        }
        
        // Find relations based on shared keywords
        for segment in segments {
            let keywords = extract_keywords(&segment.content);
            let segment_id = &segment.id;
            
            let mut related_segments: HashMap<String, usize> = HashMap::new();
            
            // Look up each keyword and find segments that share it
            for keyword in keywords {
                if let Some(segment_ids) = keyword_index.get(&keyword) {
                    for id in segment_ids {
                        if id != segment_id {
                            *related_segments.entry(id.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
            
            // Filter to segments with enough shared keywords (arbitrary threshold)
            let significant_relations: Vec<(String, usize)> = related_segments.into_iter()
                .filter(|(_, count)| *count >= 3) // At least 3 shared keywords
                .collect();
                
            // Add to relations
            if !significant_relations.is_empty() {
                let relations = self.relational_indexes.entry(segment_id.clone())
                    .or_insert_with(HashMap::new);
                    
                for (related_id, count) in significant_relations {
                    let relation_types = relations.entry(related_id)
                        .or_insert_with(HashSet::new);
                        
                    relation_types.insert("content_similarity".to_string());
                    relation_count += 1;
                }
            }
        }
        
        Ok(relation_count)
    }

    /// Create a temporal index for organizing memory by time
    pub fn create_temporal_index(&self) -> Result<String, Box<dyn Error>> {
        // Generate a unique index ID
        let index_id = format!("temporal_{}", current_timestamp());
        
        // Create index directory
        let index_dir = Path::new(&self.config.index_directory).join(&index_id);
        fs::create_dir_all(&index_dir)?;
        
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Define fields
        let id = schema_builder.add_text_field("id", TEXT | STORED);
        let agent = schema_builder.add_text_field("agent", TEXT | STORED);
        let timestamp = schema_builder.add_date_field("timestamp", INDEXED | STORED);
        let year = schema_builder.add_u64_field("year", INDEXED | STORED);
        let month = schema_builder.add_u64_field("month", INDEXED | STORED);
        let day = schema_builder.add_u64_field("day", INDEXED | STORED);
        let memory_type = schema_builder.add_text_field("memory_type", TEXT | STORED);
        let summary = schema_builder.add_text_field("summary", TEXT | STORED);
        
        let schema = schema_builder.build();
        
        // Create the index
        let index = Index::create_in_dir(&index_dir, schema)?;
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        // Create metadata
        let metadata = IndexMetadata {
            id: index_id.clone(),
            index_type: IndexType::Temporal,
            name: "Temporal Index".to_string(),
            created_timestamp: current_timestamp(),
            last_updated_timestamp: current_timestamp(),
            document_count: 0,
            custom_metadata: HashMap::from([
                ("schema_version".to_string(), "1".to_string()),
                ("type".to_string(), "temporal".to_string()),
            ]),
            path: index_dir,
        };
        
        // Save metadata
        self.save_index_metadata(&metadata)?;
        
        // Store the index
        self.search_indexes.insert(index_id.clone(), Arc::new(index));
        self.index_writers.insert(index_id.clone(), Arc::new(Mutex::new(writer)));
        self.indexes.insert(index_id.clone(), metadata);
        
        Ok(index_id)
    }

    /// Load available memory segments and index them by time
    pub fn build_temporal_index(&self) -> Result<usize, Box<dyn Error>> {
        // Create temporal index
        let index_id = self.create_temporal_index()?;
        
        // Get the index and writer
        let index_arc = self.search_indexes.get(&index_id).ok_or_else(|| format!("Index not found: {}", index_id))?;
        let index = &*index_arc;
        
        let writer_arc = self.index_writers.get(&index_id).ok_or_else(|| format!("Writer not found: {}", index_id))?;
        let mut writer = writer_arc.lock().map_err(|e| format!("Failed to lock writer: {}", e))?;
        
        // Get the schema
        let schema = index.schema();
        
        // Get field accessors
        let id_field = schema.get_field("id").unwrap();
        let agent_field = schema.get_field("agent").unwrap();
        let timestamp_field = schema.get_field("timestamp").unwrap();
        let year_field = schema.get_field("year").unwrap();
        let month_field = schema.get_field("month").unwrap();
        let day_field = schema.get_field("day").unwrap();
        let memory_type_field = schema.get_field("memory_type").unwrap();
        let summary_field = schema.get_field("summary").unwrap();
        
        // Get list of agents
        let agent_names: Vec<String> = self.indexes.iter()
            .filter_map(|entry| {
                let metadata = entry.value();
                if metadata.index_type == IndexType::Agent {
                    Some(metadata.name.clone())
                } else {
                    None
                }
            })
            .collect();
            
        let mut indexed_count = 0;
        
        // Process each agent
        for agent_name in agent_names {
            // Get segments with temporal information
            let memory_loader = self.memory_loader.lock().map_err(|e| format!("Failed to lock memory loader: {}", e))?;
            let segments = memory_loader.load_agent_memory(&agent_name, None)?;
            drop(memory_loader);
            
            // Filter for segments with temporal information
            let temporal_segments: Vec<&MemorySegment> = segments.iter()
                .filter(|s| s.metadata.contains_key("timestamp") || s.metadata.contains_key("created_at"))
                .collect();
                
            // Index each segment
            for segment in temporal_segments {
                // Extract timestamp
                let timestamp_str = segment.metadata.get("timestamp")
                    .or_else(|| segment.metadata.get("created_at"))
                    .or_else(|| segment.metadata.get("date"))
                    .unwrap_or(&String::new());
                    
                let timestamp = parse_timestamp(timestamp_str);
                
                // Extract date components
                let (year, month, day) = extract_date_components(timestamp);
                
                // Generate a summary
                let summary = generate_segment_summary(segment);
                
                // Create document
                let mut doc = Document::new();
                doc.add_text(id_field, &segment.id);
                doc.add_text(agent_field, &agent_name);
                doc.add_date(timestamp_field, tantivy::DateTime::from_timestamp_secs(timestamp as i64));
                doc.add_u64(year_field, year);
                doc.add_u64(month_field, month);
                doc.add_u64(day_field, day);
                doc.add_text(memory_type_field, format!("{:?}", segment.memory_type));
                doc.add_text(summary_field, &summary);
                
                // Add to index
                writer.add_document(doc)?;
                indexed_count += 1;
            }
        }
        
        // Commit changes
        writer.commit()?;
        
        // Update metadata
        if let Some(mut metadata) = self.indexes.get_mut(&index_id) {
            metadata.document_count = indexed_count;
            metadata.last_updated_timestamp = current_timestamp();
            self.save_index_metadata(&metadata)?;
        }
        
        Ok(indexed_count)
    }

    /// Get a view of the memory hierarchy
    pub fn get_hierarchy_view(&self, agent_name: Option<&str>, max_depth: Option<usize>) -> Result<HashMap<String, HierarchicalNode>, Box<dyn Error>> {
        let max_depth = max_depth.unwrap_or(10);
        
        if let Some(agent_name) = agent_name {
            // Get agent-specific hierarchy
            if let Some(hierarchy_arc) = self.agent_hierarchies.get(agent_name) {
                let hierarchy = hierarchy_arc.read().map_err(|e| format!("Failed to lock agent hierarchy: {}", e))?;
                
                // Filter by depth
                let filtered: HashMap<String, HierarchicalNode> = hierarchy.iter()
                    .filter(|(_, node)| node.depth <= max_depth)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                    
                Ok(filtered)
            } else {
                Err(format!("No hierarchy found for agent: {}", agent_name).into())
            }
        } else {
            // Get global hierarchy
            let global_hierarchy = self.concept_hierarchy.read().map_err(|e| format!("Failed to lock global hierarchy: {}", e))?;
            
            // Filter by depth
            let filtered: HashMap<String, HierarchicalNode> = global_hierarchy.iter()
                .filter(|(_, node)| node.depth <= max_depth)
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
                
            Ok(filtered)
        }
    }

    /// Build a BTree-based representation of the hierarchy for efficient traversal
    pub fn build_hierarchy_tree(&self, agent_name: Option<&str>) -> Result<BTreeMap<String, Vec<String>>, Box<dyn Error>> {
        // Get the appropriate hierarchy
        let hierarchy = self.get_hierarchy_view(agent_name, None)?;
        
        // Build a tree representation
        let mut tree = BTreeMap::new();
        
        // First pass: collect all nodes by level
        let mut nodes_by_level: HashMap<usize, Vec<String>> = HashMap::new();
        
        for (id, node) in &hierarchy {
            nodes_by_level.entry(node.depth)
                .or_insert_with(Vec::new)
                .push(id.clone());
        }
        
        // Second pass: build the tree level by level
        for level in 0..=nodes_by_level.keys().max().copied().unwrap_or(0) {
            if let Some(nodes) = nodes_by_level.get(&level) {
                for node_id in nodes {
                    if let Some(node) = hierarchy.get(node_id) {
                        // Add this node's children to the tree
                        let children = node.child_ids.clone();
                        if !children.is_empty() {
                            tree.insert(node_id.clone(), children);
                        } else {
                            tree.insert(node_id.clone(), Vec::new());
                        }
                    }
                }
            }
        }
        
        Ok(tree)
    }

    /// Export the hierarchy to a JSON file
    pub fn export_hierarchy(&self, agent_name: Option<&str>, output_path: &Path) -> Result<(), Box<dyn Error>> {
        let hierarchy = self.get_hierarchy_view(agent_name, None)?;
        
        // Open file for writing
        let file = File::create(output_path)?;
        let writer = BufWriter::new(file);
        
        // Write hierarchy as JSON
        serde_json::to_writer_pretty(writer, &hierarchy)?;
        
        Ok(())
    }

    /// Estimate the disk space used by the indexes
    pub fn estimate_index_size(&self) -> Result<HashMap<String, u64>, Box<dyn Error>> {
        let mut sizes = HashMap::new();
        
        for entry in self.indexes.iter() {
            let metadata = entry.value();
            
            // Use walkdir to calculate directory size
            let mut total_size = 0;
            for entry in walkdir::WalkDir::new(&metadata.path)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok()) {
                if entry.path().is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                }
            }
            
            sizes.insert(metadata.id.clone(), total_size);
        }
        
        Ok(sizes)
    }
}

/// Extract keywords from content for basic relation building
fn extract_keywords(content: &str) -> HashSet<String> {
    let mut keywords = HashSet::new();
    
    // Split into words
    let words: Vec<&str> = content
        .split(|c: char| !c.is_alphanumeric())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && s.len() >= 4) // Only consider words 4+ chars
        .collect();
        
    // Count word frequencies
    let mut word_counts: HashMap<&str, usize> = HashMap::new();
    for word in &words {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    
    // Filter for significant words (mentioned multiple times)
    let significant_words: Vec<&str> = word_counts.iter()
        .filter(|(_, &count)| count >= 2) // Mentioned at least twice
        .map(|(&word, _)| word)
        .collect();
        
    // Convert to lowercase for normalization
    for word in significant_words {
        keywords.insert(word.to_lowercase());
    }
    
    keywords
}

/// Parse a timestamp string into a unix timestamp
fn parse_timestamp(timestamp_str: &str) -> u64 {
    if timestamp_str.is_empty() {
        return current_timestamp();
    }
    
    // Try various formats
    if let Ok(timestamp) = timestamp_str.parse::<u64>() {
        // Direct unix timestamp
        return timestamp;
    }
    
    // Try ISO format
    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(timestamp_str) {
        return datetime.timestamp() as u64;
    }
    
    // Try common date formats
    let formats = [
        "%Y-%m-%d",
        "%Y/%m/%d",
        "%d-%m-%Y",
        "%d/%m/%Y",
        "%Y-%m-%d %H:%M:%S",
    ];
    
    for format in &formats {
        if let Ok(datetime) = chrono::NaiveDateTime::parse_from_str(timestamp_str, format) {
            return datetime.timestamp() as u64;
        }
        
        if let Ok(date) = chrono::NaiveDate::parse_from_str(timestamp_str, format) {
            return date.and_hms_opt(0, 0, 0).unwrap().timestamp() as u64;
        }
    }
    
    // Default to current time if parsing fails
    current_timestamp()
}

/// Extract year, month, day from a unix timestamp
fn extract_date_components(timestamp: u64) -> (u64, u64, u64) {
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    
    (
        datetime.year() as u64,
        datetime.month() as u64,
        datetime.day() as u64,
    )
}

/// Generate a summary of a memory segment (basic implementation)
fn generate_segment_summary(segment: &MemorySegment) -> String {
    // Extract the first heading if available
    let lines: Vec<&str> = segment.content.lines().collect();
    for line in &lines {
        if line.starts_with("# ") {
            return line[2..].trim().to_string();
        }
    }
    
    // If no heading, take the first non-empty line
    for line in &lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            // Limit to reasonable length
            if trimmed.len() <= 100 {
                return trimmed.to_string();
            } else {
                return format!("{}...", &trimmed[..97]);
            }
        }
    }
    
    // Default summary if content is empty
    format!("{:?} memory segment", segment.memory_type)
}

/// Sanitize a string for use as an ID
fn sanitize_for_id(input: &str) -> String {
    // Remove special characters and replace spaces with underscores
    let mut result = String::new();
    
    for c in input.chars() {
        if c.is_alphanumeric() {
            result.push(c.to_ascii_lowercase());
        } else if c.is_whitespace() {
            result.push('_');
        }
    }
    
    // Ensure it's not empty
    if result.is_empty() {
        result = "unknown".to_string();
    }
    
    result
}

/// Get the current timestamp in seconds
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    // Helper to create test segments
    fn create_test_segments() -> Vec<MemorySegment> {
        vec![
            MemorySegment {
                id: "test-segment-1".to_string(),
                memory_type: MemoryType::Semantic,
                tier: 1,
                content: "# Test Concept\n\nThis is a test semantic memory about databases.\nDatabases store data in an organized way.".to_string(),
                metadata: HashMap::from([
                    ("agent".to_string(), "TestAgent".to_string()),
                    ("tags".to_string(), "database,storage,test".to_string()),
                ]),
                activation_score: 0.5,
                last_accessed_timestamp: Some(current_timestamp()),
                related_segments: Vec::new(),
            },
            MemorySegment {
                id: "test-segment-2".to_string(),
                memory_type: MemoryType::Procedural,
                tier: 1,
                content: "# Database Query Process\n\n1. Connect to database\n2. Prepare SQL query\n3. Execute query\n4. Process results".to_string(),
                metadata: HashMap::from([
                    ("agent".to_string(), "TestAgent".to_string()),
                    ("tags".to_string(), "database,query,process".to_string()),
                    ("timestamp".to_string(), "2023-01-15".to_string()),
                ]),
                activation_score: 0.7,
                last_accessed_timestamp: Some(current_timestamp()),
                related_segments: vec!["test-segment-1".to_string()],
            },
        ]
    }
    
    #[test]
    fn test_extract_keywords() {
        let content = "Databases are organized collections of data. SQL databases use structured query language for accessing data.";
        let keywords = extract_keywords(content);
        
        assert!(keywords.contains("databases"));
        assert!(keywords.contains("organized"));
        assert!(keywords.contains("collections"));
        assert!(keywords.contains("structured"));
        assert!(keywords.contains("language"));
        assert!(keywords.contains("accessing"));
    }
    
    #[test]
    fn test_sanitize_for_id() {
        assert_eq!(sanitize_for_id("Test Concept"), "test_concept");
        assert_eq!(sanitize_for_id("Database-Query Process"), "databasequery_process");
        assert_eq!(sanitize_for_id("!@#$%^"), "unknown");
        assert_eq!(sanitize_for_id("SQL JOIN Operations"), "sql_join_operations");
    }
    
    #[test]
    fn test_parse_timestamp() {
        // Unix timestamp
        assert_eq!(parse_timestamp("1623456789"), 1623456789);
        
        // ISO format
        let iso_time = chrono::DateTime::parse_from_rfc3339("2023-05-15T14:30:00Z")
            .unwrap()
            .timestamp() as u64;
        assert_eq!(parse_timestamp("2023-05-15T14:30:00Z"), iso_time);
        
        // Date format
        let date_time = chrono::NaiveDate::from_ymd_opt(2023, 5, 15)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp() as u64;
        assert_eq!(parse_timestamp("2023-05-15"), date_time);
    }
    
    #[test]
    fn test_generate_segment_summary() {
        let segment = MemorySegment {
            id: "test-id".to_string(),
            memory_type: MemoryType::Semantic,
            tier: 1,
            content: "# Database Concepts\n\nThis is about databases.".to_string(),
            metadata: HashMap::new(),
            activation_score: 0.5,
            last_accessed_timestamp: None,
            related_segments: Vec::new(),
        };
        
        assert_eq!(generate_segment_summary(&segment), "Database Concepts");
        
        // Test with no heading
        let segment2 = MemorySegment {
            id: "test-id-2".to_string(),
            memory_type: MemoryType::Semantic,
            tier: 1,
            content: "This is a test segment without a heading.".to_string(),
            metadata: HashMap::new(),
            activation_score: 0.5,
            last_accessed_timestamp: None,
            related_segments: Vec::new(),
        };
        
        assert_eq!(generate_segment_summary(&segment2), "This is a test segment without a heading.");
    }
}