# Federated Schema Registry Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a federated schema registry where each agent publishes its interface contracts, data schemas, and semantic mappings for interoperability.

## Registry Architecture

### Core Components
```python
REGISTRY_COMPONENTS = {
    'schema_repository': 'Central storage for all agent schemas',
    'mapping_engine': 'Semantic translation between schemas',
    'compatibility_matrix': 'Version and interface compatibility tracking',
    'discovery_service': 'Agent capability and interface discovery',
    'validation_service': 'Schema compliance and validation'
}

SCHEMA_VERSION_FORMAT = "semantic_versioning"  # MAJOR.MINOR.PATCH
COMPATIBILITY_LEVELS = ['exact', 'compatible', 'adaptable', 'incompatible']
```

## Data Extraction Process

### Step 1: Agent Schema Discovery
```python
def discover_agent_schemas():
    """
    Discover and extract schema information from all agents
    """
    agent_schemas = {}
    
    agent_dirs = [d for d in Path('../AGENTS').iterdir() 
                  if d.is_dir() and d.name[0].isupper()]
    
    for agent_dir in agent_dirs:
        agent_name = agent_dir.name
        
        schema_info = {
            'agent_id': agent_name,
            'schema_version': generate_schema_version(agent_dir),
            'interfaces': extract_interfaces(agent_dir),
            'data_types': extract_data_types(agent_dir),
            'capabilities': extract_capability_contracts(agent_dir),
            'dependencies': extract_dependencies(agent_dir),
            'semantic_mappings': extract_semantic_mappings(agent_dir),
            'metadata': extract_schema_metadata(agent_dir)
        }
        
        agent_schemas[agent_name] = schema_info
    
    return agent_schemas

def generate_schema_version(agent_dir):
    """
    Generate semantic version based on agent evolution
    """
    # Count working files to estimate version progression
    working_files = list(agent_dir.glob('working_*.md'))
    
    # Use timestamp patterns to determine version
    timestamps = []
    for file in working_files:
        match = re.search(r'working_(\d+)\.md', file.name)
        if match:
            timestamps.append(int(match.group(1)))
    
    if not timestamps:
        return "1.0.0"
    
    # Simple versioning based on file count and recency
    major = 1
    minor = min(len(timestamps) // 10, 9)  # Increment minor every 10 files
    patch = len(timestamps) % 10
    
    return f"{major}.{minor}.{patch}"

def extract_interfaces(agent_dir):
    """
    Extract interface definitions from agent documentation
    """
    interfaces = {}
    
    # Read main documentation files
    doc_files = ['README.md', 'agent.md', 'identity.md', 'MEMORY.md']
    
    for doc_file in doc_files:
        file_path = agent_dir / doc_file
        if file_path.exists():
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Extract capability descriptions as interfaces
            capabilities = extract_capabilities_with_signatures(content)
            
            for cap_name, cap_info in capabilities.items():
                interfaces[cap_name] = {
                    'input_schema': infer_input_schema(cap_info),
                    'output_schema': infer_output_schema(cap_info),
                    'description': cap_info.get('description', ''),
                    'requirements': cap_info.get('requirements', []),
                    'side_effects': cap_info.get('side_effects', [])
                }
    
    return interfaces

def extract_capabilities_with_signatures(content):
    """
    Extract capability definitions with input/output signatures
    """
    capabilities = {}
    
    # Look for structured capability descriptions
    capability_patterns = [
        r'## ([A-Z][A-Za-z\s]+)\n(.*?)(?=\n##|\n#|\Z)',  # Markdown headers
        r'\*\*([A-Z][A-Za-z\s]+)\*\*[:\n](.*?)(?=\n\*\*|\Z)',  # Bold headers
        r'- ([A-Z][A-Za-z\s]+):(.*?)(?=\n-|\Z)',  # List items
    ]
    
    for pattern in capability_patterns:
        matches = re.finditer(pattern, content, re.DOTALL | re.MULTILINE)
        
        for match in matches:
            cap_name = match.group(1).strip()
            cap_description = match.group(2).strip()
            
            # Skip very short descriptions
            if len(cap_description) < 10:
                continue
            
            capabilities[cap_name] = {
                'description': cap_description,
                'requirements': extract_requirements(cap_description),
                'side_effects': extract_side_effects(cap_description)
            }
    
    return capabilities

def infer_input_schema(capability_info):
    """
    Infer input schema from capability description
    """
    description = capability_info.get('description', '')
    
    # Look for input patterns
    input_patterns = [
        r'input[:\s]+([^\n\.]+)',
        r'takes[:\s]+([^\n\.]+)',
        r'requires[:\s]+([^\n\.]+)',
        r'given[:\s]+([^\n\.]+)'
    ]
    
    inputs = []
    for pattern in input_patterns:
        matches = re.findall(pattern, description, re.IGNORECASE)
        inputs.extend(matches)
    
    # Convert to structured schema
    schema = {'type': 'object', 'properties': {}}
    
    for input_desc in inputs:
        # Extract data types and field names
        fields = parse_input_description(input_desc)
        schema['properties'].update(fields)
    
    return schema

def infer_output_schema(capability_info):
    """
    Infer output schema from capability description
    """
    description = capability_info.get('description', '')
    
    # Look for output patterns
    output_patterns = [
        r'output[:\s]+([^\n\.]+)',
        r'returns[:\s]+([^\n\.]+)',
        r'produces[:\s]+([^\n\.]+)',
        r'generates[:\s]+([^\n\.]+)'
    ]
    
    outputs = []
    for pattern in output_patterns:
        matches = re.findall(pattern, description, re.IGNORECASE)
        outputs.extend(matches)
    
    # Convert to structured schema
    schema = {'type': 'object', 'properties': {}}
    
    for output_desc in outputs:
        fields = parse_output_description(output_desc)
        schema['properties'].update(fields)
    
    return schema

def parse_input_description(description):
    """
    Parse natural language input description into schema fields
    """
    fields = {}
    
    # Common data type patterns
    type_mappings = {
        r'\b(code|text|string|name|message)\b': 'string',
        r'\b(number|count|size|length|score)\b': 'number',
        r'\b(list|array|collection)\b': 'array',
        r'\b(object|data|structure)\b': 'object',
        r'\b(flag|boolean|true|false)\b': 'boolean'
    }
    
    # Extract field names and types
    words = description.lower().split()
    
    for word in words:
        for pattern, data_type in type_mappings.items():
            if re.search(pattern, word):
                # Create field name
                field_name = word.replace(',', '').replace('.', '')
                if field_name and len(field_name) > 2:
                    fields[field_name] = {'type': data_type}
                break
    
    return fields

def extract_data_types(agent_dir):
    """
    Extract custom data types used by the agent
    """
    data_types = {}
    
    # Look for structured data in working files
    working_files = list(agent_dir.glob('working_*.md'))
    
    for working_file in working_files:
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract JSON-like structures
        json_patterns = [
            r'\{[^{}]*\}',  # Simple JSON objects
            r'\[[^\[\]]*\]'  # JSON arrays
        ]
        
        for pattern in json_patterns:
            matches = re.findall(pattern, content)
            
            for match in matches:
                try:
                    # Attempt to parse as JSON
                    parsed = json.loads(match)
                    type_name = f"DataType_{hash(match) % 1000}"
                    data_types[type_name] = infer_schema_from_data(parsed)
                except json.JSONDecodeError:
                    continue
    
    return data_types
```

