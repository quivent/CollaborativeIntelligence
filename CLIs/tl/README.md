# TL CLI: Teaching/Learning Command Line Interface

The TL CLI is the internal command-line tool for the CollaborativeIntelligence repository, focused on repository maintenance and development tasks.

## Purpose

Unlike the external CI CLI which is designed for project integration, the TL CLI provides specialized commands for internal repository maintenance:

- Agent management
- Memory operations
- Knowledge registry
- Debugging tools
- System optimization

## Installation

Build from source:

```bash
cd /path/to/CollaborativeIntelligence/CLIs/tl
cargo build --release
```

## Usage

### Agent Commands

```bash
# Activate an agent
tl agent activate Athena --directory /path/to/working/dir --intent "Task description"

# Create a new agent
tl agent create Analyzer --description "Code analysis specialist"

# Validate agent structure
tl agent validate Athena

# List available agents
tl agent list
```

### Memory Commands

```bash
# Load agent memory
tl memory load Athena --context "Optional context"

# Validate memory files
tl memory validate Athena

# Manage memory cache
tl memory cache clear
tl memory cache status

# Display memory statistics
tl memory stats
tl memory stats Athena
```

### Knowledge Commands

```bash
# Search the knowledge registry
tl knowledge search "query"
tl knowledge search --category "Architecture"
tl knowledge search --tag "optimization"

# Index knowledge content
tl knowledge index

# Distribute knowledge
tl knowledge distribute
```

### Debug Commands

```bash
# Trace command execution
tl debug trace "command" arg1 arg2

# Inspect system state
tl debug inspect component_name

# Profile performance
tl debug profile operation_name
```

### System Commands

```bash
# Optimize system performance
tl system optimize

# Generate system reports
tl system report
tl system report --report-type detailed
```

## Styling

The TL CLI uses consistent styling elements:

- Color-coded output (success, error, warning, info)
- Progress indicators
- Standardized formatting
- Agent response formatting

## Development

To extend the TL CLI, add new commands in the appropriate modules:

1. Define the command in the main command enum
2. Implement the command handler in the commands module
3. Update documentation

## Integration with CI CLI

The TL CLI is separate from the external CI CLI but shares common styling and utility libraries. This separation allows for independent development while maintaining a consistent user experience.

## Future Development

As repository needs evolve, the TL CLI will expand to include:

- Automated testing commands
- Performance benchmarking
- Advanced debugging tools
- Enhanced knowledge management