# Sparse Distributed Representation Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into high-dimensional sparse vectors where each agent is represented as a binary pattern with only 2-5% active dimensions.

## Vector Space Configuration

### Parameters
```python
VECTOR_DIMENSIONS = 10000  # High-dimensional space
SPARSITY_LEVEL = 0.02      # 2% of dimensions active
SEED_HASH_SIZE = 256       # For consistent hashing
OVERLAP_THRESHOLD = 0.3    # Minimum similarity for relationships
```

### Dimension Semantics
```python
# Dimension ranges for different types of features
DIMENSION_ALLOCATION = {
    'agent_type': (0, 500),           # Agent classification features
    'capabilities': (500, 3000),      # Capability micro-features  
    'domain_knowledge': (3000, 7000), # Domain-specific knowledge
    'interaction_patterns': (7000, 8500), # Behavioral patterns
    'temporal_features': (8500, 9500),    # Time-based characteristics
    'meta_features': (9500, 10000)        # High-level abstractions
}
```

## Data Extraction Process

### Step 1: Feature Extraction Pipeline
```python
def extract_agent_features(agent_name):
    """
    Extract all relevant features from agent directory
    """
    agent_dir = Path(f'../AGENTS/{agent_name}')
    features = {
        'agent_type': [],
        'capabilities': [],
        'domain_knowledge': [],
        'interaction_patterns': [],
        'temporal_features': [],
        'meta_features': []
    }
    
    # Agent type features from naming and structure
    features['agent_type'] = classify_agent_type(agent_name)
    
    # Capabilities from README and memory files
    readme_files = ['README.md', 'agent.md', 'identity.md', 'MEMORY.md']
    for readme_file in readme_files:
        readme_path = agent_dir / readme_file
        if readme_path.exists():
            content = read_file(readme_path)
            features['capabilities'].extend(extract_capabilities(content))
            features['domain_knowledge'].extend(extract_domain_knowledge(content))
    
    # Interaction patterns from working files
    working_files = list(agent_dir.glob('working_*.md'))
    features['interaction_patterns'] = analyze_interaction_patterns(working_files)
    
    # Temporal features from file timestamps and session data
    features['temporal_features'] = extract_temporal_patterns(agent_dir)
    
    # Meta features from overall agent analysis
    features['meta_features'] = compute_meta_features(features)
    
    return features

def classify_agent_type(agent_name):
    """
    Classify agent into broad categories
    """
    classifications = []
    
    name_lower = agent_name.lower()
    
    # Technical classifications
    if any(keyword in name_lower for keyword in 
           ['developer', 'engineer', 'architect', 'debugger']):
        classifications.append('technical_development')
    
    if any(keyword in name_lower for keyword in 
           ['database', 'cacher', 'consolidator']):
        classifications.append('technical_data')
    
    if any(keyword in name_lower for keyword in 
           ['security', 'crypto', 'bypasser']):
        classifications.append('technical_security')
    
    # Creative classifications  
    if any(keyword in name_lower for keyword in 
           ['designer', 'artist', 'creative']):
        classifications.append('creative_visual')
    
    if any(keyword in name_lower for keyword in 
           ['marketer', 'event']):
        classifications.append('creative_marketing')
    
    # Business classifications
    if any(keyword in name_lower for keyword in 
           ['business', 'cfo', 'financial', 'manager']):
        classifications.append('business_operations')
    
    # Analysis classifications
    if any(keyword in name_lower for keyword in 
           ['analyst', 'comparitor', 'benchmarker']):
        classifications.append('analysis_data')
    
    return classifications
```

