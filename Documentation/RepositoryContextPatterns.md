# Repository Context Detection Patterns

This document defines patterns used to determine whether a task should be performed in the home repository (CI system) or in a project repository.

## Home Repository Patterns

### Path Patterns

These path patterns typically indicate operations that should be performed in the home repository:

```
AGENTS/*/
AGENTS/*/*.md
Sessions/*/
*/ContinuousLearning.md
*/MEMORY.md
*metadata.json
```

### Intent Patterns

These linguistic patterns typically indicate home repository operations:

```
create (a|new) agent
update (agent|memory|learning)
add to (memory|learning)
check (memory|learning)
consult with [agent name]
agent (capabilities|memory|learning)
```

## Project Repository Patterns

### Path Patterns

These path patterns typically indicate operations that should be performed in the project repository:

```
src/
tests/
docs/
*.js
*.py
*.ts
*.html
*.css
package.json
```

### Intent Patterns

These linguistic patterns typically indicate project repository operations:

```
(add|create|implement) (feature|functionality)
(fix|resolve) (bug|issue|error)
(refactor|improve|optimize) code
(write|update|add) (test|documentation)
(analyze|review) (code|performance)
(build|compile|run) (project|application)
```

## Operation Domain Tags

Users can explicitly tag requests to specify the repository context:

- `#agent` - Agent-related operations (typically home repository)
- `#memory` - Memory and learning operations (typically home repository)
- `#project` - Project development operations (typically project repository)
- `#analyze` - Analysis operations (can apply to either repository)

## Context Switching

When a task spans both repositories, the system will:

1. Detect the primary operation context
2. Perform home repository operations in the home repository
3. Apply knowledge and perform project operations in the project repository
4. Maintain proper separation between context-specific operations

## Examples

### Home Repository Operations

- "Create a new agent for optimization tasks"
- "Update the Athena agent's memory with recent learning"
- "Add these insights to RepositoryTopologist's ContinuousLearning.md"

### Project Repository Operations

- "Add a login feature to the application"
- "Fix the bug in src/utils/auth.js"
- "Optimize the database queries in the user profile module"

### Mixed Operations with Explicit Tags

- "#agent Check what TheFixer knows about React performance issues"
- "#memory Read what we've learned about authentication, then #project implement it in this project"
- "#project Analyze the current codebase structure, then #agent consult with RepositoryTopologist"