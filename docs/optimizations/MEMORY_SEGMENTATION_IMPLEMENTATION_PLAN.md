# Memory Segmentation Implementation Plan

This document outlines the phased implementation plan for the neuroscience-inspired memory segmentation system.

## Phase 1: Core Infrastructure (Week 1-2)

### 1. Segment Marker System

**Implementation:**
```bash
#!/bin/bash
# segment_parser.sh
# Extract memory segments from agent memory files

extract_segment() {
  local file="$1"
  local type="$2"
  local tier="$3"
  local domain="$4"
  
  awk -v type_pattern="TYPE=$type" -v tier_pattern="TIER=$tier" -v domain_pattern="DOMAIN=$domain" '
    $0 ~ "<!-- MEMORY:" {
      if ($0 ~ type_pattern && $0 ~ tier_pattern && ($0 ~ domain_pattern || domain_pattern == "DOMAIN=")) {
        flag=1;
        print "<!-- SEGMENT_METADATA_START -->"
        print $0
        print "<!-- SEGMENT_METADATA_END -->"
        next
      }
    }
    $0 ~ "<!-- END_MEMORY -->" {
      if (flag == 1) {
        flag=0;
        print $0
        print ""
      }
    }
    flag {
      print
    }
  ' "$file"
}

list_segments() {
  local file="$1"
  
  grep -o "<!-- MEMORY:[^>]*" "$file" | while read -r segment_metadata; do
    # Extract key metadata
    type=$(echo "$segment_metadata" | grep -o "TYPE=[^:]*" | cut -d= -f2)
    tier=$(echo "$segment_metadata" | grep -o "TIER=[^:]*" | cut -d= -f2)
    domain=$(echo "$segment_metadata" | grep -o "DOMAIN=[^:]*" | cut -d= -f2)
    activation=$(echo "$segment_metadata" | grep -o "ACTIVATION=[^:]*" | cut -d= -f2)
    
    echo "Type: $type, Tier: $tier, Domain: $domain, Activation: $activation"
  done
}

# Example usage:
# extract_segment "/AGENTS/Athena/MEMORY.md" "IDENTITY" "1" ""
# list_segments "/AGENTS/Athena/MEMORY.md"
```

**Deliverables:**
- Segment marker syntax definition
- Parser for extracting segments
- Tools for listing and analyzing segments
- Documentation for segment creation

### 2. Memory Type Classification

**Types:**
- IDENTITY: Core agent definition (role, expertise, primary capabilities)
- SEMANTIC: Factual knowledge and conceptual understanding
- EPISODIC: Historical experiences and contextual examples
- PROCEDURAL: Process-based knowledge and operational guidelines
- RELATIONAL: Connection patterns between concepts/agents/systems

**Deliverables:**
- Classification guidelines document
- Type conversion recommendations for existing memory
- Examples of each memory type
- Default tier assignments for each type

### 3. Conversion Tool

