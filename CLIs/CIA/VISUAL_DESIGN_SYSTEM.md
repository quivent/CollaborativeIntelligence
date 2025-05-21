# CIA CLI Visual Design System

This document defines the visual design system for the Collaborative Intelligence Adaption CLI (CIA), ensuring consistent user experience across all commands and output formats.

## Color Palette

CIA uses a carefully selected color palette that works well in both light and dark terminals:

### Primary Colors
- **Primary**: Cyan (`#06B6D4`) - Used for main commands and primary actions
- **Secondary**: Blue (`#3B82F6`) - Used for supporting information and secondary actions
- **Accent**: Green (`#10B981`) - Used for success indicators and confirmations

### Status Colors
- **Success**: Green (`#10B981`) - Operations completed successfully
- **Warning**: Yellow (`#F59E0B`) - Potential issues requiring attention
- **Error**: Red (`#EF4444`) - Errors and failures
- **Info**: Blue (`#3B82F6`) - Informational messages
- **Critical**: Magenta (`#8B5CF6`) - Critical system operations

### Monochrome Scale
- **White**: (`#FFFFFF`) - Main text on dark backgrounds
- **Light Gray**: (`#E5E7EB`) - Secondary text on dark backgrounds
- **Mid Gray**: (`#9CA3AF`) - Disabled text, comments
- **Dark Gray**: (`#4B5563`) - Subtle information on light backgrounds
- **Black**: (`#000000`) - Main text on light backgrounds

## Typography

### Text Styles
- **Headers**: Bold white/cyan for command categories and main sections
- **Subheaders**: Bold blue for subsections
- **Body Text**: Regular white/black (depending on terminal theme)
- **Metadata**: Light gray, slightly dimmed
- **Code/Paths**: Monospace formatting, often cyan for emphasis
- **User Input**: Bold to distinguish from system output

### Icons and Symbols
Use consistent visual symbols (emoji or text) across the CLI:

- **Success**: ✅ or `[OK]`
- **Error**: ❌ or `[ERROR]`
- **Warning**: ⚠️ or `[WARN]`
- **Info**: ℹ️ or `[INFO]`
- **Processing**: 🔄 or `[...]`
- **Waiting**: ⏳ or `[WAIT]`
- **Debug**: 🔍 or `[DEBUG]`
- **Critical**: 🚨 or `[CRITICAL]`

## Layout Components

### Command Headers
```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ COMMAND NAME: Purpose description  ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

### Status Banners
```
╔════════════════════════════════════╗
║ SUCCESS: Operation completed        ║
╚════════════════════════════════════╝
```

### Progress Indicators
```
Operation in progress [⣾⣽⣻⢿⡿⣟⣯⣷]
[####################] 100% Complete
```

### Data Tables
```
┌────────────────┬───────────────┬────────────┐
│ Column Header  │ Column Header │ Header     │
├────────────────┼───────────────┼────────────┤
│ Data cell      │ Data cell     │ Data cell  │
│ Data cell      │ Data cell     │ Data cell  │
└────────────────┴───────────────┴────────────┘
```

### File Trees
```
repo/
├── dir1/
│   ├── file1.rs
│   └── file2.rs
└── dir2/
    └── file3.rs
```

### Command Groups
```
AGENT COMMANDS:
  activate    Activate an agent
  deactivate  Deactivate an agent
  list        List available agents
```

## Output Patterns

### Welcome Message
```
🔷 Collaborative Intelligence Adaption CLI v1.0.0
Repository: CollaborativeIntelligence
Branch: main (✓ clean)
```

### Command Execution
```
→ Executing command: agent activate
  ├ Checking agent availability...  OK
  ├ Loading agent resources...      OK
  ├ Verifying permissions...        OK
  └ Activation complete             ✓
```

### Error Reporting
```
❌ ERROR: Could not activate agent
  └ Reason: Agent resources not found at expected path
    Location: /path/to/resources
    Suggestion: Check that the agent name is correct
```

### Help Output
```
USAGE:
    cia [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -v, --verbose    Enable verbose output
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    agent      Manage agents
    memory     Memory operations
    knowledge  Knowledge operations
    help       Display help for a specific command
```

## Animation and Interaction

### Spinners
Use consistent spinner styles for long-running operations:
```
⣾⣽⣻⢿⡿⣟⣯⣷
```

### Progress Bars
Use structured, colored progress bars:
```
[■■■■■■■■■■□□□□□□□□□□] 50%
```

### Interactive Prompts
```
? Select agent (use arrow keys):
  > Athena
    Topologist
    Developer
    Mnemosyne
```

## Implementation Guidelines

### In Rust Code
```rust
use colored::*;

// Headers
println!("{}", "AGENT MANAGEMENT".bright_cyan().bold());

// Success message
println!("{} {}", "✓".green(), "Agent activated successfully".green());

// Error message
eprintln!("{} {}: {}", 
    "✖".red(), 
    "ERROR".red().bold(), 
    "Agent activation failed".bright_red());

// Data organization
println!("  {} {}", "├".bright_black(), "Verification step".bright_blue());
```

### Output Structure
1. **Context**: Begin with the context (command being executed)
2. **Progress**: Show progress as command executes
3. **Results**: Display results clearly with appropriate colors
4. **Next Steps**: Include suggestions for next steps when applicable

## Accessibility Considerations

1. **Color Blindness**: Never rely solely on color to convey information
2. **High Contrast**: Maintain adequate contrast for readability
3. **Plain Mode**: Support a `--plain` flag to disable colors for compatibility
4. **Screen Readers**: Structure output for proper parsing by screen readers

## Intelligence Adaption Visual Identity

The CIA CLI visual design embodies the principles of Collaborative Intelligence Adaption:

- **Adaptive UI Elements**: Interface adapts based on context and user patterns
- **Collaborative Colors**: Color scheme promotes collaboration and shared understanding
- **Progressive Disclosure**: Information revealed progressively as needed
- **Continuous Feedback**: Visual feedback reflects ongoing intelligence adaption processes

## Usage Example

A complete example of CIA CLI output with proper formatting:

```
🔷 CIA: Collaborative Intelligence Adaption CLI v1.0.0

→ AGENT ACTIVATION
  └ Target: Athena

  ├ Checking agent existence...      ✓
  ├ Validating agent resources...    ✓
  ├ Checking permissions...          ✓
  ├ Loading memory segments...       ✓
  └ Activation complete

✅ SUCCESS: Athena activated successfully

CURRENT AGENT STATUS:
┌────────────┬───────────┬──────────────────────────┐
│ Agent      │ Status    │ Last Activity            │
├────────────┼───────────┼──────────────────────────┤
│ Athena     │ ACTIVE    │ Just now                 │
│ Topologist │ ACTIVE    │ Running                  │
│ Developer  │ INACTIVE  │ 2 hours ago              │
└────────────┴───────────┴──────────────────────────┘

RECOMMENDED NEXT STEPS:
1. Run 'cia agent verify' to ensure proper configuration
2. Review activation log at '/path/to/log'
```

---

By following this visual design system, the CIA CLI maintains consistency and user-friendliness across all commands and output formats while embodying the principles of Collaborative Intelligence Adaption.

*Last Updated: May 22, 2025*