### Step 2: Semantic Mapping Generation
```python
def generate_semantic_mappings(agent_schemas):
    """
    Generate semantic mappings between different agent schemas
    """
    mappings = {}
    
    # Extract all field names and concepts across agents
    all_concepts = {}
    for agent_name, schema in agent_schemas.items():
        agent_concepts = extract_concepts_from_schema(schema)
        all_concepts[agent_name] = agent_concepts
    
    # Generate mappings between similar concepts
    for agent1, concepts1 in all_concepts.items():
        mappings[agent1] = {}
        
        for agent2, concepts2 in all_concepts.items():
            if agent1 == agent2:
                continue
            
            # Find semantic similarities
            concept_mappings = find_concept_mappings(concepts1, concepts2)
            if concept_mappings:
                mappings[agent1][agent2] = concept_mappings
    
    return mappings

def extract_concepts_from_schema(schema):
    """
    Extract semantic concepts from agent schema
    """
    concepts = set()
    
    # Extract from interface names
    for interface_name in schema.get('interfaces', {}):
        concepts.update(tokenize_concept_name(interface_name))
    
    # Extract from data type names
    for data_type_name in schema.get('data_types', {}):
        concepts.update(tokenize_concept_name(data_type_name))
    
    # Extract from capability names
    for capability_name in schema.get('capabilities', {}):
        concepts.update(tokenize_concept_name(capability_name))
    
    return concepts

def find_concept_mappings(concepts1, concepts2):
    """
    Find semantic mappings between concept sets
    """
    mappings = {}
    
    # Exact matches
    exact_matches = concepts1.intersection(concepts2)
    for concept in exact_matches:
        mappings[concept] = concept
    
    # Semantic similarity matches
    from difflib import SequenceMatcher
    
    remaining_concepts1 = concepts1 - exact_matches
    remaining_concepts2 = concepts2 - exact_matches
    
    for concept1 in remaining_concepts1:
        best_match = None
        best_similarity = 0.0
        
        for concept2 in remaining_concepts2:
            similarity = SequenceMatcher(None, concept1, concept2).ratio()
            
            # Also check for semantic relationships
            semantic_sim = calculate_semantic_similarity(concept1, concept2)
            combined_similarity = (similarity + semantic_sim) / 2
            
            if combined_similarity > best_similarity and combined_similarity > 0.6:
                best_similarity = combined_similarity
                best_match = concept2
        
        if best_match:
            mappings[concept1] = best_match
    
    return mappings

def calculate_semantic_similarity(concept1, concept2):
    """
    Calculate semantic similarity between concepts
    """
    # Simple semantic relationships
    semantic_pairs = [
        ('code', 'program'), ('debug', 'fix'), ('analyze', 'examine'),
        ('create', 'generate'), ('design', 'architect'), ('manage', 'control'),
        ('data', 'information'), ('result', 'output'), ('input', 'parameter'),
        ('user', 'client'), ('system', 'platform'), ('file', 'document')
    ]
    
    concept1_lower = concept1.lower()
    concept2_lower = concept2.lower()
    
    for pair in semantic_pairs:
        if (concept1_lower in pair and concept2_lower in pair):
            return 0.8
    
    # Check for containment
    if concept1_lower in concept2_lower or concept2_lower in concept1_lower:
        return 0.6
    
    return 0.0
```

