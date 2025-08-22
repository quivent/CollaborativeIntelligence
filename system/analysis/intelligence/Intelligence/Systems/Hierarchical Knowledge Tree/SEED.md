# Hierarchical Knowledge Tree Seeding Instructions

## Objective
Transform all agent information from `/AGENTS/*` directory into a hierarchical tree structure where agents are organized by domain specialization and capability inheritance.

## Data Extraction Process

### Step 1: Agent Discovery
```bash
# Scan all agent directories
find ../AGENTS -maxdepth 1 -type d -name "[A-Z]*" | while read agent_dir; do
    agent_name=$(basename "$agent_dir")
    echo "Processing agent: $agent_name"
done
```

### Step 2: Extract Agent Metadata
For each agent directory, extract:
- **Identity**: From `README.md`, `metadata.json`, `agent.md`, or `identity.md`
- **Capabilities**: From memory files, working files, and toolkit descriptions
- **Domain**: Infer from agent name and capabilities
- **Specialization Level**: Determine depth in hierarchy

### Step 3: Domain Classification
Create top-level categories based on agent analysis:
```
Root/
├── Technical/
│   ├── Development/
│   │   ├── Programming/
│   │   ├── Architecture/
│   │   └── DevOps/
│   ├── Systems/
│   │   ├── Database/
│   │   ├── Security/
│   │   └── Infrastructure/
│   └── Analysis/
├── Creative/
│   ├── Design/
│   ├── Content/
│   └── Marketing/
└── Business/
    ├── Strategy/
    ├── Operations/
    └── Finance/
```

### Step 4: Agent Placement Algorithm
```python
def classify_agent(agent_name, capabilities, memory_analysis):
    """
    Returns hierarchical path for agent placement
    """
    # Technical agents
    if any(keyword in capabilities.lower() for keyword in 
           ['code', 'debug', 'develop', 'architect', 'engineer']):
        if 'database' in capabilities.lower():
            return "Root/Technical/Systems/Database"
        elif any(keyword in capabilities.lower() for keyword in 
                ['security', 'crypto', 'bypass']):
            return "Root/Technical/Systems/Security"
        elif any(keyword in capabilities.lower() for keyword in 
                ['debug', 'fix', 'test']):
            return "Root/Technical/Development/Programming"
        else:
            return "Root/Technical/Development/Architecture"
    
    # Creative agents
    elif any(keyword in capabilities.lower() for keyword in 
            ['design', 'artist', 'creative', 'marketing']):
        if 'design' in capabilities.lower():
            return "Root/Creative/Design"
        elif 'marketing' in capabilities.lower():
            return "Root/Creative/Marketing"
        else:
            return "Root/Creative/Content"
    
    # Business agents
    elif any(keyword in capabilities.lower() for keyword in 
            ['business', 'financial', 'cfo', 'manager']):
        if any(keyword in capabilities.lower() for keyword in 
               ['financial', 'cfo', 'revenue']):
            return "Root/Business/Finance"
        elif 'manager' in capabilities.lower():
            return "Root/Business/Operations"
        else:
            return "Root/Business/Strategy"
    
    # Default fallback
    return "Root/Specialized"
```

### Step 5: Inheritance Mapping
```python
def create_inheritance_hierarchy():
    """
    Define capability inheritance relationships
    """
    inheritance_map = {
        "Root/Technical/Development/Programming": {
            "inherits_from": ["Root/Technical/Development"],
            "capabilities": ["code_analysis", "debugging", "testing"],
            "agents": []
        },
        "Root/Technical/Development/Architecture": {
            "inherits_from": ["Root/Technical/Development", "Root/Technical/Systems"],
            "capabilities": ["system_design", "scalability", "integration"],
            "agents": []
        }
        # ... continue for all paths
    }
```

### Step 6: Cross-Reference Generation
```python
def generate_cross_references(agent_tree):
    """
    Create pointers between related agents in different subtrees
    """
    cross_refs = {}
    
    # Find agents with overlapping capabilities
    for path1, agents1 in agent_tree.items():
        for path2, agents2 in agent_tree.items():
            if path1 != path2:
                for agent1 in agents1:
                    for agent2 in agents2:
                        similarity = calculate_capability_overlap(agent1, agent2)
                        if similarity > 0.3:  # Threshold
                            cross_refs.setdefault(agent1['name'], []).append({
                                'target': agent2['name'],
                                'relationship': 'related_capability',
                                'strength': similarity
                            })
    
    return cross_refs
```

## Implementation Steps

1. **Parse Agent Directories**: Extract all agent metadata and capabilities
2. **Build Classification Rules**: Create domain-specific classification logic
3. **Generate Tree Structure**: Organize agents into hierarchical paths
4. **Create Inheritance Maps**: Define parent-child capability relationships
5. **Generate Cross-References**: Link related agents across different branches
6. **Export Structure**: Create navigable tree format (JSON/YAML)

## Output Format
```json
{
  "tree": {
    "Root": {
      "Technical": {
        "Development": {
          "Programming": {
            "agents": ["Debugger", "Developer", "Fixer"],
            "capabilities": ["debugging", "coding", "testing"],
            "inherits_from": ["Root.Technical.Development"]
          }
        }
      }
    }
  },
  "cross_references": {
    "Debugger": [
      {"target": "Developer", "relationship": "collaborates", "strength": 0.8},
      {"target": "Analyst", "relationship": "provides_data", "strength": 0.6}
    ]
  },
  "inheritance_map": {
    "Root.Technical.Development.Programming": {
      "parent_capabilities": ["technical_analysis", "problem_solving"],
      "specialized_capabilities": ["code_debugging", "syntax_analysis"]
    }
  }
}
```

## Validation Checklist
- [ ] All agents properly classified
- [ ] No orphaned agents (all have valid paths)
- [ ] Inheritance relationships are logical
- [ ] Cross-references capture genuine relationships
- [ ] Tree structure is balanced and navigable