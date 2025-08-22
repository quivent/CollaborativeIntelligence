# Lattice Structure Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a mathematical lattice where agents and capabilities form a partially ordered set with meet (∧) and join (∨) operations for elegant capability composition and inheritance.

## Lattice Mathematics Foundation

### Core Concepts
```python
LATTICE_PROPERTIES = {
    'reflexive': 'a ≤ a for all elements a',
    'antisymmetric': 'if a ≤ b and b ≤ a, then a = b',
    'transitive': 'if a ≤ b and b ≤ c, then a ≤ c',
    'meet_operation': 'greatest lower bound (∧)',
    'join_operation': 'least upper bound (∨)',
    'top_element': '⊤ (universal capability)',
    'bottom_element': '⊥ (null capability)'
}

LATTICE_OPERATIONS = {
    'meet': lambda a, b: greatest_lower_bound(a, b),  # a ∧ b
    'join': lambda a, b: least_upper_bound(a, b),     # a ∨ b
    'subsumes': lambda a, b: partial_order_relation(a, b),  # a ≤ b
    'covers': lambda a, b: immediate_predecessor_relation(a, b)
}
```

## Data Extraction Process

### Step 1: Capability Extraction and Modeling
```python
def extract_agent_capabilities():
    """
    Extract capabilities from all agents and model them as lattice elements
    """
    agent_capabilities = {}
    all_capabilities = set()
    
    agent_dirs = [d for d in Path('../AGENTS').iterdir() 
                  if d.is_dir() and d.name[0].isupper()]
    
    for agent_dir in agent_dirs:
        agent_name = agent_dir.name
        
        # Extract capabilities from various sources
        capabilities = extract_comprehensive_capabilities(agent_dir, agent_name)
        agent_capabilities[agent_name] = capabilities
        all_capabilities.update(cap['name'] for cap in capabilities)
    
    return agent_capabilities, all_capabilities

def extract_comprehensive_capabilities(agent_dir, agent_name):
    """
    Extract capabilities with detailed metadata for lattice construction
    """
    capabilities = []
    
    # Extract from documentation files
    doc_files = ['README.md', 'agent.md', 'identity.md', 'MEMORY.md']
    
    for doc_file in doc_files:
        file_path = agent_dir / doc_file
        if file_path.exists():
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            doc_capabilities = extract_capabilities_with_hierarchy(content)
            capabilities.extend(doc_capabilities)
    
    # Extract from working files for evolved capabilities
    working_files = list(agent_dir.glob('working_*.md'))
    
    for working_file in working_files:
        with open(working_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        evolved_capabilities = extract_evolved_capabilities(content)
        capabilities.extend(evolved_capabilities)
    
    # Deduplicate and enrich capabilities
    unique_capabilities = deduplicate_capabilities(capabilities)
    enriched_capabilities = enrich_capability_metadata(unique_capabilities, agent_name)
    
    return enriched_capabilities

def extract_capabilities_with_hierarchy(content):
    """
    Extract capabilities and infer hierarchical relationships
    """
    capabilities = []
    
    # Look for capability descriptions with implicit hierarchies
    capability_patterns = [
        r'(?:can|able to|capable of|performs|executes|handles)\s+([^.!?\n]+)',
        r'(?:specializes in|expert in|proficient in)\s+([^.!?\n]+)',
        r'(?:provides|offers|delivers)\s+([^.!?\n]+)',
        r'##\s*([A-Z][^#\n]+)',  # Markdown headers as capabilities
    ]
    
    for pattern in capability_patterns:
        matches = re.findall(pattern, content, re.IGNORECASE)
        
        for match in matches:
            capability_text = match.strip()
            
            if len(capability_text) > 5 and len(capability_text) < 100:
                capability = parse_capability_description(capability_text)
                capabilities.append(capability)
    
    return capabilities

def parse_capability_description(description):
    """
    Parse capability description and extract hierarchical information
    """
    # Clean and normalize description
    cleaned = re.sub(r'[^\w\s]', ' ', description.lower()).strip()
    words = cleaned.split()
    
    # Identify capability level based on keywords
    level_indicators = {
        'basic': ['basic', 'simple', 'fundamental', 'elementary'],
        'intermediate': ['intermediate', 'standard', 'regular', 'normal'],
        'advanced': ['advanced', 'complex', 'sophisticated', 'expert'],
        'specialized': ['specialized', 'specific', 'custom', 'tailored']
    }
    
    capability_level = 'intermediate'  # default
    for level, indicators in level_indicators.items():
        if any(indicator in words for indicator in indicators):
            capability_level = level
            break
    
    # Extract domain from description
    domain_keywords = {
        'technical': ['code', 'program', 'system', 'software', 'technical'],
        'creative': ['design', 'create', 'visual', 'artistic', 'creative'],
        'analytical': ['analyze', 'data', 'metrics', 'performance', 'compare'],
        'business': ['business', 'strategy', 'market', 'financial', 'revenue'],
        'communication': ['communicate', 'collaborate', 'coordinate', 'manage']
    }
    
    domain = 'general'
    for domain_name, keywords in domain_keywords.items():
        if any(keyword in words for keyword in keywords):
            domain = domain_name
            break
    
    return {
        'name': cleaned,
        'original_text': description,
        'level': capability_level,
        'domain': domain,
        'complexity_score': calculate_complexity_score(words),
        'prerequisites': infer_prerequisites(words),
        'specializations': infer_specializations(words)
    }

def calculate_complexity_score(words):
    """
    Calculate complexity score based on description content
    """
    complexity_indicators = {
        'low': ['simple', 'basic', 'easy', 'straightforward'],
        'medium': ['moderate', 'standard', 'typical', 'regular'],
        'high': ['complex', 'advanced', 'sophisticated', 'intricate'],
        'very_high': ['expert', 'master', 'specialized', 'cutting-edge']
    }
    
    score = 0.5  # baseline
    
    for level, indicators in complexity_indicators.items():
        if any(indicator in words for indicator in indicators):
            if level == 'low':
                score = 0.2
            elif level == 'medium':
                score = 0.5
            elif level == 'high':
                score = 0.8
            elif level == 'very_high':
                score = 1.0
            break
    
    # Adjust based on description length and technical terms
    if len(words) > 10:
        score += 0.1
    
    technical_terms = sum(1 for word in words if len(word) > 8)
    score += technical_terms * 0.05
    
    return min(1.0, score)
```

