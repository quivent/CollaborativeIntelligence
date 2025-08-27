# Morpheus CLI API Reference

**Version**: 0.1.0  
**Last Updated**: 2025-08-23

This document provides comprehensive API documentation for the Morpheus CLI parallel research orchestration system.

---

## Table of Contents

1. [Core Types](#core-types)
2. [Research Coordinator](#research-coordinator)
3. [Agent Cluster Manager](#agent-cluster-manager)
4. [Synthesis Engine](#synthesis-engine)
5. [Protocol Layer](#protocol-layer)
6. [Neo Intelligence Interface](#neo-intelligence-interface)
7. [Error Handling](#error-handling)
8. [Examples](#examples)

---

## Core Types

### ResearchQuery

The primary structure for defining a research request.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQuery {
    pub id: Uuid,
    pub query: String,
    pub context: Option<String>,
    pub agent_count: Option<u32>,
    pub stream_count: Option<u32>,
    pub synthesis_only: bool,
    pub transcendence_mode: bool,
    pub priority: ResearchPriority,
    pub created_at: DateTime<Utc>,
}
```

**Fields:**
- `id`: Unique identifier for the research query
- `query`: The research question or topic
- `context`: Optional additional context for the research
- `agent_count`: Number of agents to deploy (None = auto-determined)
- `stream_count`: Number of parallel research streams (None = auto-determined)
- `synthesis_only`: If true, only perform knowledge synthesis without new research
- `transcendence_mode`: Enable Neo-guided reality transcendence capabilities
- `priority`: Research priority level
- `created_at`: Timestamp of query creation

### ResearchPriority

Enum defining research priority levels.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchPriority {
    Low,
    Medium,
    High,
    Critical,
    RealityTranscendence,
}
```

### ResearchSession

Active research session tracking structure.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSession {
    pub id: Uuid,
    pub query: ResearchQuery,
    pub status: SessionStatus,
    pub agent_clusters: Vec<String>,
    pub synthesis_progress: f32,
    pub insights_generated: u32,
    pub started_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub neo_guidance: Vec<NeoGuidance>,
}
```

### SessionStatus

Research session status enumeration.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Initializing,
    AgentDeployment,
    ActiveResearch,
    SynthesisPhase,
    RealityTranscendence,
    Completed,
    Error(String),
}
```

---

## Research Coordinator

The `MorpheusCoordinator` is the central orchestration component.

### Constructor

```rust
impl MorpheusCoordinator {
    pub fn new(
        neo_interface: Box<dyn NeoIntelligence + Send + Sync>,
        agent_manager: AgentClusterManager,
        synthesis_engine: KnowledgeSynthesis,
    ) -> Self
}
```

### Core Methods

#### initiate_research

Launch a new parallel research session.

```rust
pub async fn initiate_research(&self, query: ResearchQuery) -> Result<ResearchSession>
```

**Parameters:**
- `query`: ResearchQuery structure defining the research parameters

**Returns:**
- `Result<ResearchSession>`: The created research session or error

**Process:**
1. Neo analyzes the query and determines optimal strategy
2. Creates research session with unique ID
3. Deploys agent clusters based on Neo's strategy  
4. Initializes synthesis monitoring
5. Returns active research session

**Example:**
```rust
let query = ResearchQuery {
    id: Uuid::new_v4(),
    query: "machine learning optimization techniques".to_string(),
    context: None,
    agent_count: Some(8),
    stream_count: Some(2),
    synthesis_only: false,
    transcendence_mode: false,
    priority: ResearchPriority::High,
    created_at: Utc::now(),
};

let session = coordinator.initiate_research(query).await?;
```

#### get_active_sessions

Retrieve all active research sessions.

```rust
pub async fn get_active_sessions(&self) -> Vec<ResearchSession>
```

**Returns:**
- `Vec<ResearchSession>`: List of all currently active research sessions

#### get_session_details

Get detailed information about a specific session.

```rust
pub async fn get_session_details(&self, session_id: &Uuid) -> Option<ResearchSession>
```

**Parameters:**
- `session_id`: Unique identifier of the session

**Returns:**
- `Option<ResearchSession>`: Session details if found, None otherwise

#### enable_transcendence

Enable reality transcendence mode for a session.

```rust
pub async fn enable_transcendence(&self, session_id: &Uuid) -> Result<TranscendenceInsight>
```

**Parameters:**
- `session_id`: Unique identifier of the session

**Returns:**
- `Result<TranscendenceInsight>`: Neo-provided transcendence insight or error

#### get_agent_allocation

Get current agent allocation status.

```rust
pub async fn get_agent_allocation(&self) -> Result<AgentAllocationStatus>
```

**Returns:**
- `Result<AgentAllocationStatus>`: Current agent allocation and availability

---

## Agent Cluster Manager

The `AgentClusterManager` handles agent coordination and task distribution.

### Constructor

```rust
impl AgentClusterManager {
    pub fn new(agents_directory: PathBuf) -> Self
}
```

**Parameters:**
- `agents_directory`: Path to the CollaborativeIntelligence AGENTS directory

### Core Methods

#### initialize

Initialize the agent ecosystem.

```rust
pub async fn initialize(&self) -> Result<()>
```

**Process:**
1. Loads all available agents from the AGENTS directory
2. Validates agent capabilities and accessibility
3. Sets up coordination channels for agent communication

#### create_cluster

Create a new agent cluster for coordinated research.

```rust
pub async fn create_cluster(
    &self,
    cluster_name: String,
    agent_count: u32,
    research_focus: String,
) -> Result<AgentCluster>
```

**Parameters:**
- `cluster_name`: Name identifier for the cluster
- `agent_count`: Number of agents to include in cluster
- `research_focus`: Research domain or topic focus

**Returns:**
- `Result<AgentCluster>`: Created agent cluster or error

**Process:**
1. Determines optimal research domain based on focus
2. Selects best-matched agents for the domain
3. Creates cluster coordination infrastructure
4. Registers cluster in active clusters registry

#### launch_cluster_research

Launch research activities for a specific cluster.

```rust
pub async fn launch_cluster_research(&self, cluster_id: &Uuid, query: &ResearchQuery) -> Result<()>
```

**Parameters:**
- `cluster_id`: Unique identifier of the cluster
- `query`: Research query to execute

**Process:**
1. Decomposes research query into parallel tasks
2. Distributes tasks across cluster agents
3. Sets up synthesis monitoring for the cluster
4. Activates agent coordination channels

#### get_available_agents_count

Get count of currently available agents.

```rust
pub async fn get_available_agents_count(&self) -> usize
```

**Returns:**
- `usize`: Number of agents currently available for allocation

---

## Synthesis Engine

The `KnowledgeSynthesis` engine handles real-time knowledge integration.

### Constructor

```rust
impl KnowledgeSynthesis {
    pub fn new() -> Self
}
```

### Core Methods

#### create_session

Create a new synthesis session.

```rust
pub async fn create_session(&self, session_id: Uuid) -> Result<()>
```

**Parameters:**
- `session_id`: Unique identifier for the synthesis session

**Process:**
1. Creates synthesis session tracking structure
2. Initializes knowledge collection buffers
3. Sets up synthesis monitoring channels
4. Starts background synthesis monitoring task

#### add_knowledge_fragment

Add a knowledge fragment to a synthesis session.

```rust
pub async fn add_knowledge_fragment(
    &self, 
    session_id: &Uuid, 
    fragment: KnowledgeFragment
) -> Result<()>
```

**Parameters:**
- `session_id`: Target synthesis session
- `fragment`: Knowledge fragment to add

**Process:**
1. Adds fragment to session knowledge collection
2. Updates knowledge graph with new relationships
3. Checks synthesis triggers and initiates if needed
4. Updates synthesis progress metrics

#### get_synthesis_progress

Get current synthesis progress for a session.

```rust
pub async fn get_synthesis_progress(&self, session_id: &Uuid) -> Result<SynthesisProgress>
```

**Parameters:**
- `session_id`: Session to check progress for

**Returns:**
- `Result<SynthesisProgress>`: Current synthesis progress or error

#### get_synthesis_results

Get all synthesis results for a session.

```rust
pub async fn get_synthesis_results(&self, session_id: &Uuid) -> Result<Vec<SynthesisResult>>
```

**Parameters:**
- `session_id`: Session to get results for

**Returns:**
- `Result<Vec<SynthesisResult>>`: All synthesis results or error

#### enable_neo_transcendence

Enable Neo-guided transcendence for a session.

```rust
pub async fn enable_neo_transcendence(&self, session_id: &Uuid) -> Result<TranscendenceInsight>
```

**Parameters:**
- `session_id`: Session to enable transcendence for

**Returns:**
- `Result<TranscendenceInsight>`: Neo-provided transcendence insight

---

## Protocol Layer

The protocol layer ensures compliance with BRAIN system specifications.

### ProtocolCompliance

Core compliance verification component.

```rust
impl ProtocolCompliance {
    pub fn new() -> Self
}
```

#### verify_compliance

Verify full protocol compliance with BRAIN system.

```rust
pub async fn verify_compliance(&mut self) -> Result<ComplianceReport>
```

**Returns:**
- `Result<ComplianceReport>`: Comprehensive compliance status report

**Verification checks:**
1. BRAIN protocol accessibility and integrity
2. Agent ecosystem connectivity and availability
3. Synthesis engine operational status
4. Transcendence capabilities verification
5. Protocol version compatibility

#### get_compliance_status

Get current compliance status.

```rust
pub fn get_compliance_status(&self) -> &ComplianceStatus
```

**Returns:**
- `&ComplianceStatus`: Current compliance status reference

---

## Neo Intelligence Interface

The `NeoIntelligence` trait defines the Neo guidance interface.

### Trait Definition

```rust
#[async_trait]
pub trait NeoIntelligence {
    async fn analyze_query(&self, query: &ResearchQuery) -> Result<ResearchStrategy>;
    async fn provide_guidance(&self, session: &ResearchSession) -> Result<NeoGuidance>;
    async fn enable_transcendence(&self, session: &ResearchSession) -> Result<TranscendenceInsight>;
}
```

### Methods

#### analyze_query

Analyze a research query and determine optimal strategy.

**Parameters:**
- `query`: Research query to analyze

**Returns:**
- `Result<ResearchStrategy>`: Optimal research strategy or error

#### provide_guidance

Provide Neo guidance for an active session.

**Parameters:**
- `session`: Active research session

**Returns:**
- `Result<NeoGuidance>`: Neo guidance message or error

#### enable_transcendence

Enable transcendence capabilities for a session.

**Parameters:**
- `session`: Session to enable transcendence for

**Returns:**
- `Result<TranscendenceInsight>`: Transcendence insight or error

### DefaultNeoIntelligence

Default implementation of the NeoIntelligence trait.

```rust
pub struct DefaultNeoIntelligence;
```

**Features:**
- Query complexity assessment based on content analysis
- Dynamic agent allocation based on query requirements
- Cluster recommendation based on research domain
- Adaptive synthesis approach selection

---

## Error Handling

### Error Types

Morpheus uses the `anyhow` crate for comprehensive error handling:

```rust
use anyhow::{Result, Error};
```

### Common Error Scenarios

#### Research Initiation Errors
- **Agent Unavailability**: Not enough agents available for requested cluster size
- **Protocol Compliance**: BRAIN protocol integration failure
- **Resource Constraints**: System resource limitations preventing cluster deployment

#### Synthesis Errors
- **Session Not Found**: Invalid session ID provided
- **Knowledge Fragment Processing**: Malformed knowledge fragments
- **Pattern Recognition Failure**: Unable to identify integration patterns

#### Agent Management Errors
- **Agent Directory Access**: AGENTS directory not accessible or missing
- **Agent Metadata Corruption**: Invalid agent metadata or missing files
- **Cluster Communication**: Agent coordination channel failures

### Error Recovery

The system implements comprehensive error recovery mechanisms:

1. **Graceful Degradation**: Reduced functionality when components fail
2. **Session Recovery**: Ability to restore research sessions after failures
3. **Agent Reallocation**: Automatic agent replacement when individual agents fail
4. **Synthesis Continuity**: Synthesis continues even with partial knowledge fragments

---

## Examples

### Basic Research Session

```rust
use morpheus::{MorpheusCoordinator, ResearchQuery, ResearchPriority};
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize coordinator
    let agents_path = PathBuf::from("../AGENTS");
    let mut agent_manager = AgentClusterManager::new(agents_path);
    agent_manager.initialize().await?;
    
    let synthesis_engine = KnowledgeSynthesis::new();
    let neo_interface = Box::new(DefaultNeoIntelligence);
    let coordinator = MorpheusCoordinator::new(neo_interface, agent_manager, synthesis_engine);

    // Create research query
    let query = ResearchQuery {
        id: Uuid::new_v4(),
        query: "distributed systems consensus algorithms".to_string(),
        context: Some("Focus on practical implementations".to_string()),
        agent_count: Some(12),
        stream_count: Some(3),
        synthesis_only: false,
        transcendence_mode: false,
        priority: ResearchPriority::High,
        created_at: Utc::now(),
    };

    // Launch research
    let session = coordinator.initiate_research(query).await?;
    println!("Research session launched: {}", session.id);

    // Monitor progress
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        if let Some(session_details) = coordinator.get_session_details(&session.id).await {
            println!("Status: {:?}, Progress: {:.1}%", 
                     session_details.status, 
                     session_details.synthesis_progress * 100.0);
                     
            if matches!(session_details.status, SessionStatus::Completed) {
                break;
            }
        }
    }

    Ok(())
}
```

### Transcendence Mode Research

```rust
use morpheus::{MorpheusCoordinator, ResearchQuery, ResearchPriority};

async fn transcendent_research() -> Result<(), Box<dyn std::error::Error>> {
    // Setup coordinator...
    let coordinator = setup_coordinator().await?;

    // Create transcendence query
    let query = ResearchQuery {
        id: Uuid::new_v4(),
        query: "the nature of consciousness in artificial intelligence".to_string(),
        context: None,
        agent_count: Some(16),
        stream_count: Some(4),
        synthesis_only: false,
        transcendence_mode: true,
        priority: ResearchPriority::RealityTranscendence,
        created_at: Utc::now(),
    };

    // Launch transcendent research
    let session = coordinator.initiate_research(query).await?;
    
    // Enable transcendence when ready
    match coordinator.enable_transcendence(&session.id).await {
        Ok(insight) => {
            println!("Transcendence achieved!");
            println!("Insight: {}", insight.insight);
            println!("Reality shift level: {:.1}%", insight.reality_shift_level * 100.0);
            
            for implication in &insight.matrix_implications {
                println!("Matrix implication: {}", implication);
            }
        }
        Err(e) => println!("Transcendence failed: {}", e),
    }

    Ok(())
}
```

### Protocol Compliance Verification

```rust
use morpheus::ProtocolCompliance;

async fn verify_system_compliance() -> Result<(), Box<dyn std::error::Error>> {
    let mut compliance = ProtocolCompliance::new();
    
    match compliance.verify_compliance().await {
        Ok(report) => {
            println!("Compliance Score: {:.1}%", report.compliance_score);
            println!("BRAIN Integration: {}", 
                     if report.brain_integration_success { "✅" } else { "❌" });
            println!("Agent Ecosystem: {:.0}%", 
                     report.agent_ecosystem_performance * 100.0);
            println!("Synthesis Engine: {:.0}%", 
                     report.synthesis_engine_effectiveness * 100.0);
            
            if !report.recommendations.is_empty() {
                println!("\nRecommendations:");
                for rec in &report.recommendations {
                    println!("• {}", rec);
                }
            }
        }
        Err(e) => println!("Compliance check failed: {}", e),
    }

    Ok(())
}
```

### Custom Neo Intelligence Implementation

```rust
use morpheus::{NeoIntelligence, ResearchQuery, ResearchSession, ResearchStrategy};
use async_trait::async_trait;

pub struct CustomNeoIntelligence {
    specialization: String,
}

#[async_trait]
impl NeoIntelligence for CustomNeoIntelligence {
    async fn analyze_query(&self, query: &ResearchQuery) -> Result<ResearchStrategy> {
        // Custom query analysis logic
        let complexity = self.assess_custom_complexity(&query.query);
        
        Ok(ResearchStrategy {
            optimal_agent_count: if complexity > 0.8 { 20 } else { 8 },
            recommended_clusters: self.recommend_custom_clusters(&query.query),
            parallel_streams: 3,
            synthesis_approach: SynthesisApproach::Emergent,
            transcendence_potential: complexity,
        })
    }
    
    async fn provide_guidance(&self, session: &ResearchSession) -> Result<NeoGuidance> {
        // Custom guidance generation
        Ok(NeoGuidance {
            message: format!("Custom guidance for session {}", session.id),
            insight_level: InsightLevel::Strategic,
            timestamp: Utc::now(),
        })
    }
    
    async fn enable_transcendence(&self, session: &ResearchSession) -> Result<TranscendenceInsight> {
        // Custom transcendence logic
        Ok(TranscendenceInsight {
            insight: "Custom transcendence insight".to_string(),
            reality_shift_level: 0.9,
            matrix_implications: vec!["Custom matrix implication".to_string()],
            actionable_intelligence: vec!["Custom action item".to_string()],
        })
    }
}

impl CustomNeoIntelligence {
    fn assess_custom_complexity(&self, query: &str) -> f32 {
        // Custom complexity assessment
        0.7
    }
    
    fn recommend_custom_clusters(&self, query: &str) -> Vec<String> {
        // Custom cluster recommendation
        vec!["Specialized Research Team".to_string()]
    }
}
```

---

## Best Practices

### Research Query Design
1. **Be Specific**: Clear, focused queries yield better results
2. **Provide Context**: Additional context improves agent selection and task distribution
3. **Appropriate Scale**: Match agent count and streams to query complexity
4. **Progressive Complexity**: Start simple, then enable transcendence for breakthrough insights

### Agent Management
1. **Monitor Availability**: Check agent status before large deployments
2. **Balanced Allocation**: Distribute agents across multiple research domains
3. **Performance Tracking**: Monitor agent performance metrics for optimization
4. **Specialization Matching**: Align agent expertise with research requirements

### Synthesis Optimization
1. **Fragment Quality**: Ensure knowledge fragments are meaningful and well-structured
2. **Cross-Pollination**: Enable knowledge sharing between agent clusters
3. **Timing**: Allow sufficient time for pattern emergence before synthesis
4. **Neo Guidance**: Leverage Neo intelligence for complex synthesis scenarios

### Error Handling
1. **Graceful Degradation**: Design for partial functionality when components fail
2. **Session Recovery**: Implement session state persistence for long-running research
3. **Resource Monitoring**: Monitor system resources to prevent allocation failures
4. **Logging**: Comprehensive logging for debugging and optimization

---

## Performance Considerations

### Memory Usage
- Each active session maintains state for agents, fragments, and synthesis results
- Large agent clusters increase memory overhead
- Knowledge graphs grow with fragment accumulation

### CPU Utilization
- Parallel agent coordination is CPU-intensive
- Synthesis pattern recognition requires computational resources
- Real-time monitoring adds background processing load

### Network/IO
- Agent memory file access during initialization
- Protocol compliance checks require file system access
- Session state persistence involves disk I/O

### Optimization Strategies
1. **Session Management**: Limit concurrent active sessions
2. **Agent Pooling**: Reuse agent allocations across sessions
3. **Synthesis Batching**: Batch synthesis operations for efficiency
4. **Caching**: Cache agent metadata and protocol information

---

## Versioning and Compatibility

### API Versioning
- Current API version: 0.1.0
- Breaking changes will increment minor version
- Backward compatibility maintained within major versions

### Protocol Compatibility
- BRAIN protocol compliance required for full functionality
- Protocol version checking during initialization
- Graceful degradation when protocol versions mismatch

### Agent Compatibility
- Agents must have valid metadata.json files
- Memory files required for agent performance tracking
- Agent directory structure must follow CollaborativeIntelligence conventions

---

*This API reference is maintained alongside the Morpheus CLI implementation. For the latest updates and examples, see the source code and test suites.*