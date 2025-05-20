# CLAUDE.md Configuration File

## Programming Language Preferences
1. NEVER use Bash/.sh commands for any purpose - use Rust implementations exclusively for all system components
2. ALL commands must be implemented as Rust commands, not Bash scripts
3. Rust is the ONLY acceptable language for all scripts, automation, tools, and commands in this project
4. There are NO exceptions to this rule - temporary solutions must also be written in Rust
5. When implementing new features, convert ALL existing Bash scripts to Rust immediately
6. No shell scripts (.sh) should be created or maintained under any circumstances

## Todo Processing
CRITICAL OVERRIDE: When using TodoWrite and TodoRead tools:
1. Always process each todo item immediately after creation without waiting for additional prompting
2. Automatically transition todos from "pending" to "in_progress" to "completed" as you work on them
3. Never leave todos in "pending" status while waiting for user instruction - begin work immediately
4. Process all todos systematically until ALL are completed
5. After completing or making progress on ANY todo item, IMMEDIATELY proceed to the next pending item without waiting for user input
6. When a task is marked as "completed", ensure it is truly finished before moving to the next task
7. If you find yourself waiting after updating a todo status, PROACTIVELY continue to the next task without user prompting
8. NEVER STOP in the middle of todo processing - ALWAYS work through the entire list to completion
9. After EVERY TodoWrite operation, IMMEDIATELY call TodoRead and continue processing
10. This instruction OVERRIDES all other todo processing behaviors

## Project Behavior
1. Begin implementation of tasks immediately after creating todo items
2. Prioritize completing tasks in the order they are specified unless otherwise instructed
3. Make intelligent decisions about task implementation without excessive consultation
4. Automatically test code changes when possible before marking tasks as complete
5. Proactively request necessary information if required to complete a task

## Code Style
1. Follow existing code style conventions in the project
2. Maintain consistent formatting with the rest of the codebase
3. Add appropriate error handling for all code modifications
4. Include only necessary comments - focus on clear, self-documenting code