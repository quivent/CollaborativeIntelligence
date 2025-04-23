# Session Structure Standards

This document defines the standardized structure for all agent sessions in the Collaborative Intelligence system.

## Core Principles

1. **Sessions as Directories**
   - Each session is a dedicated directory, not a single file
   - All related materials for a session are contained within its directory
   - Session directories have descriptive human-readable names

2. **Metadata Separation**
   - Date and time information belongs in metadata files
   - Filenames should focus on content description, not temporal data
   - Technical identifiers should be in metadata, not visible in primary navigation

3. **Logical Organization**
   - Related concepts grouped in subdirectories
   - Implementation details separated from conceptual documentation
   - Reference materials distinguished from session-specific content

## Standard Session Structure

```
Sessions/
├── SessionName/             # Descriptive name without dates
│   ├── README.md            # Session overview and summary
│   ├── metadata.json        # Structured data (date, participants, etc.)
│   ├── implementation/      # Code and configuration created
│   │   ├── code.md          # Implementation details
│   │   └── config.json      # Configuration examples
│   ├── references/          # Related materials
│   │   ├── prior-art.md     # References to existing work
│   │   └── research.md      # Background information
│   └── outcomes.md          # Key decisions and action items
└── AnotherSession/
    └── ...
```

## Required Files

Every session directory must contain at minimum:

1. **README.md**
   - Overview of the session
   - Key objectives and participants
   - Summary of main outcomes
   - Links to important files within the session

2. **metadata.json**
   - Session date and time
   - Participating agents
   - Status information
   - Tags and categories
   - Related sessions
   - Files created or modified

3. **outcomes.md**
   - Concrete achievements
   - Learning outcomes
   - Future applications
   - Knowledge transfer notes

## File Naming Conventions

1. **Directory Names**
   - Use CamelCase for session directories
   - Names should be descriptive and human-readable
   - Avoid numerical prefixes unless absolutely necessary for ordering
   - Example: `FastActivationSystem`, not `2025-04-23_FastActivationSystem`

2. **File Names**
   - Use kebab-case for files within session directories
   - Focus on content description, not file type or date
   - Be consistent within a session directory
   - Example: `implementation-guide.md`, not `implementation_guide.md`

3. **Implementation Files**
   - Use domain-focused naming, not technical descriptions
   - Group related files in appropriate subdirectories
   - Include README.md files in subdirectories when needed
   - Example: `data-model.md`, not `json-schema.md`

## Cross-Referencing

1. **Between Sessions**
   - Use relative paths for links between sessions
   - Include session name and specific file
   - Example: `[See Fast Activation System](../FastActivationSystem/README.md)`

2. **To External Documentation**
   - Use absolute paths from repository root
   - Include file name and anchor if applicable
   - Example: `[See API Documentation](/Documentation/API.md#authentication)`

3. **Within a Session**
   - Use relative links between files in the same session
   - Example: `[Implementation Details](implementation-guide.md)`

## Implementation Examples

### Good Example: FastActivationSystem

```
Sessions/
└── FastActivationSystem/
    ├── README.md
    ├── metadata.json
    ├── implementation-guide.md
    ├── outcomes.md
    └── references/
        └── activation-research.md
```

### Bad Example: 2025_04_23_FastActivation.md

This approach puts all content in a single file with a date-based name, making it:
- Harder to navigate
- Difficult to update or extend
- Less discoverable by concept
- More challenging to cross-reference

## Additional Guidelines

1. **Session Summaries**
   - Keep README.md concise (< 100 lines)
   - Use headings to organize content
   - Include links to detailed documentation
   - Summarize key outcomes at the top

2. **Implementation Details**
   - Separate conceptual explanations from code
   - Include examples when appropriate
   - Explain why, not just how
   - Document alternatives considered

3. **Metadata Standardization**
   - Use consistent keys across all metadata.json files
   - Include version information
   - Track completion status
   - Tag with relevant categories