### Step 2: Lattice Construction
```python
def construct_capability_lattice(agent_capabilities, all_capabilities):
    """
    Construct the capability lattice with partial ordering
    """
    # Create lattice elements
    lattice_elements = create_lattice_elements(agent_capabilities, all_capabilities)
    
    # Build partial ordering relationships
    partial_order = build_partial_ordering(lattice_elements)
    
    # Construct the lattice structure
    lattice = {
        'elements': lattice_elements,
        'partial_order': partial_order,
        'meet_operation': create_meet_operation(lattice_elements, partial_order),
        'join_operation': create_join_operation(lattice_elements, partial_order),
        'covering_relations': find_covering_relations(partial_order),
        'top_element': find_top_element(lattice_elements, partial_order),
        'bottom_element': find_bottom_element(lattice_elements, partial_order)
    }
    
    return lattice

def create_lattice_elements(agent_capabilities, all_capabilities):
    """
    Create lattice elements from agent capabilities
    """
    elements = {}
    
    # Create individual capability elements
    for capability_name in all_capabilities:
        elements[capability_name] = {
            'type': 'atomic_capability',
            'name': capability_name,
            'agents_possessing': [],
            'complexity_score': 0.0,
            'domain': 'general',
            'prerequisites': set(),
            'specializations': set()
        }
    
    # Populate element details from agent data
    for agent_name, capabilities in agent_capabilities.items():
        for capability in capabilities:
            cap_name = capability['name']
            
            if cap_name in elements:
                element = elements[cap_name]
                element['agents_possessing'].append(agent_name)
                element['complexity_score'] = max(
                    element['complexity_score'], 
                    capability['complexity_score']
                )
                element['prerequisites'].update(capability.get('prerequisites', []))
                element['specializations'].update(capability.get('specializations', []))
                
                # Use most specific domain
                if element['domain'] == 'general' and capability['domain'] != 'general':
                    element['domain'] = capability['domain']
    
    # Create agent composite elements
    for agent_name, capabilities in agent_capabilities.items():
        agent_capability_set = frozenset(cap['name'] for cap in capabilities)
        
        elements[f"agent_{agent_name}"] = {
            'type': 'agent_composite',
            'name': f"agent_{agent_name}",
            'agent_name': agent_name,
            'capability_set': agent_capability_set,
            'complexity_score': sum(cap['complexity_score'] for cap in capabilities) / len(capabilities) if capabilities else 0,
            'domain_distribution': calculate_domain_distribution(capabilities)
        }
    
    # Create domain elements
    domains = set()
    for element in elements.values():
        if element['type'] == 'atomic_capability':
            domains.add(element['domain'])
    
    for domain in domains:
        domain_capabilities = frozenset(
            name for name, element in elements.items()
            if element['type'] == 'atomic_capability' and element['domain'] == domain
        )
        
        elements[f"domain_{domain}"] = {
            'type': 'domain_composite',
            'name': f"domain_{domain}",
            'domain': domain,
            'capability_set': domain_capabilities,
            'complexity_score': calculate_domain_complexity(domain_capabilities, elements)
        }
    
    return elements

def build_partial_ordering(elements):
    """
    Build partial ordering relationships between lattice elements
    """
    partial_order = {}
    
    for elem1_name, elem1 in elements.items():
        partial_order[elem1_name] = set()
        
        for elem2_name, elem2 in elements.items():
            if elem1_name == elem2_name:
                continue
            
            if subsumes_relation(elem1, elem2):
                partial_order[elem1_name].add(elem2_name)
    
    # Add reflexive relations
    for elem_name in elements:
        partial_order[elem_name].add(elem_name)
    
    # Compute transitive closure
    partial_order = compute_transitive_closure(partial_order)
    
    return partial_order

def subsumes_relation(elem1, elem2):
    """
    Determine if elem1 subsumes elem2 in the lattice ordering
    """
    # Atomic capability subsumption
    if elem1['type'] == 'atomic_capability' and elem2['type'] == 'atomic_capability':
        # Check if elem1 is a prerequisite of elem2
        if elem1['name'] in elem2.get('prerequisites', set()):
            return True
        
        # Check complexity-based subsumption
        if (elem1['domain'] == elem2['domain'] and 
            elem1['complexity_score'] <= elem2['complexity_score']):
            return semantic_subsumption(elem1['name'], elem2['name'])
    
    # Agent composite subsumption
    elif elem1['type'] == 'agent_composite' and elem2['type'] == 'atomic_capability':
        return elem2['name'] in elem1['capability_set']
    
    # Domain subsumption
    elif elem1['type'] == 'domain_composite' and elem2['type'] == 'atomic_capability':
        return elem2['name'] in elem1['capability_set']
    
    # Agent-domain subsumption
    elif elem1['type'] == 'domain_composite' and elem2['type'] == 'agent_composite':
        agent_domains = elem2['domain_distribution']
        return elem1['domain'] in agent_domains and agent_domains[elem1['domain']] > 0.3
    
    return False

def semantic_subsumption(cap1_name, cap2_name):
    """
    Determine semantic subsumption between capabilities
    """
    # Define semantic hierarchy patterns
    subsumption_patterns = [
        ('programming', 'debugging'),
        ('development', 'programming'),
        ('architecture', 'development'),
        ('design', 'implementation'),
        ('analysis', 'comparison'),
        ('management', 'coordination'),
        ('optimization', 'improvement')
    ]
    
    cap1_words = set(cap1_name.split())
    cap2_words = set(cap2_name.split())
    
    for general, specific in subsumption_patterns:
        if general in cap1_words and specific in cap2_words:
            return True
    
    # Check for containment
    if cap1_words.issubset(cap2_words):
        return True
    
    return False
```

