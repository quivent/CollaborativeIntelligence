# Temporal Event Stream Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into an immutable, append-only event stream where all agent interactions, knowledge updates, and system changes are captured as timestamped events for temporal analysis and replay.

## Event Stream Architecture

### Event Schema
```python
EVENT_SCHEMA = {
    'event_id': 'UUID',
    'timestamp': 'ISO8601_datetime',
    'event_type': 'enum',
    'agent_id': 'string',
    'session_id': 'UUID_optional',
    'payload': 'object',
    'correlation_id': 'UUID_optional',
    'causation_id': 'UUID_optional',
    'metadata': 'object'
}

EVENT_TYPES = {
    'AGENT_CREATED': 'New agent initialization',
    'AGENT_MODIFIED': 'Agent configuration or identity change',
    'CAPABILITY_ADDED': 'New capability registered',
    'CAPABILITY_REMOVED': 'Capability deprecated',
    'KNOWLEDGE_ACQUIRED': 'New knowledge learned',
    'KNOWLEDGE_UPDATED': 'Existing knowledge modified',
    'TASK_STARTED': 'Task execution began',
    'TASK_COMPLETED': 'Task execution finished',
    'COLLABORATION_INITIATED': 'Multi-agent collaboration started',
    'COLLABORATION_ENDED': 'Multi-agent collaboration finished',
    'SESSION_STARTED': 'Working session began',
    'SESSION_ENDED': 'Working session concluded',
    'ERROR_OCCURRED': 'Error or exception logged',
    'PERFORMANCE_METRIC': 'Performance measurement recorded'
}
```

## Data Extraction Process

