# Multi-Modal Embedding Space Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a unified high-dimensional embedding space where different agent types occupy distinct subspaces with learned projection functions for cross-modal translation.

## Embedding Space Architecture

### Configuration
```python
EMBEDDING_DIMENSIONS = 1024
MODALITY_SUBSPACES = {
    'technical_agents': {
        'range': (0, 256),
        'types': ['Developer', 'Debugger', 'Engineer', 'Architect', 'Database']
    },
    'creative_agents': {
        'range': (256, 512), 
        'types': ['Designer', 'Artist', 'EventMarketer']
    },
    'business_agents': {
        'range': (512, 768),
        'types': ['BusinessOwner', 'CFO', 'BusinessManager', 'ClientAcquisition']
    },
    'shared_semantic': {
        'range': (768, 1024),
        'types': ['all']  # Shared concepts across all modalities
    }
}

ANCHOR_CONCEPTS = [
    'problem_solving', 'collaboration', 'analysis', 'optimization',
    'communication', 'planning', 'execution', 'learning'
]
```

## Data Extraction Process

### Step 1: Agent Modality Classification
```python
def classify_agent_modality(agent_name, agent_features):
    """
    Classify agents into modalities based on capabilities and domain
    """
    name_lower = agent_name.lower()
    capabilities = ' '.join(agent_features.get('capabilities', [])).lower()
    
    # Technical modality
    technical_keywords = [
        'develop', 'debug', 'code', 'architect', 'engineer', 'database',
        'system', 'security', 'crypto', 'cache', 'assembly', 'gpu'
    ]
    
    # Creative modality  
    creative_keywords = [
        'design', 'artist', 'creative', 'marketing', 'event', 'visual'
    ]
    
    # Business modality
    business_keywords = [
        'business', 'financial', 'cfo', 'manager', 'acquisition',
        'revenue', 'strategy', 'operations'
    ]
    
    # Analysis modality
    analysis_keywords = [
        'analyst', 'comparitor', 'benchmarker', 'documenter', 'consolidator'
    ]
    
    # Score each modality
    technical_score = sum(1 for kw in technical_keywords 
                         if kw in name_lower or kw in capabilities)
    creative_score = sum(1 for kw in creative_keywords 
                        if kw in name_lower or kw in capabilities)
    business_score = sum(1 for kw in business_keywords 
                        if kw in name_lower or kw in capabilities)
    analysis_score = sum(1 for kw in analysis_keywords 
                        if kw in name_lower or kw in capabilities)
    
    # Assign to primary modality
    scores = {
        'technical_agents': technical_score,
        'creative_agents': creative_score, 
        'business_agents': business_score,
        'analysis_agents': analysis_score
    }
    
    primary_modality = max(scores.items(), key=lambda x: x[1])[0]
    
    # Handle analysis agents as a subset of technical
    if primary_modality == 'analysis_agents':
        primary_modality = 'technical_agents'
    
    return primary_modality, scores

def extract_shared_concepts(agent_features):
    """
    Extract concepts that should exist in shared semantic space
    """
    shared_concepts = []
    
    # Extract from capabilities and memory
    text_content = ' '.join([
        ' '.join(agent_features.get('capabilities', [])),
        agent_features.get('description', ''),
        ' '.join(agent_features.get('domain_knowledge', []))
    ]).lower()
    
    # Look for anchor concepts
    for concept in ANCHOR_CONCEPTS:
        if concept.replace('_', ' ') in text_content or concept in text_content:
            shared_concepts.append(concept)
    
    # Extract abstract capabilities
    abstract_patterns = [
        r'manage\w*', r'coordin\w*', r'optim\w*', r'analyz\w*',
        r'creat\w*', r'design\w*', r'implement\w*', r'monitor\w*'
    ]
    
    import re
    for pattern in abstract_patterns:
        matches = re.findall(pattern, text_content)
        shared_concepts.extend(matches)
    
    return list(set(shared_concepts))
```

