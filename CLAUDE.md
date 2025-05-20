# Project: project
# Created: 2025-05-20 08:31:49
# Integration: Standalone

# Collaborative Intelligence Configuration
This project is configured to use the CI (Collaborative Intelligence) system
in standalone mode, which provides full functionality without requiring the original
repository.

_CI.config('project_name', 'project')_
_CI.config('created_at', '2025-05-20 08:31:49')_
_CI.config('integration_type', 'standalone')_

## Available Agents
_CI.load_agents('Athena,ProjectArchitect')_

## Project Context
This project uses the standalone CI integration, which processes directives
directly within the CI tool. No external repository is required.

## Custom Instructions
- Focus on code quality and maintainability
- Follow established patterns in the codebase
- Add appropriate error handling for all edge cases
- Include helpful comments for complex sections