**Implementation:**
```bash
#!/bin/bash
# convert_to_segmented.sh
# Convert existing memory to segmented format

convert_memory_file() {
  local input_file="$1"
  local output_file="$2"
  local agent_name="$3"
  
  # Create temporary file
  local temp_file=$(mktemp)
  
  # Process the file section by section based on markdown headers
  awk -v agent="$agent_name" '
    # Initialize variables
    BEGIN {
      in_section = 0
      section_level = 0
      section_title = ""
      section_content = ""
      current_type = ""
      current_tier = ""
      current_domain = ""
      current_activation = ""
    }
    
    # Detect section headers and generate appropriate memory segment markers
    /^#{1,6} / {
      # If we were in a section, close it
      if (in_section) {
        print "<!-- MEMORY:TYPE=" current_type ":TIER=" current_tier ":DOMAIN=" current_domain ":ACTIVATION=" current_activation ":CONNECTIONS=auto -->"
        print section_content
        print "<!-- END_MEMORY -->"
        print ""
      }
      
      # Start new section
      section_level = length($1)
      section_title = substr($0, section_level + 2)
      section_content = $0 "\n"
      in_section = 1
      
      # Determine segment type based on section title and content
      if (section_title ~ /Identity|Purpose|Role|About/) {
        current_type = "IDENTITY"
        current_tier = "1"
        current_domain = "core_purpose"
        current_activation = "low"
      } else if (section_title ~ /Principles|Guidelines|Rules/) {
        current_type = "SEMANTIC"
        current_tier = "1"
        current_domain = "guiding_principles"
        current_activation = "low"
      } else if (section_title ~ /Framework|Process|Technique|Method/) {
        current_type = "PROCEDURAL"
        current_tier = "1"
        current_domain = tolower(gensub(/[^a-zA-Z0-9]/, "_", "g", section_title))
        current_activation = "low"
      } else if (section_title ~ /Update|History|Session|Case/) {
        current_type = "EPISODIC"
        current_tier = "3"
        current_domain = tolower(gensub(/[^a-zA-Z0-9]/, "_", "g", section_title))
        current_activation = "high"
      } else {
        current_type = "SEMANTIC"
        current_tier = "2"
        current_domain = tolower(gensub(/[^a-zA-Z0-9]/, "_", "g", section_title))
        current_activation = "medium"
      }
      next
    }
    
    # Add to current section content
    {
      if (in_section) {
        section_content = section_content $0 "\n"
      } else {
        print
      }
    }
    
    # Close final section at end of file
    END {
      if (in_section) {
        print "<!-- MEMORY:TYPE=" current_type ":TIER=" current_tier ":DOMAIN=" current_domain ":ACTIVATION=" current_activation ":CONNECTIONS=auto -->"
        print section_content
        print "<!-- END_MEMORY -->"
      }
    }
  ' "$input_file" > "$temp_file"
  
  # Add header if needed
  if ! grep -q "^# " "$temp_file"; then
    echo "# ${agent_name} Memory Architecture" > "$output_file"
    echo "" >> "$output_file"
    cat "$temp_file" >> "$output_file"
  else
    cp "$temp_file" "$output_file"
  fi
  
  # Clean up
  rm "$temp_file"
  
  echo "Converted memory file: $output_file"
}

# Example usage:
# convert_memory_file "/AGENTS/Athena/MEMORY.md" "/AGENTS/Athena/MEMORY.segmented.md" "Athena"
```

**Deliverables:**
- Automatic conversion tool for existing memory files
- Backup mechanism for original files
- Validation tool for ensuring proper segmentation
- Documentation for manual adjustments

## Phase 2: Access Mechanism (Week 3-4)

### 1. Memory Loading System

**Implementation:**
```bash
#!/bin/bash
# memory_loader.sh
# Load segmented memory with parallel processing

load_segmented_memory() {
  local agent_name="$1"
  local context="$2"
  local memory_file="/AGENTS/${agent_name}/MEMORY.md"
  
  # Create temporary directory for segments
  local temp_dir=$(mktemp -d)
  
  # Extract all segments and their metadata
  grep -n -A 1 "<!-- MEMORY:" "$memory_file" | while read -r line; do
    if [[ "$line" =~ ^([0-9]+):.*TYPE=([^:]+):TIER=([^:]+):DOMAIN=([^:]+):ACTIVATION=([^:]+) ]]; then
      line_num="${BASH_REMATCH[1]}"
      type="${BASH_REMATCH[2]}"
      tier="${BASH_REMATCH[3]}"
      domain="${BASH_REMATCH[4]}"
      activation="${BASH_REMATCH[5]}"
      
      # Extract segment end line
      end_line=$(tail -n +$line_num "$memory_file" | grep -n "<!-- END_MEMORY -->" | head -1 | cut -d: -f1)
      end_line=$((line_num + end_line))
      
      # Extract segment content
      segment_file="${temp_dir}/${tier}_${type}_${domain}.md"
      sed -n "$line_num,${end_line}p" "$memory_file" > "$segment_file"
      
      # Add to processing queue
      echo "$tier $type $domain $activation $segment_file" >> "${temp_dir}/process_queue.txt"
    fi
  done
  
  # Load Tier 1 segments immediately
  while read -r tier type domain activation file; do
    if [[ "$tier" == "1" ]]; then
      cat "$file"
    fi
  done < "${temp_dir}/process_queue.txt"
  
  # Start background process to load Tier 2 and 3
  {
    # First Tier 2
    while read -r tier type domain activation file; do
      if [[ "$tier" == "2" ]]; then
        # TODO: Add context relevance check here
        cat "$file"
      fi
    done < "${temp_dir}/process_queue.txt"
    
    # Then Tier 3 if context-relevant
    if [[ -n "$context" ]]; then
      while read -r tier type domain activation file; do
        if [[ "$tier" == "3" ]] && [[ "$context" =~ $domain ]]; then
          cat "$file"
        fi
      done < "${temp_dir}/process_queue.txt"
    fi
    
    # Clean up
    rm -rf "$temp_dir"
  } &
}

# Example usage:
# load_segmented_memory "Athena" "memory_architecture"
```