### Step 2: Feature Extraction for Embeddings
```python
def extract_embedding_features(agent_name):
    """
    Extract features specifically for embedding generation
    """
    agent_dir = Path(f'../AGENTS/{agent_name}')
    
    features = {
        'textual_features': [],
        'behavioral_features': [],
        'capability_features': [],
        'interaction_features': [],
        'temporal_features': []
    }
    
    # Textual features from documentation
    text_files = ['README.md', 'agent.md', 'identity.md', 'MEMORY.md']
    all_text = []
    
    for text_file in text_files:
        file_path = agent_dir / text_file
        if file_path.exists():
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                all_text.append(content)
    
    # Process combined text
    combined_text = ' '.join(all_text)
    features['textual_features'] = extract_textual_features(combined_text)
    
    # Behavioral features from working files
    working_files = list(agent_dir.glob('working_*.md'))
    features['behavioral_features'] = extract_behavioral_patterns(working_files)
    
    # Capability features from explicit capability lists
    features['capability_features'] = extract_explicit_capabilities(combined_text)
    
    # Interaction features from session data
    features['interaction_features'] = extract_interaction_features(agent_name)
    
    # Temporal features from file patterns
    features['temporal_features'] = extract_temporal_features(agent_dir)
    
    return features

def extract_textual_features(text):
    """
    Extract semantic features from text using NLP techniques
    """
    import re
    from collections import Counter
    
    # Clean and tokenize
    text = re.sub(r'[^\w\s]', ' ', text.lower())
    words = text.split()
    
    # Extract key terms
    stopwords = {'the', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by'}
    meaningful_words = [w for w in words if len(w) > 3 and w not in stopwords]
    
    # Get most frequent terms
    word_counts = Counter(meaningful_words)
    top_terms = [word for word, count in word_counts.most_common(50)]
    
    # Extract domain-specific terms
    domain_patterns = {
        'technical': r'\b(code|debug|system|architecture|database|algorithm|optimization)\b',
        'creative': r'\b(design|visual|creative|artistic|aesthetic|brand)\b',
        'business': r'\b(revenue|strategy|market|customer|profit|growth|scale)\b',
        'analytical': r'\b(analysis|data|metrics|performance|benchmark|compare)\b'
    }
    
    domain_features = []
    for domain, pattern in domain_patterns.items():
        matches = re.findall(pattern, text)
        if matches:
            domain_features.append(f"domain_{domain}")
    
    return {
        'top_terms': top_terms[:20],
        'domain_indicators': domain_features,
        'text_length': len(words),
        'vocabulary_richness': len(set(words)) / max(len(words), 1)
    }
```