### Step 2: Vector Encoding Algorithm
```python
def encode_features_to_vector(features):
    """
    Convert extracted features to sparse distributed representation
    """
    vector = np.zeros(VECTOR_DIMENSIONS, dtype=np.uint8)
    active_indices = set()
    
    # Encode each feature category
    for category, feature_list in features.items():
        dim_start, dim_end = DIMENSION_ALLOCATION[category]
        category_active = encode_feature_category(
            feature_list, dim_start, dim_end, len(feature_list) * 10
        )
        active_indices.update(category_active)
    
    # Ensure sparsity level
    target_active = int(VECTOR_DIMENSIONS * SPARSITY_LEVEL)
    if len(active_indices) > target_active:
        # Randomly sample to maintain sparsity
        active_indices = set(random.sample(list(active_indices), target_active))
    elif len(active_indices) < target_active:
        # Add random indices to reach target sparsity
        available_indices = set(range(VECTOR_DIMENSIONS)) - active_indices
        additional_needed = target_active - len(active_indices)
        active_indices.update(random.sample(list(available_indices), additional_needed))
    
    # Set active dimensions
    for idx in active_indices:
        vector[idx] = 1
    
    return vector, list(active_indices)

def encode_feature_category(features, dim_start, dim_end, num_activations):
    """
    Encode a list of features into a specific dimension range
    """
    active_indices = set()
    dim_range = dim_end - dim_start
    
    for feature in features:
        # Use consistent hashing to map features to dimensions
        feature_hash = hashlib.sha256(str(feature).encode()).hexdigest()
        
        # Generate multiple indices per feature using different hash seeds
        for i in range(min(5, num_activations // max(1, len(features)))):
            seed_hash = hashlib.sha256(f"{feature_hash}_{i}".encode()).hexdigest()
            idx = int(seed_hash[:8], 16) % dim_range + dim_start
            active_indices.add(idx)
            
            if len(active_indices) >= num_activations:
                break
    
    return active_indices
```

### Step 3: Similarity and Operations
```python
def cosine_similarity(vector1, vector2):
    """
    Compute cosine similarity between sparse vectors
    """
    dot_product = np.dot(vector1, vector2)
    norm1 = np.linalg.norm(vector1)
    norm2 = np.linalg.norm(vector2)
    
    if norm1 == 0 or norm2 == 0:
        return 0.0
    
    return dot_product / (norm1 * norm2)

def hamming_similarity(vector1, vector2):
    """
    Compute Hamming similarity (for sparse binary vectors)
    """
    intersection = np.sum(vector1 & vector2)
    union = np.sum(vector1 | vector2)
    
    if union == 0:
        return 1.0  # Both vectors are zero
    
    return intersection / union

def vector_union(vector1, vector2):
    """
    Combine capabilities of two agents (OR operation)
    """
    return vector1 | vector2

def vector_intersection(vector1, vector2):
    """
    Find shared capabilities (AND operation)
    """
    return vector1 & vector2

def find_similar_agents(target_vector, all_agent_vectors, threshold=0.3):
    """
    Find agents similar to target agent
    """
    similar_agents = []
    
    for agent_name, agent_vector in all_agent_vectors.items():
        similarity = cosine_similarity(target_vector, agent_vector)
        if similarity >= threshold:
            similar_agents.append((agent_name, similarity))
    
    return sorted(similar_agents, key=lambda x: x[1], reverse=True)
```

### Step 4: Agent Vector Generation
```python
def generate_all_agent_vectors():
    """
    Process all agents and generate their vector representations
    """
    agent_vectors = {}
    agent_features = {}
    active_indices_map = {}
    
    # Discover all agents
    agent_dirs = [d for d in Path('../AGENTS').iterdir() 
                  if d.is_dir() and d.name[0].isupper()]
    
    for agent_dir in agent_dirs:
        agent_name = agent_dir.name
        print(f"Processing agent: {agent_name}")
        
        # Extract features
        features = extract_agent_features(agent_name)
        agent_features[agent_name] = features
        
        # Generate vector representation
        vector, active_indices = encode_features_to_vector(features)
        agent_vectors[agent_name] = vector
        active_indices_map[agent_name] = active_indices
        
        print(f"  Vector sparsity: {np.sum(vector)}/{len(vector)} "
              f"({100*np.sum(vector)/len(vector):.2f}%)")
    
    return agent_vectors, agent_features, active_indices_map
```