**Deliverables:**
- Tiered memory loading function
- Parallel processing implementation
- Context-based filtering system
- Loading status feedback system

### 2. Activation Model

**Implementation Sketch:**
```python
# activation_model.py
# Spreading activation model for memory retrieval

class MemorySegment:
    def __init__(self, type, tier, domain, activation_threshold, connections):
        self.type = type
        self.tier = tier
        self.domain = domain
        self.activation_threshold = activation_threshold
        self.connections = connections
        self.content = ""
        
class ActivationSystem:
    def __init__(self, agent_name):
        self.agent_name = agent_name
        self.segments = self._load_segments()
        self.activation_map = {}
        self.decay_factor = 0.8
        
    def _load_segments(self):
        """Load segment metadata from agent memory file."""
        segments = []
        # Implementation to parse memory file and extract segments
        return segments
        
    def activate(self, trigger_concept, initial_activation=1.0):
        """Activate segments based on spreading activation."""
        # Reset activation map
        self.activation_map = {}
        
        # Find initial segments to activate
        initial_segments = []
        for segment in self.segments:
            if trigger_concept.lower() in segment.domain.lower():
                initial_segments.append(segment)
                
        # Initialize activation queue
        activation_queue = [(segment, initial_activation) for segment in initial_segments]
        
        # Process queue until empty or below threshold
        while activation_queue:
            segment, activation = activation_queue.pop(0)
            
            # Skip if already processed with higher activation
            if segment in self.activation_map and self.activation_map[segment] >= activation:
                continue
                
            # Set activation level
            self.activation_map[segment] = activation
            
            # Skip spreading if below threshold
            if activation < segment.activation_threshold:
                continue
                
            # Spread activation to connected segments
            for connection in segment.connections:
                connected_segment = self._find_segment_by_domain(connection)
                if connected_segment:
                    # Calculate connection strength (simplified)
                    connection_strength = 0.5
                    
                    # Calculate new activation level
                    new_activation = activation * connection_strength * self.decay_factor
                    
                    # Add to queue if not already processed with higher activation
                    if (connected_segment not in self.activation_map or 
                            self.activation_map[connected_segment] < new_activation):
                        activation_queue.append((connected_segment, new_activation))
                        
        # Return activated segments sorted by activation level
        return sorted(
            [(s, a) for s, a in self.activation_map.items()],
            key=lambda x: x[1],
            reverse=True
        )
```

**Deliverables:**
- Initial activation based on context/query
- Spreading activation between connected segments
- Activation threshold implementation
- Retrieval ordering based on activation strength

## Phase 3: Memory Updating Integration (Week 5-6)

### 1. Identify Current Update System

**Research Tasks:**
1. Locate memory update points in the codebase
2. Identify update patterns and workflows
3. Determine entry points for memory modification
4. Analyze memory update API calls

**Deliverables:**
- Documentation of current memory update system
- Flowchart of update process
- Entry points for modification
- Analysis of update patterns

### 2. Segment-Aware Update System

**Implementation Design:**
```python
# segment_updater.py
# Segmented memory update system

def update_memory_segment(agent_name, segment_type, domain, content, tier=None):
    """Update a specific memory segment while preserving structure."""
    
    memory_file = f"/AGENTS/{agent_name}/MEMORY.md"
    
    # Read existing file
    with open(memory_file, 'r') as f:
        memory_content = f.read()
    
    # Find target segment
    segment_pattern = f"<!-- MEMORY:TYPE={segment_type}:.*?DOMAIN={domain}:.*?-->.*?<!-- END_MEMORY -->"
    segment_regex = re.compile(segment_pattern, re.DOTALL)
    
    existing_segment = segment_regex.search(memory_content)
    
    if existing_segment:
        # Extract metadata
        metadata_match = re.search(r"<!-- MEMORY:(.*?)-->", existing_segment.group())
        metadata = metadata_match.group(1) if metadata_match else ""
        
        # Update tier if provided
        if tier:
            metadata = re.sub(r"TIER=[^:]*", f"TIER={tier}", metadata)
        
        # Create updated segment
        updated_segment = f"<!-- MEMORY:{metadata}-->\n{content}\n<!-- END_MEMORY -->"
        
        # Replace in file
        updated_memory = memory_content.replace(existing_segment.group(), updated_segment)
    else:
        # Create new segment
        if not tier:
            tier = "2"  # Default to Tier 2 for new segments
            
        # Generate connections based on content analysis
        connections = generate_connections(agent_name, content)
        
        # Create new segment
        new_segment = (f"<!-- MEMORY:TYPE={segment_type}:TIER={tier}:DOMAIN={domain}:"
                      f"ACTIVATION=medium:CONNECTIONS={connections} -->\n"
                      f"{content}\n"
                      f"<!-- END_MEMORY -->\n\n")
        
        # Add to file in appropriate position
        updated_memory = memory_content + new_segment
    
    # Write updated file
    with open(memory_file, 'w') as f:
        f.write(updated_memory)
        
    # Invalidate cache for this segment
    invalidate_segment_cache(agent_name, segment_type, domain)
    
    return True
```

