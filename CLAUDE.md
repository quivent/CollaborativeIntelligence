# Project: project
# Created: 2025-05-20 08:31:49
# Integration: Standalone

# Collaborative Intelligence Rust Configuration
This project is configured to use the CIR (Collaborative Intelligence in Rust) system
in standalone mode, which provides full functionality without requiring the original
CI repository.

_CIR.config('project_name', 'project')_
_CIR.config('created_at', '2025-05-20 08:31:49')_
_CIR.config('integration_type', 'standalone')_

## Available Agents
_CIR.load_agents('Athena,ProjectArchitect')_

## Project Context
This project uses the standalone CIR integration, which processes directives
directly within the CIR tool. No external CI repository is required.

## Custom Instructions
- Focus on code quality and maintainability
- Follow established patterns in the codebase
- Add appropriate error handling for all edge cases
- Include helpful comments for complex sections