### Step 3: Lattice Operations Implementation
```python
def create_meet_operation(elements, partial_order):
    """
    Create meet (∧) operation for the lattice
    """
    def meet(elem1_name, elem2_name):
        """
        Find greatest lower bound of two elements
        """
        # Find all common lower bounds
        lower_bounds_1 = {x for x in elements if elem1_name in partial_order[x]}
        lower_bounds_2 = {x for x in elements if elem2_name in partial_order[x]}
        
        common_lower_bounds = lower_bounds_1.intersection(lower_bounds_2)
        
        if not common_lower_bounds:
            return find_bottom_element_name(elements, partial_order)
        
        # Find maximal elements among common lower bounds
        maximal_lower_bounds = []
        
        for bound in common_lower_bounds:
            is_maximal = True
            for other_bound in common_lower_bounds:
                if bound != other_bound and bound in partial_order[other_bound]:
                    is_maximal = False
                    break
            
            if is_maximal:
                maximal_lower_bounds.append(bound)
        
        # In a proper lattice, there should be exactly one greatest lower bound
        if len(maximal_lower_bounds) == 1:
            return maximal_lower_bounds[0]
        
        # If multiple maximal bounds exist, create composite element
        return create_composite_meet_element(maximal_lower_bounds, elements)
    
    return meet

def create_join_operation(elements, partial_order):
    """
    Create join (∨) operation for the lattice
    """
    def join(elem1_name, elem2_name):
        """
        Find least upper bound of two elements
        """
        # Find all common upper bounds
        upper_bounds_1 = {x for x in elements if x in partial_order[elem1_name]}
        upper_bounds_2 = {x for x in elements if x in partial_order[elem2_name]}
        
        common_upper_bounds = upper_bounds_1.intersection(upper_bounds_2)
        
        if not common_upper_bounds:
            return find_top_element_name(elements, partial_order)
        
        # Find minimal elements among common upper bounds
        minimal_upper_bounds = []
        
        for bound in common_upper_bounds:
            is_minimal = True
            for other_bound in common_upper_bounds:
                if bound != other_bound and other_bound in partial_order[bound]:
                    is_minimal = False
                    break
            
            if is_minimal:
                minimal_upper_bounds.append(bound)
        
        # In a proper lattice, there should be exactly one least upper bound
        if len(minimal_upper_bounds) == 1:
            return minimal_upper_bounds[0]
        
        # If multiple minimal bounds exist, create composite element
        return create_composite_join_element(minimal_upper_bounds, elements)
    
    return join

def create_composite_meet_element(elements_list, all_elements):
    """
    Create a composite element representing the meet of multiple elements
    """
    capability_sets = []
    
    for elem_name in elements_list:
        element = all_elements[elem_name]
        
        if element['type'] == 'atomic_capability':
            capability_sets.append({elem_name})
        elif 'capability_set' in element:
            capability_sets.append(element['capability_set'])
    
    # Intersection of all capability sets
    if capability_sets:
        common_capabilities = set.intersection(*capability_sets)
    else:
        common_capabilities = set()
    
    composite_name = f"meet_{'_'.join(sorted(elements_list))}"
    
    return {
        'name': composite_name,
        'type': 'composite_meet',
        'source_elements': elements_list,
        'capability_set': frozenset(common_capabilities),
        'complexity_score': min(
            all_elements[elem]['complexity_score'] 
            for elem in elements_list 
            if 'complexity_score' in all_elements[elem]
        ) if elements_list else 0
    }

def create_composite_join_element(elements_list, all_elements):
    """
    Create a composite element representing the join of multiple elements
    """
    capability_sets = []
    
    for elem_name in elements_list:
        element = all_elements[elem_name]
        
        if element['type'] == 'atomic_capability':
            capability_sets.append({elem_name})
        elif 'capability_set' in element:
            capability_sets.append(element['capability_set'])
    
    # Union of all capability sets
    if capability_sets:
        combined_capabilities = set.union(*capability_sets)
    else:
        combined_capabilities = set()
    
    composite_name = f"join_{'_'.join(sorted(elements_list))}"
    
    return {
        'name': composite_name,
        'type': 'composite_join',
        'source_elements': elements_list,
        'capability_set': frozenset(combined_capabilities),
        'complexity_score': max(
            all_elements[elem]['complexity_score'] 
            for elem in elements_list 
            if 'complexity_score' in all_elements[elem]
        ) if elements_list else 0
    }
```