### Step 3: Compatibility Matrix Generation
```python
def generate_compatibility_matrix(agent_schemas, semantic_mappings):
    """
    Generate compatibility matrix between all agent pairs
    """
    agents = list(agent_schemas.keys())
    compatibility_matrix = {}
    
    for agent1 in agents:
        compatibility_matrix[agent1] = {}
        
        for agent2 in agents:
            if agent1 == agent2:
                compatibility_matrix[agent1][agent2] = 'exact'
                continue
            
            compatibility = assess_compatibility(
                agent_schemas[agent1], 
                agent_schemas[agent2],
                semantic_mappings.get(agent1, {}).get(agent2, {})
            )
            
            compatibility_matrix[agent1][agent2] = compatibility
    
    return compatibility_matrix

def assess_compatibility(schema1, schema2, mappings):
    """
    Assess compatibility level between two agent schemas
    """
    compatibility_score = 0.0
    total_checks = 0
    
    # Check interface compatibility
    interfaces1 = set(schema1.get('interfaces', {}).keys())
    interfaces2 = set(schema2.get('interfaces', {}).keys())
    
    if interfaces1 or interfaces2:
        interface_overlap = len(interfaces1.intersection(interfaces2))
        interface_union = len(interfaces1.union(interfaces2))
        compatibility_score += interface_overlap / max(interface_union, 1)
        total_checks += 1
    
    # Check data type compatibility
    types1 = set(schema1.get('data_types', {}).keys())
    types2 = set(schema2.get('data_types', {}).keys())
    
    if types1 or types2:
        type_overlap = len(types1.intersection(types2))
        type_union = len(types1.union(types2))
        compatibility_score += type_overlap / max(type_union, 1)
        total_checks += 1
    
    # Check semantic mapping quality
    if mappings:
        mapping_quality = len(mappings) / max(len(interfaces1), len(interfaces2), 1)
        compatibility_score += mapping_quality
        total_checks += 1
    
    # Calculate average compatibility
    if total_checks == 0:
        return 'incompatible'
    
    avg_compatibility = compatibility_score / total_checks
    
    # Classify compatibility level
    if avg_compatibility >= 0.9:
        return 'exact'
    elif avg_compatibility >= 0.7:
        return 'compatible'
    elif avg_compatibility >= 0.4:
        return 'adaptable'
    else:
        return 'incompatible'
```