### Step 3: Embedding Generation
```python
def generate_embeddings_with_subspaces(agents_features):
    """
    Generate embeddings for all agents with modality-specific subspaces
    """
    embeddings = {}
    modality_assignments = {}
    
    # Classify agents into modalities
    for agent_name, features in agents_features.items():
        modality, scores = classify_agent_modality(agent_name, features)
        modality_assignments[agent_name] = {
            'primary_modality': modality,
            'modality_scores': scores
        }
    
    # Generate embeddings for each modality
    for modality_name, modality_config in MODALITY_SUBSPACES.items():
        if modality_name == 'shared_semantic':
            continue  # Handle shared space separately
            
        # Get agents in this modality
        modality_agents = [
            agent for agent, assignment in modality_assignments.items()
            if assignment['primary_modality'] == modality_name
        ]
        
        if not modality_agents:
            continue
        
        # Generate modality-specific embeddings
        dim_start, dim_end = modality_config['range']
        modality_embeddings = generate_modality_embeddings(
            modality_agents, agents_features, dim_start, dim_end
        )
        
        # Store in main embeddings dict
        for agent_name, embedding in modality_embeddings.items():
            if agent_name not in embeddings:
                embeddings[agent_name] = np.zeros(EMBEDDING_DIMENSIONS)
            embeddings[agent_name][dim_start:dim_end] = embedding
    
    # Generate shared semantic embeddings
    shared_embeddings = generate_shared_semantic_embeddings(
        agents_features, MODALITY_SUBSPACES['shared_semantic']['range']
    )
    
    for agent_name, shared_embedding in shared_embeddings.items():
        if agent_name not in embeddings:
            embeddings[agent_name] = np.zeros(EMBEDDING_DIMENSIONS)
        dim_start, dim_end = MODALITY_SUBSPACES['shared_semantic']['range']
        embeddings[agent_name][dim_start:dim_end] = shared_embedding
    
    return embeddings, modality_assignments

def generate_modality_embeddings(agent_names, agents_features, dim_start, dim_end):
    """
    Generate embeddings for agents within a specific modality subspace
    """
    subspace_dim = dim_end - dim_start
    modality_embeddings = {}
    
    # Collect all features for this modality
    modality_feature_space = []
    for agent_name in agent_names:
        features = agents_features[agent_name]
        
        # Convert features to vector representation
        feature_vector = vectorize_features(features, subspace_dim)
        modality_feature_space.append(feature_vector)
        modality_embeddings[agent_name] = feature_vector
    
    # Apply PCA/dimensionality reduction if needed
    if len(modality_feature_space) > 1:
        from sklearn.decomposition import PCA
        from sklearn.preprocessing import StandardScaler
        
        # Standardize features
        scaler = StandardScaler()
        scaled_features = scaler.fit_transform(modality_feature_space)
        
        # Apply PCA to reduce to subspace dimensions
        pca = PCA(n_components=min(subspace_dim, len(scaled_features)))
        reduced_features = pca.fit_transform(scaled_features)
        
        # Update embeddings with reduced features
        for i, agent_name in enumerate(agent_names):
            embedding = np.zeros(subspace_dim)
            embedding[:reduced_features.shape[1]] = reduced_features[i]
            modality_embeddings[agent_name] = embedding
    
    return modality_embeddings

def generate_shared_semantic_embeddings(agents_features, dim_range):
    """
    Generate shared semantic space embeddings for all agents
    """
    dim_start, dim_end = dim_range
    subspace_dim = dim_end - dim_start
    shared_embeddings = {}
    
    # Extract shared concepts for all agents
    all_shared_concepts = {}
    for agent_name, features in agents_features.items():
        shared_concepts = extract_shared_concepts(features)
        all_shared_concepts[agent_name] = shared_concepts
    
    # Create concept vocabulary
    all_concepts = set()
    for concepts in all_shared_concepts.values():
        all_concepts.update(concepts)
    
    concept_vocab = list(all_concepts)
    
    # Generate embeddings based on concept presence
    for agent_name, concepts in all_shared_concepts.items():
        embedding = np.zeros(subspace_dim)
        
        # Simple encoding: each concept maps to multiple dimensions
        for i, concept in enumerate(concept_vocab):
            if concept in concepts:
                # Map concept to multiple dimensions with sine/cosine encoding
                for j in range(min(5, subspace_dim // len(concept_vocab))):
                    dim_idx = (i * 5 + j) % subspace_dim
                    embedding[dim_idx] = np.sin(i * 0.1 + j * 0.2)
        
        # Normalize
        norm = np.linalg.norm(embedding)
        if norm > 0:
            embedding = embedding / norm
        
        shared_embeddings[agent_name] = embedding
    
    return shared_embeddings
```