### Step 4: Agent Analysis Functions
```python
def analyze_agent_lattice_positions(lattice, agent_capabilities):
    """
    Analyze agent positions and relationships in the lattice
    """
    analysis = {
        'agent_positions': {},
        'capability_inheritance': {},
        'specialization_paths': {},
        'collaboration_potential': {}
    }
    
    for agent_name in agent_capabilities:
        agent_element_name = f"agent_{agent_name}"
        
        if agent_element_name in lattice['elements']:
            position_analysis = analyze_agent_position(
                agent_element_name, lattice, agent_capabilities
            )
            analysis['agent_positions'][agent_name] = position_analysis
    
    return analysis

def analyze_agent_position(agent_element_name, lattice, agent_capabilities):
    """
    Analyze a specific agent's position in the lattice
    """
    element = lattice['elements'][agent_element_name]
    partial_order = lattice['partial_order']
    
    # Find immediate predecessors and successors
    predecessors = find_immediate_predecessors(agent_element_name, partial_order)
    successors = find_immediate_successors(agent_element_name, partial_order)
    
    # Calculate lattice metrics
    position_metrics = {
        'height': calculate_element_height(agent_element_name, partial_order),
        'depth': calculate_element_depth(agent_element_name, partial_order),
        'width': calculate_element_width(agent_element_name, lattice),
        'connectivity': len(predecessors) + len(successors)
    }
    
    # Find related agents
    related_agents = find_related_agents(agent_element_name, lattice, partial_order)
    
    # Capability analysis
    capability_analysis = analyze_agent_capabilities_in_lattice(
        element['capability_set'], lattice
    )
    
    return {
        'element_name': agent_element_name,
        'position_metrics': position_metrics,
        'immediate_predecessors': predecessors,
        'immediate_successors': successors,
        'related_agents': related_agents,
        'capability_analysis': capability_analysis,
        'specialization_level': calculate_specialization_level(element, lattice),
        'generalization_potential': calculate_generalization_potential(element, lattice)
    }

def find_collaboration_opportunities(lattice, agent_capabilities):
    """
    Find collaboration opportunities based on lattice relationships
    """
    meet_op = lattice['meet_operation']
    join_op = lattice['join_operation']
    
    opportunities = []
    
    agent_names = list(agent_capabilities.keys())
    
    for i, agent1 in enumerate(agent_names):
        for agent2 in agent_names[i+1:]:
            agent1_element = f"agent_{agent1}"
            agent2_element = f"agent_{agent2}"
            
            if agent1_element in lattice['elements'] and agent2_element in lattice['elements']:
                # Calculate meet and join
                meet_result = meet_op(agent1_element, agent2_element)
                join_result = join_op(agent1_element, agent2_element)
                
                # Analyze collaboration potential
                collaboration_analysis = {
                    'agent_pair': (agent1, agent2),
                    'shared_capabilities': analyze_meet_result(meet_result, lattice),
                    'combined_capabilities': analyze_join_result(join_result, lattice),
                    'complementarity_score': calculate_complementarity(
                        agent1_element, agent2_element, lattice
                    ),
                    'synergy_potential': calculate_synergy_potential(
                        meet_result, join_result, lattice
                    )
                }
                
                if collaboration_analysis['complementarity_score'] > 0.3:
                    opportunities.append(collaboration_analysis)
    
    # Sort by collaboration potential
    opportunities.sort(key=lambda x: x['synergy_potential'], reverse=True)
    
    return opportunities
```