**Deliverables:**
- Segment-aware update functions
- Metadata preservation during updates
- Automatic connection discovery
- Cache invalidation mechanism

### 3. Memory Consolidation Logic

**Implementation Design:**
```python
# memory_consolidation.py
# Knowledge consolidation for memory updates

def consolidate_knowledge(existing_content, new_content, memory_type):
    """Consolidate new knowledge with existing content based on memory type."""
    
    if memory_type == "EPISODIC":
        # For episodic memories, prepend new experiences
        return f"{new_content}\n\n{existing_content}"
        
    elif memory_type == "SEMANTIC":
        # For semantic memories, merge facts and concepts
        return merge_semantic_content(existing_content, new_content)
        
    elif memory_type == "PROCEDURAL":
        # For procedural memories, update steps and processes
        return update_procedural_content(existing_content, new_content)
        
    elif memory_type == "IDENTITY":
        # For identity memories, carefully integrate new attributes
        return integrate_identity_content(existing_content, new_content)
        
    else:
        # Default merge behavior
        return f"{existing_content}\n\n{new_content}"
        
def merge_semantic_content(existing, new):
    """Merge semantic knowledge avoiding duplicates and integrating new information."""
    # Implementation for merging semantic content
    
def update_procedural_content(existing, new):
    """Update procedural knowledge, replacing outdated processes."""
    # Implementation for updating procedural content
    
def integrate_identity_content(existing, new):
    """Carefully integrate new identity attributes without disrupting core identity."""
    # Implementation for integrating identity content
```

**Deliverables:**
- Type-specific consolidation logic
- Duplicate detection and resolution
- Conflict resolution strategies
- Integration patterns for different memory types

## Phase 4: Prototype Implementation (Week 7-8)

### 1. Agent Memory Conversion

**Priority Agents:**
1. Optimizer (as prototype)
2. Athena (critical memory architecture)
3. Manager (system oversight)
4. Developer (high usage)

**Deliverables:**
- Converted memory files for priority agents
- Validation of proper segmentation
- Documentation of conversion process
- Guidelines for other agent conversions

### 2. Integration Testing

**Test Scenarios:**
1. Agent activation with segmented memory
2. Context-based memory retrieval
3. Memory updates through various pathways
4. Parallel loading performance
5. Cache invalidation and rebuilding

**Deliverables:**
- Test scripts for each scenario
- Performance metrics comparison
- Reliability assessment
- Issue tracking and resolution

### 3. Documentation and Training

**Documentation:**
- Full system architecture documentation
- API reference for segment-aware functions
- Guidelines for memory structure and organization
- Best practices for memory updates

**Deliverables:**
- Developer documentation
- User guides for memory management
- Visual diagrams of system architecture
- Example implementations

## Timeline Overview

| Week | Focus | Key Deliverables |
|------|-------|------------------|
| 1-2 | Core Infrastructure | Segment marker system, conversion tools |
| 3-4 | Access Mechanism | Memory loading system, activation model |
| 5-6 | Update Integration | Update system adaptation, consolidation logic |
| 7-8 | Prototype Implementation | Priority agent conversion, testing, documentation |

## Initial Implementation Tasks

1. Create segment parser script
2. Define memory type classification guidelines
3. Develop conversion tool for existing memory
4. Generate example of segmented Athena memory
5. Build basic tier-based loading function
6. Identify memory update system entry points

This implementation plan provides a structured approach to introducing memory segmentation while ensuring compatibility with existing systems and maintaining human readability.