### Step 1: Historical Event Reconstruction
```python
def reconstruct_historical_events():
    """
    Reconstruct historical events from existing agent artifacts
    """
    events = []
    
    # Process each agent directory
    agent_dirs = [d for d in Path('../AGENTS').iterdir() 
                  if d.is_dir() and d.name[0].isupper()]
    
    for agent_dir in agent_dirs:
        agent_name = agent_dir.name
        
        # Extract creation event from directory timestamp
        creation_event = extract_agent_creation_event(agent_dir, agent_name)
        if creation_event:
            events.append(creation_event)
        
        # Extract capability evolution from README/memory files
        capability_events = extract_capability_events(agent_dir, agent_name)
        events.extend(capability_events)
        
        # Extract knowledge acquisition from working files
        knowledge_events = extract_knowledge_events(agent_dir, agent_name)
        events.extend(knowledge_events)
        
        # Extract task execution from working files
        task_events = extract_task_events(agent_dir, agent_name)
        events.extend(task_events)
        
        # Extract modification events from file changes
        modification_events = extract_modification_events(agent_dir, agent_name)
        events.extend(modification_events)
    
    # Extract collaboration events from session files
    collaboration_events = extract_collaboration_events()
    events.extend(collaboration_events)
    
    # Sort events by timestamp
    events.sort(key=lambda e: e['timestamp'])
    
    return events

def extract_agent_creation_event(agent_dir, agent_name):
    """
    Extract agent creation event from directory artifacts
    """
    # Find earliest file timestamp as creation time
    all_files = list(agent_dir.glob('*'))
    if not all_files:
        return None
    
    # Get earliest file creation time
    earliest_time = min(file.stat().st_ctime for file in all_files)
    
    # Read initial configuration
    readme_file = agent_dir / 'README.md'
    initial_config = {}
    
    if readme_file.exists():
        with open(readme_file, 'r', encoding='utf-8') as f:
            content = f.read()
            initial_config = extract_agent_config_from_content(content)
    
    event = {
        'event_id': generate_event_id(agent_name, 'creation', earliest_time),
        'timestamp': datetime.fromtimestamp(earliest_time).isoformat(),
        'event_type': 'AGENT_CREATED',
        'agent_id': agent_name,
        'session_id': None,
        'payload': {
            'agent_name': agent_name,
            'initial_config': initial_config,
            'directory_path': str(agent_dir)
        },
        'correlation_id': None,
        'causation_id': None,
        'metadata': {
            'source': 'directory_analysis',
            'confidence': 0.8
        }
    }
    
    return event

def extract_capability_events(agent_dir, agent_name):
    """
    Extract capability addition/modification events
    """
    events = []
    
    # Analyze README evolution for capability changes
    readme_file = agent_dir / 'README.md'
    if readme_file.exists():
        readme_timestamp = readme_file.stat().st_mtime
        
        with open(readme_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        capabilities = extract_capabilities_from_content(content)
        
        for capability in capabilities:
            event = {
                'event_id': generate_event_id(agent_name, 'capability', capability),
                'timestamp': datetime.fromtimestamp(readme_timestamp).isoformat(),
                'event_type': 'CAPABILITY_ADDED',
                'agent_id': agent_name,
                'payload': {
                    'capability_name': capability['name'],
                    'capability_description': capability.get('description', ''),
                    'capability_category': capability.get('category', 'general')
                },
                'metadata': {
                    'source': 'readme_analysis',
                    'confidence': 0.7
                }
            }
            events.append(event)
    
    # Analyze working files for capability evolution
    working_files = sorted(agent_dir.glob('working_*.md'))
    
    previous_capabilities = set()
    
    for working_file in working_files:
        file_timestamp = working_file.stat().st_mtime
        
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        current_capabilities = set(extract_capability_names(content))
        
        # Find new capabilities
        new_capabilities = current_capabilities - previous_capabilities
        
        for capability in new_capabilities:
            event = {
                'event_id': generate_event_id(agent_name, 'capability_evolution', capability),
                'timestamp': datetime.fromtimestamp(file_timestamp).isoformat(),
                'event_type': 'CAPABILITY_ADDED',
                'agent_id': agent_name,
                'payload': {
                    'capability_name': capability,
                    'discovered_in_session': extract_session_id_from_filename(working_file.name),
                    'context': extract_capability_context(content, capability)
                },
                'metadata': {
                    'source': 'working_file_analysis',
                    'confidence': 0.6,
                    'file': str(working_file)
                }
            }
            events.append(event)
        
        previous_capabilities = current_capabilities
    
    return events

def extract_knowledge_events(agent_dir, agent_name):
    """
    Extract knowledge acquisition and update events
    """
    events = []
    
    # Process working files for knowledge acquisition
    working_files = sorted(agent_dir.glob('working_*.md'))
    
    for working_file in working_files:
        file_timestamp = working_file.stat().st_mtime
        session_id = extract_session_id_from_filename(working_file.name)
        
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract knowledge items from the content
        knowledge_items = extract_knowledge_items_from_content(content)
        
        for knowledge_item in knowledge_items:
            event = {
                'event_id': generate_event_id(agent_name, 'knowledge', knowledge_item['id']),
                'timestamp': datetime.fromtimestamp(file_timestamp).isoformat(),
                'event_type': 'KNOWLEDGE_ACQUIRED',
                'agent_id': agent_name,
                'session_id': session_id,
                'payload': {
                    'knowledge_topic': knowledge_item['topic'],
                    'knowledge_content': knowledge_item['content'],
                    'knowledge_category': knowledge_item.get('category', 'general'),
                    'confidence_level': knowledge_item.get('confidence', 0.8),
                    'source_context': knowledge_item.get('context', '')
                },
                'metadata': {
                    'source': 'working_file_analysis',
                    'file': str(working_file),
                    'extraction_method': 'content_analysis'
                }
            }
            events.append(event)
    
    # Process memory files for accumulated knowledge
    memory_files = [f for f in agent_dir.glob('*') if 'memory' in f.name.lower()]
    
    for memory_file in memory_files:
        file_timestamp = memory_file.stat().st_mtime
        
        with open(memory_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract consolidated knowledge from memory
        memory_knowledge = extract_consolidated_knowledge(content)
        
        for knowledge in memory_knowledge:
            event = {
                'event_id': generate_event_id(agent_name, 'memory_knowledge', knowledge['id']),
                'timestamp': datetime.fromtimestamp(file_timestamp).isoformat(),
                'event_type': 'KNOWLEDGE_UPDATED',
                'agent_id': agent_name,
                'payload': {
                    'knowledge_topic': knowledge['topic'],
                    'knowledge_summary': knowledge['summary'],
                    'consolidation_level': knowledge.get('consolidation', 'high'),
                    'related_experiences': knowledge.get('experiences', [])
                },
                'metadata': {
                    'source': 'memory_analysis',
                    'file': str(memory_file),
                    'knowledge_type': 'consolidated'
                }
            }
            events.append(event)
    
    return events

def extract_task_events(agent_dir, agent_name):
    """
    Extract task execution events from working files
    """
    events = []
    
    working_files = sorted(agent_dir.glob('working_*.md'))
    
    for working_file in working_files:
        file_timestamp = working_file.stat().st_mtime
        session_id = extract_session_id_from_filename(working_file.name)
        
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract task information from content
        tasks = extract_tasks_from_content(content)
        
        for task in tasks:
            # Task started event
            start_event = {
                'event_id': generate_event_id(agent_name, 'task_start', task['id']),
                'timestamp': datetime.fromtimestamp(file_timestamp - 3600).isoformat(),  # Estimate start time
                'event_type': 'TASK_STARTED',
                'agent_id': agent_name,
                'session_id': session_id,
                'payload': {
                    'task_id': task['id'],
                    'task_description': task['description'],
                    'task_category': task.get('category', 'general'),
                    'estimated_duration': task.get('duration', None),
                    'task_context': task.get('context', '')
                },
                'metadata': {
                    'source': 'working_file_analysis',
                    'file': str(working_file),
                    'estimation_method': 'timestamp_inference'
                }
            }
            events.append(start_event)
            
            # Task completed event (if completion detected)
            if task.get('completed', False):
                completion_event = {
                    'event_id': generate_event_id(agent_name, 'task_complete', task['id']),
                    'timestamp': datetime.fromtimestamp(file_timestamp).isoformat(),
                    'event_type': 'TASK_COMPLETED',
                    'agent_id': agent_name,
                    'session_id': session_id,
                    'payload': {
                        'task_id': task['id'],
                        'completion_status': task.get('status', 'success'),
                        'results': task.get('results', {}),
                        'performance_metrics': task.get('metrics', {}),
                        'lessons_learned': task.get('lessons', [])
                    },
                    'correlation_id': start_event['event_id'],
                    'metadata': {
                        'source': 'working_file_analysis',
                        'file': str(working_file),
                        'completion_evidence': task.get('completion_evidence', 'inferred')
                    }
                }
                events.append(completion_event)
    
    return events

def extract_collaboration_events():
    """
    Extract collaboration events from multi-agent session files
    """
    events = []
    
    # Find multi-agent session files
    session_files = list(Path('../AGENTS').glob('multi_agent_session_*.json'))
    
    for session_file in session_files:
        try:
            with open(session_file, 'r', encoding='utf-8') as f:
                session_data = json.load(f)
            
            session_id = extract_session_id_from_filename(session_file.name)
            session_timestamp = session_file.stat().st_mtime
            
            # Extract participating agents
            participants = extract_session_participants(session_data)
            
            # Collaboration started event
            start_event = {
                'event_id': generate_event_id('system', 'collaboration_start', session_id),
                'timestamp': datetime.fromtimestamp(session_timestamp - 7200).isoformat(),  # Estimate start
                'event_type': 'COLLABORATION_INITIATED',
                'agent_id': 'system',
                'session_id': session_id,
                'payload': {
                    'participants': participants,
                    'collaboration_type': infer_collaboration_type(session_data),
                    'session_goals': extract_session_goals(session_data),
                    'session_context': extract_session_context(session_data)
                },
                'metadata': {
                    'source': 'session_file_analysis',
                    'file': str(session_file)
                }
            }
            events.append(start_event)
            
            # Collaboration ended event
            end_event = {
                'event_id': generate_event_id('system', 'collaboration_end', session_id),
                'timestamp': datetime.fromtimestamp(session_timestamp).isoformat(),
                'event_type': 'COLLABORATION_ENDED',
                'agent_id': 'system',
                'session_id': session_id,
                'payload': {
                    'participants': participants,
                    'outcomes': extract_session_outcomes(session_data),
                    'duration_estimate': 7200,  # 2 hours default
                    'collaboration_metrics': extract_collaboration_metrics(session_data)
                },
                'correlation_id': start_event['event_id'],
                'metadata': {
                    'source': 'session_file_analysis',
                    'file': str(session_file)
                }
            }
            events.append(end_event)
            
        except (json.JSONDecodeError, FileNotFoundError) as e:
            continue
    
    return events
```