### Step 4: Cross-Modal Projection Functions
```python
def learn_cross_modal_projections(embeddings, modality_assignments):
    """
    Learn projection functions between different modalities
    """
    projections = {}
    
    # Define modality pairs for projection learning
    modality_pairs = [
        ('technical_agents', 'creative_agents'),
        ('technical_agents', 'business_agents'),
        ('creative_agents', 'business_agents')
    ]
    
    for source_modality, target_modality in modality_pairs:
        # Get agents from each modality
        source_agents = [
            agent for agent, assignment in modality_assignments.items()
            if assignment['primary_modality'] == source_modality
        ]
        target_agents = [
            agent for agent, assignment in modality_assignments.items()
            if assignment['primary_modality'] == target_modality
        ]
        
        if len(source_agents) == 0 or len(target_agents) == 0:
            continue
        
        # Use shared semantic space as bridge
        shared_range = MODALITY_SUBSPACES['shared_semantic']['range']
        source_range = MODALITY_SUBSPACES[source_modality]['range']
        target_range = MODALITY_SUBSPACES[target_modality]['range']
        
        # Learn projection via shared semantics
        projection_matrix = learn_projection_via_shared_space(
            embeddings, source_agents, target_agents,
            source_range, target_range, shared_range
        )
        
        projections[f"{source_modality}_to_{target_modality}"] = projection_matrix
        
        # Symmetric projection
        reverse_projection = learn_projection_via_shared_space(
            embeddings, target_agents, source_agents,
            target_range, source_range, shared_range
        )
        
        projections[f"{target_modality}_to_{source_modality}"] = reverse_projection
    
    return projections

def learn_projection_via_shared_space(embeddings, source_agents, target_agents,
                                    source_range, target_range, shared_range):
    """
    Learn projection between modalities using shared semantic space
    """
    from sklearn.linear_model import Ridge
    
    # Extract shared semantic features as common ground
    shared_start, shared_end = shared_range
    source_start, source_end = source_range
    target_start, target_end = target_range
    
    # Prepare training data using agents with high semantic similarity
    X_train = []  # Source modality features + shared features
    Y_train = []  # Target modality features
    
    # Find cross-modal agent pairs with high shared semantic similarity
    for source_agent in source_agents:
        source_shared = embeddings[source_agent][shared_start:shared_end]
        
        # Find most similar target agent based on shared semantics
        best_target = None
        best_similarity = -1
        
        for target_agent in target_agents:
            target_shared = embeddings[target_agent][shared_start:shared_end]
            similarity = np.dot(source_shared, target_shared)
            
            if similarity > best_similarity:
                best_similarity = similarity
                best_target = target_agent
        
        if best_target and best_similarity > 0.1:
            # Create training pair
            source_features = np.concatenate([
                embeddings[source_agent][source_start:source_end],
                embeddings[source_agent][shared_start:shared_end]
            ])
            target_features = embeddings[best_target][target_start:target_end]
            
            X_train.append(source_features)
            Y_train.append(target_features)
    
    if len(X_train) == 0:
        # Fallback: create identity-like projection
        source_dim = source_end - source_start
        target_dim = target_end - target_start
        return np.eye(min(source_dim, target_dim), target_dim)
    
    # Train projection model
    X_train = np.array(X_train)
    Y_train = np.array(Y_train)
    
    projection_model = Ridge(alpha=0.1)
    projection_model.fit(X_train, Y_train)
    
    return projection_model

def project_across_modalities(agent_embedding, source_modality, target_modality, projections):
    """
    Project an agent embedding from one modality to another
    """
    projection_key = f"{source_modality}_to_{target_modality}"
    
    if projection_key not in projections:
        raise ValueError(f"No projection learned for {projection_key}")
    
    projection_model = projections[projection_key]
    
    # Extract source features
    source_range = MODALITY_SUBSPACES[source_modality]['range']
    shared_range = MODALITY_SUBSPACES['shared_semantic']['range']
    
    source_features = np.concatenate([
        agent_embedding[source_range[0]:source_range[1]],
        agent_embedding[shared_range[0]:shared_range[1]]
    ])
    
    # Project to target modality
    projected_features = projection_model.predict([source_features])[0]
    
    # Create full embedding with projected features
    target_range = MODALITY_SUBSPACES[target_modality]['range']
    projected_embedding = agent_embedding.copy()
    projected_embedding[target_range[0]:target_range[1]] = projected_features
    
    return projected_embedding
```