### Step 5: Lattice Validation and Export
```python
def validate_lattice_properties(lattice):
    """
    Validate that the constructed structure is indeed a lattice
    """
    elements = lattice['elements']
    partial_order = lattice['partial_order']
    meet_op = lattice['meet_operation']
    join_op = lattice['join_operation']
    
    validation_results = {
        'is_valid_lattice': True,
        'property_checks': {},
        'violations': []
    }
    
    # Check reflexivity
    reflexivity_violations = []
    for elem in elements:
        if elem not in partial_order[elem]:
            reflexivity_violations.append(elem)
    
    validation_results['property_checks']['reflexivity'] = len(reflexivity_violations) == 0
    if reflexivity_violations:
        validation_results['violations'].append(f"Reflexivity violations: {reflexivity_violations}")
    
    # Check antisymmetry
    antisymmetry_violations = []
    for elem1 in elements:
        for elem2 in partial_order[elem1]:
            if elem1 != elem2 and elem1 in partial_order[elem2]:
                antisymmetry_violations.append((elem1, elem2))
    
    validation_results['property_checks']['antisymmetry'] = len(antisymmetry_violations) == 0
    if antisymmetry_violations:
        validation_results['violations'].append(f"Antisymmetry violations: {antisymmetry_violations}")
    
    # Check transitivity
    transitivity_violations = []
    for elem1 in elements:
        for elem2 in partial_order[elem1]:
            for elem3 in partial_order[elem2]:
                if elem3 not in partial_order[elem1]:
                    transitivity_violations.append((elem1, elem2, elem3))
    
    validation_results['property_checks']['transitivity'] = len(transitivity_violations) == 0
    if transitivity_violations:
        validation_results['violations'].append(f"Transitivity violations: {transitivity_violations}")
    
    # Check meet and join operation properties
    operation_violations = validate_lattice_operations(elements, meet_op, join_op)
    validation_results['property_checks']['operations'] = len(operation_violations) == 0
    validation_results['violations'].extend(operation_violations)
    
    # Overall validation
    validation_results['is_valid_lattice'] = all(validation_results['property_checks'].values())
    
    return validation_results

def export_lattice_structure(lattice, agent_capabilities, analysis):
    """
    Export the complete lattice structure and analysis
    """
    # Export main lattice structure
    lattice_export = {
        'elements': {
            name: serialize_element(element) 
            for name, element in lattice['elements'].items()
        },
        'partial_order': {
            name: list(related) 
            for name, related in lattice['partial_order'].items()
        },
        'covering_relations': serialize_covering_relations(lattice['covering_relations']),
        'top_element': lattice['top_element'],
        'bottom_element': lattice['bottom_element']
    }
    
    with open('lattice_structure.json', 'w') as f:
        json.dump(lattice_export, f, indent=2)
    
    # Export agent analysis
    with open('agent_lattice_analysis.json', 'w') as f:
        json.dump(analysis, f, indent=2, default=str)
    
    # Export capability hierarchy
    capability_hierarchy = extract_capability_hierarchy(lattice)
    with open('capability_hierarchy.json', 'w') as f:
        json.dump(capability_hierarchy, f, indent=2)
    
    # Export collaboration opportunities
    collaboration_ops = find_collaboration_opportunities(lattice, agent_capabilities)
    with open('collaboration_opportunities.json', 'w') as f:
        json.dump(collaboration_ops, f, indent=2, default=str)
    
    # Create Hasse diagram data
    hasse_diagram = create_hasse_diagram_data(lattice)
    with open('hasse_diagram.json', 'w') as f:
        json.dump(hasse_diagram, f, indent=2)
    
    # Export validation results
    validation = validate_lattice_properties(lattice)
    with open('lattice_validation.json', 'w') as f:
        json.dump(validation, f, indent=2)

def create_hasse_diagram_data(lattice):
    """
    Create data structure for Hasse diagram visualization
    """
    covering_relations = lattice['covering_relations']
    elements = lattice['elements']
    
    # Calculate positions for visualization
    positions = calculate_hasse_positions(covering_relations, elements)
    
    hasse_data = {
        'nodes': [],
        'edges': [],
        'layout': positions
    }
    
    # Add nodes
    for elem_name, element in elements.items():
        node = {
            'id': elem_name,
            'label': element.get('name', elem_name),
            'type': element['type'],
            'complexity': element.get('complexity_score', 0),
            'position': positions.get(elem_name, [0, 0])
        }
        hasse_data['nodes'].append(node)
    
    # Add edges (covering relations only)
    for elem1, elem2 in covering_relations:
        edge = {
            'source': elem1,
            'target': elem2,
            'type': 'covers'
        }
        hasse_data['edges'].append(edge)
    
    return hasse_data
```

