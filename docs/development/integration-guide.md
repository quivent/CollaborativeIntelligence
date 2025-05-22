# Context-Aware Cross-Repository Integration

This document captures a discussion about implementing a context-aware system to help agents distinguish between home repository tasks and project-specific tasks in the Collaborative Intelligence (CI) system.

## Discussion Background

The CI system provides a list of agents (in AGENTS_INDEX.md) that are usable in other repositories via CLI integration. The challenge is that agents need to determine whether a request involves operations on the home repository (such as creating a new agent, consulting with another home agent, checking/updating memory or learning) versus actions specific to the project being built.

## Current Integration Mechanism

The current CI integration works through a chain of references:

1. **Claude CLI startup** - When Claude is started in a directory, it looks for CLAUDE.md or CLAUDE.local.md
2. **Project-level configuration** - CLAUDE.local.md contains references to the central CI repository
3. **CI System loading** - The system loads from the CI repository CLAUDE.md
4. **Agent activation** - Specific agents are loaded based on user input or configuration

This is configured through:
- `.collaborative-intelligence.json` with integration details
- `CLAUDE.local.md` that points to the CI system

## Task Classification Approaches

Several approaches were identified for distinguishing tasks:

1. **Agent Determination Logic**
   - AgentRecommender uses confidence-based scoring (75%+ strong, 50-74% good, <50% partial)
   - Analyzes task characteristics and matches against agent capability profiles
   - Uses natural language processing for task intent extraction

2. **Linguistic Activation**
   - Single-word commands (e.g., "Athena") directly activate specific agents
   - Phrase-based activation ("Recommend an agent") triggers AgentRecommender
   - Fast Activation System reduces response time by 70-80%

3. **Integration Types**
   - Embedded: CI directly within project repository
   - Sibling: CI as separate repository with references
   - Symlink: Linked from multiple projects

## Recommended Implementation

### 1. Repository Context Configuration

Add repository context awareness to the `.collaborative-intelligence.json` file:
```json
{
  "repository_context": "project", // or "ci_home"
  "home_repository_path": "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence",
  "relationship_type": "client", // or "home"
  "allowed_operations": ["memory_read", "agent_consult", "project_development"]
}
```

This explicitly defines the repository relationship and allowed operations.

### 2. Path-Aware Task Classifier

Create a directory path analysis system that recognizes patterns indicating home vs. project operations:

```javascript
// Path patterns that indicate home repository operations
const HOME_PATTERNS = [
  /AGENTS\/.+\/(MEMORY|ContinuousLearning)\.md/,
  /Sessions\/[^\/]+\/metadata\.json/,
  /^AgentManager\/create/i
];

// Path patterns that indicate project operations
const PROJECT_PATTERNS = [
  /src\/.+/,
  /tests\/.+/,
  /package\.json/,
  /project.*config/i
];

function determineTaskContext(requestText) {
  // Check for path patterns in request
  for (const pattern of HOME_PATTERNS) {
    if (pattern.test(requestText)) return "home";
  }
  
  for (const pattern of PROJECT_PATTERNS) {
    if (pattern.test(requestText)) return "project";
  }
  
  // Default based on repository context if no pattern match
  return currentRepositoryContext;
}
```

### 3. Intent Classification System

Implement natural language classification for operation intent:

```python
OPERATION_INTENTS = {
  "agent_management": [
    "create a new agent", "update agent memory", "update learning", 
    "consult with another agent", "check agent capabilities"
  ],
  "memory_operations": [
    "update memory", "check learning", "add to ContinuousLearning", 
    "synchronize memory", "extract learnings"
  ],
  "project_development": [
    "add feature", "fix bug", "refactor code", "optimize performance", 
    "write tests", "implement functionality"
  ]
}

def classify_intent(request_text):
  highest_score = 0
  classified_intent = "unknown"
  
  for intent, patterns in OPERATION_INTENTS.items():
    score = calculate_intent_match(request_text, patterns)
    if score > highest_score:
      highest_score = score
      classified_intent = intent
  
  # Route based on classification  
  if classified_intent in ["agent_management", "memory_operations"]:
    return "home"
  elif classified_intent == "project_development":
    return "project"
    
  # Default to current context if uncertain
  return current_repository_context
```

### 4. Operation Domain Tags

Add a lightweight tagging system:

```
#agent.create - Creating new agents (HOME only)
#agent.consult - Consulting with other agents (cross-repository)
#memory.update - Updating memory structures (HOME only)
#memory.read - Reading memory information (cross-repository)
#project.develop - Project-specific development (PROJECT only)
#project.analyze - Analyzing project structures (either)
```

### 5. Integration Protocol Improvements

Update `CLAUDE.md.template` to include repository context awareness instructions:

```markdown
## Repository Context Awareness

This system operates with context awareness between repositories:

1. **Home Repository Operations**
   - Agent memory updates
   - Learning system modifications
   - Agent creation and management
   - These operations are ONLY performed in the home repository

2. **Project Repository Operations**
   - Code development and modification
   - Project-specific analysis and optimization
   - Feature implementation and testing
   - These operations are performed in the current project

3. **Cross-Repository Operations**
   - Agent consultations
   - Memory reading (but not writing)
   - Knowledge application from memory to projects
   - These operations may span repositories with proper authorization
   
When receiving requests, first determine the appropriate repository context before proceeding with execution.
```

## Summary

The context-aware cross-repository integration system distinguishes between home repository and project-specific tasks through a layered configuration approach. When activated in a project, agents automatically detect task context using path analysis and intent classification. The `.collaborative-intelligence.json` file maintains this awareness with fields like `repository_context` and `home_repository_path`.

Users interact naturally with the system - no special syntax needed. Simply reference CI-system tasks ("update agent memory", "create a new agent") to trigger home repository operations, or project-specific tasks ("implement feature", "fix bug") for local project work. The system routes appropriately based on linguistic patterns and directory references in your request.

Agents handle most context detection automatically. The underlying intelligence analyzes request contents, checks for agent/memory operations vs. project development tasks, and switches context when needed. Cross-repository capabilities are controlled by the permission system, ensuring agents only perform appropriate operations in each context. Users can optionally use `#agent`, `#memory`, or `#project` tags to explicitly direct tasks to the appropriate context.