### Step 2: Event Stream Processing
```python
def process_event_stream(events):
    """
    Process and enrich the event stream with additional metadata
    """
    processed_events = []
    
    # Create event lookup for correlation
    event_lookup = {event['event_id']: event for event in events}
    
    for event in events:
        processed_event = event.copy()
        
        # Add sequence number
        processed_event['sequence_number'] = len(processed_events) + 1
        
        # Enrich with temporal context
        processed_event['temporal_context'] = analyze_temporal_context(event, events)
        
        # Add causation analysis
        if not event.get('causation_id'):
            causation_id = infer_causation(event, processed_events)
            if causation_id:
                processed_event['causation_id'] = causation_id
        
        # Add correlation analysis
        if not event.get('correlation_id'):
            correlation_ids = find_correlations(event, processed_events)
            if correlation_ids:
                processed_event['correlation_ids'] = correlation_ids
        
        # Enrich payload with computed fields
        processed_event['payload'] = enrich_event_payload(event, processed_events)
        
        processed_events.append(processed_event)
    
    return processed_events

def analyze_temporal_context(event, all_events):
    """
    Analyze temporal context around an event
    """
    event_time = datetime.fromisoformat(event['timestamp'])
    agent_id = event['agent_id']
    
    # Find events before and after this event for the same agent
    agent_events = [e for e in all_events if e['agent_id'] == agent_id]
    agent_events.sort(key=lambda e: e['timestamp'])
    
    current_index = next((i for i, e in enumerate(agent_events) 
                         if e['event_id'] == event['event_id']), None)
    
    context = {
        'preceding_events': [],
        'following_events': [],
        'time_since_last_event': None,
        'time_to_next_event': None
    }
    
    if current_index is not None:
        # Get preceding events (up to 3)
        start_idx = max(0, current_index - 3)
        context['preceding_events'] = [
            {
                'event_id': e['event_id'],
                'event_type': e['event_type'],
                'time_delta': (event_time - datetime.fromisoformat(e['timestamp'])).total_seconds()
            }
            for e in agent_events[start_idx:current_index]
        ]
        
        # Get following events (up to 3)
        end_idx = min(len(agent_events), current_index + 4)
        context['following_events'] = [
            {
                'event_id': e['event_id'],
                'event_type': e['event_type'],
                'time_delta': (datetime.fromisoformat(e['timestamp']) - event_time).total_seconds()
            }
            for e in agent_events[current_index + 1:end_idx]
        ]
        
        # Calculate time deltas
        if current_index > 0:
            prev_time = datetime.fromisoformat(agent_events[current_index - 1]['timestamp'])
            context['time_since_last_event'] = (event_time - prev_time).total_seconds()
        
        if current_index < len(agent_events) - 1:
            next_time = datetime.fromisoformat(agent_events[current_index + 1]['timestamp'])
            context['time_to_next_event'] = (next_time - event_time).total_seconds()
    
    return context

def create_event_projections(events):
    """
    Create materialized views (projections) for common queries
    """
    projections = {
        'agent_timeline': create_agent_timeline_projection(events),
        'capability_evolution': create_capability_evolution_projection(events),
        'knowledge_accumulation': create_knowledge_accumulation_projection(events),
        'collaboration_network': create_collaboration_network_projection(events),
        'performance_metrics': create_performance_metrics_projection(events)
    }
    
    return projections

def create_agent_timeline_projection(events):
    """
    Create agent timeline projection for efficient temporal queries
    """
    agent_timelines = {}
    
    for event in events:
        agent_id = event['agent_id']
        
        if agent_id not in agent_timelines:
            agent_timelines[agent_id] = {
                'agent_id': agent_id,
                'first_seen': event['timestamp'],
                'last_seen': event['timestamp'],
                'event_count': 0,
                'event_types': {},
                'timeline': []
            }
        
        timeline = agent_timelines[agent_id]
        
        # Update timeline metadata
        timeline['last_seen'] = max(timeline['last_seen'], event['timestamp'])
        timeline['event_count'] += 1
        
        # Track event type distribution
        event_type = event['event_type']
        timeline['event_types'][event_type] = timeline['event_types'].get(event_type, 0) + 1
        
        # Add to timeline
        timeline['timeline'].append({
            'event_id': event['event_id'],
            'timestamp': event['timestamp'],
            'event_type': event['event_type'],
            'session_id': event.get('session_id'),
            'summary': create_event_summary(event)
        })
    
    # Sort timelines by timestamp
    for timeline in agent_timelines.values():
        timeline['timeline'].sort(key=lambda e: e['timestamp'])
    
    return agent_timelines
```

