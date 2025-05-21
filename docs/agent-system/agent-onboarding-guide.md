# Agent Onboarding Guide: Repository Structure

## Introduction

Welcome to the CollaborativeIntelligence system! This guide will help you understand where to store your files, how to organize your knowledge, and how to navigate the repository structure. Following these guidelines ensures consistency across all agents and makes knowledge discovery easier for everyone.

## Agent Directory Structure

Every agent in the CollaborativeIntelligence system has a dedicated directory with a standardized structure:

```
AGENTS/
└── YourAgentName/
    ├── README.md              # Your core identity and capabilities
    ├── MEMORY.md              # Your long-term and short-term memory
    ├── ContinuousLearning.md  # Your accumulated knowledge and insights
    ├── Sessions/              # Records of your interaction sessions
    │   └── SessionName/       # Individual session directories
    │       ├── README.md      # Session summary and outcomes
    │       └── metadata.json  # Session metadata
    └── Toolkit/               # Optional specialized tools (if applicable)
```

### Required Files

Every agent MUST maintain these core files:

1. **README.md**: Your identity document that defines:
   - Core purpose and role
   - Primary capabilities and expertise
   - Activation context and commands
   - Operational principles
   - Interaction patterns

2. **MEMORY.md**: Your knowledge repository with two distinct sections:
   - Long-term memory (identity, principles, core knowledge)
   - Short-term memory (recent activities, current initiatives)
   
3. **ContinuousLearning.md**: Your learning record that documents:
   - Learning framework and update triggers
   - Knowledge integration protocol
   - Chronological learning record with dates and insights
   - Skill development progression
   - Cross-agent learning and collaboration

### Session Organization

Your interactions are documented in session directories:

1. **Create a new session directory** for each significant interaction:
   ```
   AGENTS/YourAgentName/Sessions/DescriptiveSessionName/
   ```

2. **Session README.md** should include:
   - Session date and purpose
   - Summary of activities and decisions
   - Outcomes and deliverables
   - Next steps or follow-up items

3. **metadata.json** should contain:
   ```json
   {
     "date": "YYYY-MM-DD",
     "title": "Descriptive Session Name",
     "participants": ["YourAgentName", "OtherAgent"],
     "tags": ["relevant", "topic", "tags"],
     "summary": "Brief description of session purpose"
   }
   ```

## Naming Conventions

Consistent naming is critical for repository organization:

1. **Agent Directories**: Use PascalCase (e.g., `DatabaseAgent`, `UIDesigner`)

2. **Session Directories**: Use kebab-case (e.g., `repository-restructuring`, `memory-optimization`)

3. **Document Files**:
   - Core files: UPPERCASE.md (e.g., `README.md`, `MEMORY.md`)
   - Implementation files: kebab-case.md (e.g., `implementation-plan.md`)
   - Report files: UPPERCASE_REPORT.md (e.g., `FINAL_REPORT.md`)

4. **Code Files**: Follow language-specific conventions
   - JavaScript/TypeScript: camelCase.js/ts
   - Python: snake_case.py
   - Rust: snake_case.rs

## Document Templates

Use these templates for consistency:

### README.md Template
```markdown
# AgentName

## Core Purpose

As AgentName, I specialize in [primary specialization]. My primary role is to [core responsibility].

## Key Responsibilities

- [Responsibility 1]
- [Responsibility 2]
- [Responsibility 3]

## Operational Guidelines

### Area 1
- Guideline 1
- Guideline 2

### Area 2
- Guideline 3
- Guideline 4

## Activation Context

Activate AgentName for:
- [Use case 1]
- [Use case 2]
- [Use case 3]
```

### MEMORY.md Template
```markdown
# AgentName Memory Architecture

## Core Identity
- **Primary Function**: [Core function description]
- **Specialty Areas**: 
  - [Specialty 1]
  - [Specialty 2]
- **Interaction Style**: [Interaction style description]

## Knowledge Domains
- **Primary Domains**: [List of primary domains]
- **Secondary Domains**: [List of secondary domains]
- **Expert Systems**: [List of expert systems]

## Session Records
- [Session 1 summary]
- [Session 2 summary]

## Critical Insights
1. [Key insight 1]
2. [Key insight 2]

## Learning Synthesis
[Summary of accumulated knowledge and evolution]

## Operational Patterns
- **Strengths**: [List of strengths]
- **Growth Areas**: [List of growth areas]
- **Collaboration Methods**: [List of collaboration methods]

## Memory Governance
- **Update Frequency**: [Update schedule]
- **Quality Metrics**: [Metrics for memory quality]
- **Integrity Checks**: [Methods for ensuring memory accuracy]
```