### Step 5: Export and Analysis
```python
def export_multi_modal_embeddings(embeddings, modality_assignments, projections):
    """
    Export embeddings and analysis data
    """
    # Export embeddings
    embeddings_export = {}
    for agent_name, embedding in embeddings.items():
        embeddings_export[agent_name] = {
            'full_embedding': embedding.tolist(),
            'modality_subspaces': {}
        }
        
        # Extract subspace embeddings
        for modality_name, config in MODALITY_SUBSPACES.items():
            start, end = config['range']
            subspace_embedding = embedding[start:end]
            embeddings_export[agent_name]['modality_subspaces'][modality_name] = subspace_embedding.tolist()
    
    with open('multi_modal_embeddings.json', 'w') as f:
        json.dump(embeddings_export, f, indent=2)
    
    # Export modality assignments
    with open('modality_assignments.json', 'w') as f:
        json.dump(modality_assignments, f, indent=2)
    
    # Export projection matrices (serialize sklearn models)
    import joblib
    joblib.dump(projections, 'projection_models.pkl')
    
    # Create analysis report
    analysis = analyze_embedding_space(embeddings, modality_assignments)
    with open('embedding_analysis.json', 'w') as f:
        json.dump(analysis, f, indent=2)

def analyze_embedding_space(embeddings, modality_assignments):
    """
    Analyze the multi-modal embedding space
    """
    analysis = {
        'modality_statistics': {},
        'cross_modal_similarities': {},
        'within_modal_similarities': {},
        'projection_quality': {}
    }
    
    # Analyze each modality
    for modality_name in MODALITY_SUBSPACES.keys():
        modality_agents = [
            agent for agent, assignment in modality_assignments.items()
            if assignment['primary_modality'] == modality_name
        ]
        
        if len(modality_agents) == 0:
            continue
        
        # Get embeddings for this modality
        modality_embeddings = [embeddings[agent] for agent in modality_agents]
        
        # Calculate statistics
        mean_embedding = np.mean(modality_embeddings, axis=0)
        std_embedding = np.std(modality_embeddings, axis=0)
        
        analysis['modality_statistics'][modality_name] = {
            'agent_count': len(modality_agents),
            'mean_norm': float(np.linalg.norm(mean_embedding)),
            'average_std': float(np.mean(std_embedding)),
            'agents': modality_agents
        }
        
        # Within-modality similarities
        similarities = []
        for i, agent1 in enumerate(modality_agents):
            for j, agent2 in enumerate(modality_agents[i+1:], i+1):
                sim = cosine_similarity(embeddings[agent1], embeddings[agent2])
                similarities.append(sim)
        
        analysis['within_modal_similarities'][modality_name] = {
            'mean_similarity': float(np.mean(similarities)) if similarities else 0,
            'std_similarity': float(np.std(similarities)) if similarities else 0
        }
    
    return analysis
```

## Implementation Steps

1. **Configure Subspaces**: Define modality ranges and anchor concepts
2. **Classify Agents**: Assign agents to appropriate modalities  
3. **Extract Features**: Process agent data for embedding generation
4. **Generate Embeddings**: Create modality-specific and shared embeddings
5. **Learn Projections**: Train cross-modal translation functions
6. **Validate Space**: Analyze embedding quality and relationships
7. **Export Results**: Save embeddings, projections, and analysis

## Output Files
- `multi_modal_embeddings.json`: Full embeddings with subspace breakdowns
- `modality_assignments.json`: Agent-to-modality mapping
- `projection_models.pkl`: Trained cross-modal projection functions  
- `embedding_analysis.json`: Quality metrics and statistics
- `similarity_matrices/`: Modality-specific similarity data