### Step 4: Registry Service Implementation
```python
def create_registry_service(agent_schemas, semantic_mappings, compatibility_matrix):
    """
    Create the federated schema registry service
    """
    registry = {
        'schemas': agent_schemas,
        'mappings': semantic_mappings,
        'compatibility': compatibility_matrix,
        'discovery_index': create_discovery_index(agent_schemas),
        'validation_rules': create_validation_rules(agent_schemas)
    }
    
    return registry

def create_discovery_index(agent_schemas):
    """
    Create searchable index for agent discovery
    """
    discovery_index = {
        'by_capability': {},
        'by_data_type': {},
        'by_domain': {},
        'by_interface': {}
    }
    
    for agent_name, schema in agent_schemas.items():
        # Index by capabilities
        for capability in schema.get('capabilities', {}):
            if capability not in discovery_index['by_capability']:
                discovery_index['by_capability'][capability] = []
            discovery_index['by_capability'][capability].append(agent_name)
        
        # Index by data types
        for data_type in schema.get('data_types', {}):
            if data_type not in discovery_index['by_data_type']:
                discovery_index['by_data_type'][data_type] = []
            discovery_index['by_data_type'][data_type].append(agent_name)
        
        # Index by interfaces
        for interface in schema.get('interfaces', {}):
            if interface not in discovery_index['by_interface']:
                discovery_index['by_interface'][interface] = []
            discovery_index['by_interface'][interface].append(agent_name)
        
        # Index by domain (inferred from agent name and capabilities)
        domain = infer_agent_domain(agent_name, schema)
        if domain not in discovery_index['by_domain']:
            discovery_index['by_domain'][domain] = []
        discovery_index['by_domain'][domain].append(agent_name)
    
    return discovery_index

def create_validation_rules(agent_schemas):
    """
    Create validation rules for schema compliance
    """
    validation_rules = {
        'required_fields': ['agent_id', 'schema_version', 'interfaces'],
        'version_format': r'^\d+\.\d+\.\d+$',
        'interface_requirements': {
            'input_schema': 'required',
            'output_schema': 'required',
            'description': 'required'
        },
        'data_type_requirements': {
            'type': 'required',
            'properties': 'optional'
        }
    }
    
    return validation_rules
```

### Step 5: Translation Engine
```python
def create_translation_engine(semantic_mappings):
    """
    Create translation engine for cross-agent communication
    """
    translation_engine = {
        'direct_mappings': semantic_mappings,
        'transformation_functions': generate_transformation_functions(semantic_mappings),
        'validation_schemas': create_translation_validation_schemas()
    }
    
    return translation_engine

def generate_transformation_functions(semantic_mappings):
    """
    Generate transformation functions for data translation
    """
    transformations = {}
    
    for source_agent, target_mappings in semantic_mappings.items():
        transformations[source_agent] = {}
        
        for target_agent, concept_mappings in target_mappings.items():
            transformation_func = create_transformation_function(concept_mappings)
            transformations[source_agent][target_agent] = transformation_func
    
    return transformations

def create_transformation_function(concept_mappings):
    """
    Create a transformation function for specific concept mappings
    """
    def transform_data(source_data):
        """
        Transform data from source schema to target schema
        """
        if not isinstance(source_data, dict):
            return source_data
        
        transformed_data = {}
        
        for source_key, value in source_data.items():
            # Find mapped key
            target_key = concept_mappings.get(source_key, source_key)
            
            # Apply value transformations if needed
            transformed_value = transform_value(value, source_key, target_key)
            
            transformed_data[target_key] = transformed_value
        
        return transformed_data
    
    return transform_data

def transform_value(value, source_key, target_key):
    """
    Apply value-level transformations based on key mappings
    """
    # Common transformations based on key patterns
    transformations = {
        ('count', 'number'): lambda x: int(x) if isinstance(x, str) else x,
        ('text', 'message'): lambda x: str(x),
        ('list', 'array'): lambda x: list(x) if not isinstance(x, list) else x,
        ('flag', 'boolean'): lambda x: bool(x)
    }
    
    # Find applicable transformation
    for (source_pattern, target_pattern), transform_func in transformations.items():
        if source_pattern in source_key.lower() and target_pattern in target_key.lower():
            try:
                return transform_func(value)
            except (ValueError, TypeError):
                pass
    
    return value
```

## Implementation Steps

1. **Discover Schemas**: Extract interface contracts from all agent directories
2. **Generate Mappings**: Create semantic mappings between agent concepts
3. **Build Compatibility Matrix**: Assess inter-agent compatibility levels
4. **Create Registry Service**: Implement central schema repository
5. **Build Translation Engine**: Enable cross-agent data transformation
6. **Validate System**: Test schema compliance and translation accuracy
7. **Deploy Registry**: Set up accessible registry service

## Output Files
- `agent_schemas.json`: All extracted agent schemas
- `semantic_mappings.json`: Cross-agent concept mappings
- `compatibility_matrix.json`: Agent compatibility assessments
- `discovery_index.json`: Searchable capability index
- `translation_rules.json`: Data transformation specifications
- `registry_api.py`: Registry service implementation
- `validation_schemas.json`: Schema validation rules

## Registry API Endpoints
```python
# Example API endpoints for the registry service
REGISTRY_ENDPOINTS = {
    'GET /agents': 'List all registered agents',
    'GET /agents/{agent_id}/schema': 'Get agent schema',
    'GET /agents/{agent_id}/compatible': 'Find compatible agents',
    'POST /translate/{source_agent}/{target_agent}': 'Translate data between agents',
    'GET /discover?capability={capability}': 'Discover agents by capability',
    'GET /mappings/{agent1}/{agent2}': 'Get semantic mappings between agents'
}
```