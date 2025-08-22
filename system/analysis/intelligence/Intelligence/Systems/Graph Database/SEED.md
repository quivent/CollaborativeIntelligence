# Graph Database Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a property graph database model (Neo4j-style) with nodes, relationships, and properties.

## Data Extraction Process

### Step 1: Schema Definition
```cypher
-- Core node types
CREATE CONSTRAINT agent_name FOR (a:Agent) REQUIRE a.name IS UNIQUE;
CREATE CONSTRAINT capability_name FOR (c:Capability) REQUIRE c.name IS UNIQUE;
CREATE CONSTRAINT knowledge_id FOR (k:Knowledge) REQUIRE k.id IS UNIQUE;
CREATE CONSTRAINT session_id FOR (s:Session) REQUIRE s.id IS UNIQUE;

-- Indexes for performance
CREATE INDEX agent_domain FOR (a:Agent) ON (a.domain);
CREATE INDEX capability_category FOR (c:Capability) ON (c.category);
CREATE INDEX knowledge_topic FOR (k:Knowledge) ON (k.topic);
```

### Step 2: Agent Node Extraction
```python
def extract_agent_nodes():
    """
    Extract agent nodes from AGENTS directory
    """
    agents = []
    
    for agent_dir in Path('../AGENTS').glob('[A-Z]*'):
        if not agent_dir.is_dir():
            continue
            
        agent_data = {
            'name': agent_dir.name,
            'properties': {}
        }
        
        # Extract from metadata.json
        metadata_file = agent_dir / 'metadata.json'
        if metadata_file.exists():
            with open(metadata_file) as f:
                metadata = json.load(f)
                agent_data['properties'].update(metadata)
        
        # Extract from README.md or agent.md
        readme_files = ['README.md', 'agent.md', 'identity.md']
        for readme_file in readme_files:
            readme_path = agent_dir / readme_file
            if readme_path.exists():
                with open(readme_path) as f:
                    content = f.read()
                    agent_data['properties']['description'] = extract_description(content)
                    agent_data['properties']['domain'] = infer_domain(content)
                break
        
        # Extract capabilities from memory files
        memory_files = list(agent_dir.glob('*memory*.md')) + list(agent_dir.glob('MEMORY.md'))
        capabilities = []
        for memory_file in memory_files:
            with open(memory_file) as f:
                content = f.read()
                capabilities.extend(extract_capabilities(content))
        
        agent_data['capabilities'] = list(set(capabilities))
        agents.append(agent_data)
    
    return agents
```

### Step 3: Relationship Extraction
```python
def extract_relationships(agents):
    """
    Extract relationships between agents based on:
    1. Working session collaborations
    2. Capability dependencies
    3. Knowledge sharing patterns
    4. Direct mentions in memory files
    """
    relationships = []
    
    # Session-based collaborations
    session_files = Path('../AGENTS').glob('multi_agent_session_*.json')
    for session_file in session_files:
        with open(session_file) as f:
            session_data = json.load(f)
            
        session_agents = extract_session_participants(session_data)
        session_id = extract_session_id(session_file.name)
        
        # Create collaboration relationships
        for i, agent1 in enumerate(session_agents):
            for agent2 in session_agents[i+1:]:
                relationships.append({
                    'source': agent1,
                    'target': agent2,
                    'type': 'COLLABORATED_IN_SESSION',
                    'properties': {
                        'session_id': session_id,
                        'timestamp': extract_session_timestamp(session_data)
                    }
                })
    
    # Capability-based dependencies
    for agent in agents:
        for capability in agent['capabilities']:
            # Find other agents with complementary capabilities
            for other_agent in agents:
                if agent['name'] != other_agent['name']:
                    dependency_strength = calculate_capability_dependency(
                        agent['capabilities'], 
                        other_agent['capabilities']
                    )
                    if dependency_strength > 0.3:
                        relationships.append({
                            'source': agent['name'],
                            'target': other_agent['name'],
                            'type': 'DEPENDS_ON_CAPABILITY',
                            'properties': {
                                'strength': dependency_strength,
                                'capability_overlap': find_overlapping_capabilities(
                                    agent['capabilities'], 
                                    other_agent['capabilities']
                                )
                            }
                        })
    
    # Memory mention relationships
    for agent in agents:
        memory_files = get_agent_memory_files(agent['name'])
        for memory_file in memory_files:
            content = read_file(memory_file)
            mentioned_agents = find_agent_mentions(content, [a['name'] for a in agents])
            
            for mentioned_agent in mentioned_agents:
                if mentioned_agent != agent['name']:
                    relationships.append({
                        'source': agent['name'],
                        'target': mentioned_agent,
                        'type': 'REFERENCES_IN_MEMORY',
                        'properties': {
                            'mention_count': count_mentions(content, mentioned_agent),
                            'context': extract_mention_context(content, mentioned_agent)
                        }
                    })
    
    return relationships
```