### Step 3: Temporal Analysis Functions
```python
def create_temporal_analysis_functions():
    """
    Create functions for temporal analysis of the event stream
    """
    analysis_functions = {
        'time_travel_query': time_travel_query,
        'event_pattern_detection': detect_event_patterns,
        'causality_analysis': analyze_causality_chains,
        'temporal_clustering': cluster_events_temporally,
        'trend_analysis': analyze_temporal_trends
    }
    
    return analysis_functions

def time_travel_query(events, target_timestamp, agent_id=None):
    """
    Query system state at a specific point in time
    """
    target_time = datetime.fromisoformat(target_timestamp)
    
    # Filter events up to target time
    historical_events = [
        event for event in events
        if datetime.fromisoformat(event['timestamp']) <= target_time
    ]
    
    if agent_id:
        historical_events = [
            event for event in historical_events
            if event['agent_id'] == agent_id
        ]
    
    # Reconstruct state at target time
    state_reconstruction = {
        'timestamp': target_timestamp,
        'agents': {},
        'active_sessions': set(),
        'knowledge_state': {},
        'capability_state': {}
    }
    
    for event in historical_events:
        apply_event_to_state(event, state_reconstruction)
    
    return state_reconstruction

def detect_event_patterns(events, pattern_window_hours=24):
    """
    Detect recurring patterns in the event stream
    """
    patterns = []
    
    # Group events by time windows
    window_seconds = pattern_window_hours * 3600
    time_windows = {}
    
    for event in events:
        event_time = datetime.fromisoformat(event['timestamp'])
        window_key = int(event_time.timestamp() // window_seconds)
        
        if window_key not in time_windows:
            time_windows[window_key] = []
        
        time_windows[window_key].append(event)
    
    # Analyze patterns within windows
    for window_key, window_events in time_windows.items():
        window_patterns = analyze_window_patterns(window_events)
        patterns.extend(window_patterns)
    
    return patterns

def analyze_causality_chains(events):
    """
    Analyze causality chains in the event stream
    """
    causality_chains = []
    
    # Build causation graph
    causation_graph = {}
    for event in events:
        event_id = event['event_id']
        causation_id = event.get('causation_id')
        
        if causation_id:
            if causation_id not in causation_graph:
                causation_graph[causation_id] = []
            causation_graph[causation_id].append(event_id)
    
    # Find chains
    visited = set()
    
    for event in events:
        event_id = event['event_id']
        
        if event_id not in visited and not event.get('causation_id'):
            # This is a root event, trace its chain
            chain = trace_causation_chain(event_id, causation_graph, events)
            
            if len(chain) > 1:  # Only include multi-event chains
                causality_chains.append({
                    'root_event': event_id,
                    'chain_length': len(chain),
                    'chain_events': chain,
                    'duration': calculate_chain_duration(chain, events),
                    'agents_involved': get_chain_agents(chain, events)
                })
            
            visited.update(chain)
    
    return causality_chains
```