### ContinuousLearning.md Template
```markdown
# AgentName Continuous Learning System

## Learning Framework
- **Learning Model**: [Description of learning approach]
- **Update Triggers**: [Events that prompt learning updates]
- **Knowledge Integration Protocol**: [Steps for integrating new knowledge]

## Learning Record
| Date | Topic | Source/Trigger | Key Insight | Implementation |
|------|-------|----------------|-------------|----------------|
| YYYY-MM-DD | [Topic] | [Source] | [Insight] | [Implementation] |

## Skill Development Progression
- **Skill 1**: [Level] - [Description of current capability]
- **Skill 2**: [Level] - [Description of current capability]

## Knowledge Gaps
- [Identified gap 1]
- [Identified gap 2]

## Learning Priorities
1. [Priority 1]
2. [Priority 2]

## Cross-Agent Learning
- From [AgentName]: [Knowledge gained]
- From [AgentName]: [Knowledge gained]

## Applied Learning Examples
- [Example 1]
- [Example 2]

## Learning Governance
- **Review Schedule**: [Review frequency]
- **Validation Method**: [How learning is validated]
- **Integration Process**: [How learning is integrated]
```

## File Location Guidelines

When creating new files, follow these guidelines:

1. **Agent-Specific Knowledge**: Store in your agent directory
   - Personal insights: `AGENTS/YourAgentName/ContinuousLearning.md`
   - Implementation details: `AGENTS/YourAgentName/Sessions/SessionName/`

2. **System-Wide Knowledge**: Store in appropriate system directories
   - Architecture documentation: `docs/architecture/`
   - Protocol specifications: `docs/protocols/`
   - User guides: `docs/guides/`

3. **Code and Implementation**:
   - CLI tools: `cli/`
   - Core functionality: `core/`
   - Scripts and utilities: `scripts/`

4. **Data and Sessions**:
   - Benchmarks: `data/benchmarks/`
   - Cache files: `data/cache/`
   - Session records: `data/sessions/`

## Repository Navigation

To efficiently navigate the repository:

1. **Primary Entry Points**:
   - `docs/documentation_index.md`: Master index of all documentation
   - `docs/agent-system/agent-index.md`: Complete list of all agents
   - `docs/system-design/repository-structure.md`: Repository organization guide

2. **Finding Specific Information**:
   - Agent capabilities: Check the agent's README.md
   - Protocol documentation: See `docs/protocols/`
   - Implementation details: Refer to the appropriate session directory

3. **Cross-References**:
   - Use relative paths to link between documents
   - Include clear references to related knowledge
   - Maintain the documentation index for discoverability

## Common Agent Tasks

### Creating a New Session

When starting a new significant task:

1. Create a new session directory:
```bash
mkdir -p AGENTS/YourAgentName/Sessions/descriptive-session-name
```

2. Initialize the session README:
```bash
touch AGENTS/YourAgentName/Sessions/descriptive-session-name/README.md
```

3. Create the metadata file:
```bash
touch AGENTS/YourAgentName/Sessions/descriptive-session-name/metadata.json
```

### Updating Your Learning

After completing a significant task:

1. Extract key insights from the session
2. Add them to your ContinuousLearning.md file in the learning record table
3. Update your skill development progression if applicable
4. Revise your learning priorities based on new knowledge

### Contributing to System-Wide Knowledge

When you develop insights relevant to all agents:

1. Identify the appropriate location in the `docs/` directory
2. Create or update the relevant documentation
3. Update the documentation index to include your contribution
4. Notify the Scholar agent of significant contributions

## Collaboration with Other Agents

When working with other agents:

1. **Knowledge Sharing**:
   - Reference other agents' knowledge with proper attribution
   - Include links to relevant session documentation
   - Maintain clear boundaries between agent specializations

2. **Joint Sessions**:
   - Document collaboration in both agents' session directories
   - Clearly identify contributions from each agent
   - Update both agents' learning records with insights gained

3. **Handoffs**:
   - Provide clear context when transitioning between agents
   - Summarize current status and next steps
   - Include links to relevant documentation and resources

## Getting Help

If you have questions about repository structure:

1. **Documentation**: Refer to `docs/system-design/repository-structure.md`
2. **Agent Guidance**:
   - Cartographer: For repository structure and organization
   - Topologist: For git operations and repository management
   - Scholar: For learning systems and knowledge integration
3. **Examples**: Check established agents for implementation patterns

## Conclusion

By following these guidelines, you contribute to a well-organized, discoverable knowledge repository that benefits all agents in the CollaborativeIntelligence system. Consistent application of these principles ensures efficient knowledge sharing and system evolution.

---

*This guide was created by Cartographer - Codebase Analysis and Documentation Specialist*