### Step 5: Analysis and Export
```python
def analyze_vector_space(agent_vectors):
    """
    Analyze the generated vector space for patterns and quality
    """
    analysis = {
        'total_agents': len(agent_vectors),
        'vector_dimensions': VECTOR_DIMENSIONS,
        'average_sparsity': 0,
        'similarity_matrix': {},
        'clusters': {},
        'outliers': []
    }
    
    # Calculate average sparsity
    sparsities = [np.sum(vector) for vector in agent_vectors.values()]
    analysis['average_sparsity'] = np.mean(sparsities) / VECTOR_DIMENSIONS
    
    # Generate similarity matrix
    agent_names = list(agent_vectors.keys())
    for i, agent1 in enumerate(agent_names):
        analysis['similarity_matrix'][agent1] = {}
        for j, agent2 in enumerate(agent_names):
            if i <= j:  # Symmetric matrix, only compute upper triangle
                similarity = cosine_similarity(
                    agent_vectors[agent1], 
                    agent_vectors[agent2]
                )
                analysis['similarity_matrix'][agent1][agent2] = similarity
                
                if i != j:  # Don't include self-similarity
                    analysis['similarity_matrix'][agent2] = analysis['similarity_matrix'].get(agent2, {})
                    analysis['similarity_matrix'][agent2][agent1] = similarity
    
    # Identify clusters using similarity threshold
    cluster_threshold = 0.5
    analyzed_agents = set()
    cluster_id = 0
    
    for agent in agent_names:
        if agent in analyzed_agents:
            continue
            
        cluster = [agent]
        analyzed_agents.add(agent)
        
        for other_agent in agent_names:
            if (other_agent not in analyzed_agents and 
                analysis['similarity_matrix'][agent].get(other_agent, 0) >= cluster_threshold):
                cluster.append(other_agent)
                analyzed_agents.add(other_agent)
        
        if len(cluster) > 1:
            analysis['clusters'][f'cluster_{cluster_id}'] = cluster
            cluster_id += 1
        else:
            analysis['outliers'].append(agent)
    
    return analysis

def export_vector_data(agent_vectors, agent_features, active_indices_map, analysis):
    """
    Export all vector data to files
    """
    # Export sparse vectors (only active indices)
    sparse_export = {}
    for agent_name, indices in active_indices_map.items():
        sparse_export[agent_name] = {
            'active_indices': indices,
            'vector_length': VECTOR_DIMENSIONS,
            'sparsity': len(indices) / VECTOR_DIMENSIONS
        }
    
    with open('sparse_vectors.json', 'w') as f:
        json.dump(sparse_export, f, indent=2)
    
    # Export dense vectors for compatibility
    dense_export = {}
    for agent_name, vector in agent_vectors.items():
        dense_export[agent_name] = vector.tolist()
    
    with open('dense_vectors.json', 'w') as f:
        json.dump(dense_export, f, indent=2)
    
    # Export features
    with open('agent_features.json', 'w') as f:
        json.dump(agent_features, f, indent=2)
    
    # Export analysis
    with open('vector_analysis.json', 'w') as f:
        json.dump(analysis, f, indent=2)
    
    # Export similarity matrix as CSV for easy viewing
    import pandas as pd
    similarity_df = pd.DataFrame(analysis['similarity_matrix']).fillna(0)
    similarity_df.to_csv('similarity_matrix.csv')
```

## Implementation Steps

1. **Configure Vector Space**: Set dimensions, sparsity, and allocation ranges
2. **Extract Features**: Process all agent directories for comprehensive feature extraction
3. **Generate Vectors**: Encode features into sparse distributed representations
4. **Compute Similarities**: Calculate agent similarity metrics and relationships
5. **Analyze Clusters**: Identify natural groupings and outliers in the vector space
6. **Export Data**: Save vectors, features, and analysis results

## Output Files
- `sparse_vectors.json`: Efficient storage of active indices only
- `dense_vectors.json`: Full vector representations  
- `agent_features.json`: Extracted features for each agent
- `vector_analysis.json`: Clustering and similarity analysis
- `similarity_matrix.csv`: Agent-to-agent similarity scores

## Performance Considerations
- Memory efficient sparse storage
- Fast similarity computation using vectorized operations
- Batch processing for large agent collections
- Configurable sparsity levels for different use cases