### Step 4: Knowledge Node Extraction
```python
def extract_knowledge_nodes(agents):
    """
    Extract knowledge nodes from agent working files and memory
    """
    knowledge_nodes = []
    
    for agent in agents:
        agent_dir = Path(f'../AGENTS/{agent["name"]}')
        
        # Process working files
        working_files = list(agent_dir.glob('working_*.md'))
        for working_file in working_files:
            with open(working_file) as f:
                content = f.read()
            
            knowledge_items = extract_knowledge_items(content)
            for item in knowledge_items:
                knowledge_nodes.append({
                    'id': f"{agent['name']}_{working_file.stem}_{item['id']}",
                    'topic': item['topic'],
                    'content': item['content'],
                    'confidence': item.get('confidence', 0.8),
                    'source_agent': agent['name'],
                    'source_file': str(working_file),
                    'timestamp': extract_file_timestamp(working_file)
                })
        
        # Process memory files
        memory_files = [f for f in agent_dir.glob('*') if 'memory' in f.name.lower()]
        for memory_file in memory_files:
            with open(memory_file) as f:
                content = f.read()
            
            learned_knowledge = extract_learned_knowledge(content)
            for knowledge in learned_knowledge:
                knowledge_nodes.append({
                    'id': f"{agent['name']}_memory_{knowledge['id']}",
                    'topic': knowledge['topic'],
                    'content': knowledge['content'],
                    'confidence': knowledge.get('confidence', 0.9),
                    'source_agent': agent['name'],
                    'source_file': str(memory_file),
                    'knowledge_type': 'accumulated_memory'
                })
    
    return knowledge_nodes
```

### Step 5: Cypher Generation
```python
def generate_cypher_statements(agents, relationships, knowledge_nodes):
    """
    Generate Cypher statements to populate the graph database
    """
    statements = []
    
    # Create agent nodes
    for agent in agents:
        properties_str = ', '.join([
            f"{k}: {json.dumps(v)}" for k, v in agent['properties'].items()
        ])
        
        cypher = f"""
        MERGE (a:Agent {{name: {json.dumps(agent['name'])}}})
        SET a += {{{properties_str}}}
        """
        statements.append(cypher)
    
    # Create capability nodes and relationships
    all_capabilities = set()
    for agent in agents:
        for capability in agent['capabilities']:
            all_capabilities.add(capability)
    
    for capability in all_capabilities:
        cypher = f"""
        MERGE (c:Capability {{name: {json.dumps(capability)}}})
        SET c.category = {json.dumps(categorize_capability(capability))}
        """
        statements.append(cypher)
    
    # Link agents to capabilities
    for agent in agents:
        for capability in agent['capabilities']:
            cypher = f"""
            MATCH (a:Agent {{name: {json.dumps(agent['name'])}}})
            MATCH (c:Capability {{name: {json.dumps(capability)}}})
            MERGE (a)-[:HAS_CAPABILITY]->(c)
            """
            statements.append(cypher)
    
    # Create knowledge nodes
    for knowledge in knowledge_nodes:
        properties_str = ', '.join([
            f"{k}: {json.dumps(v)}" for k, v in knowledge.items() if k != 'id'
        ])
        
        cypher = f"""
        MERGE (k:Knowledge {{id: {json.dumps(knowledge['id'])}}})
        SET k += {{{properties_str}}}
        """
        statements.append(cypher)
        
        # Link knowledge to source agent
        cypher = f"""
        MATCH (a:Agent {{name: {json.dumps(knowledge['source_agent'])}}})
        MATCH (k:Knowledge {{id: {json.dumps(knowledge['id'])}}})
        MERGE (a)-[:POSSESSES_KNOWLEDGE]->(k)
        """
        statements.append(cypher)
    
    # Create relationships
    for rel in relationships:
        properties_str = ', '.join([
            f"{k}: {json.dumps(v)}" for k, v in rel['properties'].items()
        ])
        
        cypher = f"""
        MATCH (a1:Agent {{name: {json.dumps(rel['source'])}}})
        MATCH (a2:Agent {{name: {json.dumps(rel['target'])}}})
        MERGE (a1)-[r:{rel['type']}]->(a2)
        SET r += {{{properties_str}}}
        """
        statements.append(cypher)
    
    return statements
```

## Implementation Steps

1. **Set up Neo4j Database**: Install and configure Neo4j instance
2. **Extract Agent Data**: Parse all agent directories for metadata and capabilities
3. **Build Relationship Maps**: Analyze collaborations, dependencies, and references
4. **Extract Knowledge**: Process working files and memory for knowledge items
5. **Generate Cypher**: Create database population scripts
6. **Validate Graph**: Run consistency checks and performance tests

## Useful Graph Queries

```cypher
-- Find most connected agents
MATCH (a:Agent)-[r]-(other)
RETURN a.name, COUNT(r) as connections
ORDER BY connections DESC

-- Find agents with complementary capabilities
MATCH (a1:Agent)-[:HAS_CAPABILITY]->(c1:Capability)
MATCH (a2:Agent)-[:HAS_CAPABILITY]->(c2:Capability)
WHERE a1 <> a2 AND c1.category = c2.category
RETURN a1.name, a2.name, c1.name, c2.name

-- Find knowledge propagation paths
MATCH path = (a1:Agent)-[:POSSESSES_KNOWLEDGE]->(k:Knowledge)<-[:POSSESSES_KNOWLEDGE]-(a2:Agent)
WHERE a1 <> a2
RETURN path

-- Shortest collaboration path between agents
MATCH path = shortestPath((a1:Agent)-[*]-(a2:Agent))
WHERE a1.name = "Debugger" AND a2.name = "Designer"
RETURN path
```

## Output Files
- `schema.cypher`: Database schema creation
- `agents.cypher`: Agent node creation statements  
- `relationships.cypher`: Relationship creation statements
- `knowledge.cypher`: Knowledge node creation statements
- `indexes.cypher`: Performance optimization indexes