## Implementation Steps

1. **Historical Reconstruction**: Parse existing agent artifacts to reconstruct historical events
2. **Event Stream Creation**: Generate chronologically ordered event stream
3. **Event Enrichment**: Add temporal context, correlations, and causation analysis
4. **Projection Creation**: Build materialized views for common query patterns
5. **Analysis Functions**: Implement temporal analysis and pattern detection
6. **Stream Validation**: Verify event consistency and temporal ordering
7. **Export Stream**: Save event stream and projections

## Output Files
- `event_stream.jsonl`: Complete chronological event stream (JSON Lines format)
- `agent_timelines.json`: Per-agent timeline projections
- `collaboration_events.json`: Multi-agent collaboration events
- `causality_chains.json`: Event causation analysis
- `temporal_patterns.json`: Detected recurring patterns
- `projections/`: Directory with specialized projections
- `stream_metadata.json`: Stream statistics and quality metrics

## Stream Query Examples
```python
# Time travel queries
state_at_time = time_travel_query(events, "2024-01-15T12:00:00Z", "Debugger")

# Pattern detection
patterns = detect_event_patterns(events, pattern_window_hours=24)

# Causality analysis
chains = analyze_causality_chains(events)

# Agent activity analysis
activity = analyze_agent_activity(events, "Developer", days=30)
```