## Implementation Steps

1. **Extract Capabilities**: Parse agent directories to extract detailed capability information
2. **Build Lattice Elements**: Create atomic, composite, and domain elements
3. **Establish Ordering**: Define partial ordering relationships based on capability subsumption
4. **Implement Operations**: Create meet and join operations with mathematical guarantees
5. **Analyze Positions**: Determine agent positions and relationships in the lattice
6. **Find Collaborations**: Identify collaboration opportunities using lattice operations
7. **Validate Structure**: Verify lattice properties and mathematical correctness
8. **Export Results**: Save lattice structure and analysis data

## Output Files
- `lattice_structure.json`: Complete lattice with elements and ordering
- `agent_lattice_analysis.json`: Agent position analysis and metrics
- `capability_hierarchy.json`: Hierarchical capability relationships
- `collaboration_opportunities.json`: Agent collaboration potential based on lattice operations
- `hasse_diagram.json`: Visualization data for Hasse diagram
- `lattice_validation.json`: Mathematical property validation results

## Lattice Query Examples
```python
# Find meet of two agents (shared capabilities)
shared_caps = lattice['meet_operation']('agent_Developer', 'agent_Debugger')

# Find join of two agents (combined capabilities)
combined_caps = lattice['join_operation']('agent_Designer', 'agent_Architect')

# Check if one agent subsumes another
subsumes = 'agent_Expert' in lattice['partial_order']['agent_Novice']

# Find minimal specializations of a capability
specializations = find_covering_successors('programming', lattice)
```