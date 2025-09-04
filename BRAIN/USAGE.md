# BRAIN System Usage Guide
**CollaborativeIntelligence Multi-Agent System**

## 📋 Quick Start

### Prerequisites
- Rust toolchain installed
- SQLite3 available
- Git repository initialized
- Environment variables configured

### Basic Setup
```bash
# Set environment variables
export CI_DB_PATH="/path/to/CollaborativeIntelligence/db"
export CI_REPO_PATH="/path/to/CollaborativeIntelligence"

# Build and install CLI tools
cd CollaborativeIntelligence/cli
cargo build --release
cargo install --path .

# Initialize system
ci init
ci build
```

## 🤖 Agent Management

### Discovering Available Agents
```bash
# List all available agents
ci agents

# Search for specific agents
ci agents --filter "database"
ci agents --filter "design"

# View agent details
ci agents --details Agent_Name
```

### Loading and Using Agents
```bash
# Load an agent for use
ci load Architect

# Load agent with specific memory components
ci load Developer --memory core,learning

# Load multiple agents for collaboration
ci load "Architect,Developer,Tester"
```

### Agent Information Commands
```bash
# Check agent status
ci status Agent_Name

# View agent capabilities
ci agents --capabilities Architect

# Show agent usage statistics
ci agents --stats
```

## 🧠 BRAIN Knowledge System

### Accessing BRAIN Knowledge
```bash
# Query BRAIN for specific topics
ci query "memory architecture"
ci query --domain "protocols" "agent activation"

# Browse BRAIN structure
ci brain --list
ci brain --structure
ci brain --recent
```

### Knowledge Submission
```bash
# Submit knowledge to BRAIN intake
ci submit knowledge.md --category "patterns"
ci submit --urgent "critical_fix.md"

# Check submission status
ci intake --status
ci intake --pending
```

### BRAIN Maintenance
```bash
# Update BRAIN indices
ci brain --reindex

# Validate BRAIN integrity
ci brain --validate

# Generate BRAIN statistics
ci brain --stats
```

## 💾 Database Operations

### Basic Database Commands
```bash
# View database status
ci-db status

# List all agents in database
ci-db agents list

# Add new agent to database
ci-db agents add Agent_Name /path/to/agent

# Update agent metadata
ci-db agents update Agent_Name --usage-count 150
```

### Learning System Management
```bash
# View learning entries
ci-db learning list --agent Agent_Name
ci-db learning list --recent 10

# Add learning entry
ci-db learning add Agent_Name "New insight about performance"

# Export learning data
ci-db learning export Agent_Name learning_data.json
```

### Session Management
```bash
# Create new session
ci-db sessions create "Memory Architecture Review"

# List sessions
ci-db sessions list
ci-db sessions list --agent Architect

# Export session data
ci-db sessions export session_id session_data.json
```

## 🔄 Project Lifecycle

### Project Initialization
```bash
# Initialize new project
ci init project_name

# Initialize with specific agents
ci init project_name --agents "Developer,Tester,Designer"

# Initialize with template
ci init project_name --template web_app
```

### Development Workflow
```bash
# Check project status
ci status

# Run project integration
ci integrate

# Fix common issues
ci fix --auto
ci fix --agent Debugger

# Deploy project
ci deploy --environment staging
```

### Version Control Integration
```bash
# Commit changes with agent assistance
ci commit --agent Developer

# Create release with documentation
ci release --version 1.2.0 --agent Documenter

# Generate change summary
ci changelog --agent Analyst
```

## 🔍 Analysis and Monitoring

### System Analysis
```bash
# Generate system report
ci analyze --full
ci analyze --performance
ci analyze --agents

# Matrix analysis
ci matrix --dimensions "performance,risk,priority"
ci matrix --export analysis_results.json
```

### Performance Monitoring
```bash
# System performance metrics
ci metrics --system
ci metrics --agents
ci metrics --knowledge

# Generate performance report
ci report --performance --output performance_report.md
```

### Health Checks
```bash
# Run system health check
ci health
ci health --detailed
ci health --fix

# Validate system components
ci validate --all
ci validate --agents
ci validate --brain
```

## 📊 Advanced Features

### Multi-Agent Collaboration
```bash
# Start collaborative session
ci collaborate "Architect,Developer,Designer" --project web_redesign

# Load collaboration template
ci collaborate --template "code_review" --agents "Developer,Tester"

# Monitor collaboration
ci collaborate --status
ci collaborate --export collaboration_log.md
```

### Knowledge Analytics
```bash
# Analyze knowledge patterns
ci analytics --knowledge
ci analytics --usage-patterns
ci analytics --learning-trends

# Generate insights report
ci insights --agents
ci insights --knowledge
ci insights --export insights_report.json
```

### Batch Operations
```bash
# Batch update agents
ci batch update --all-agents

# Batch process knowledge
ci batch process --intake-folder /path/to/submissions

# Batch generate reports
ci batch report --all-projects
```

## ⚙️ Configuration Management

### System Configuration
```bash
# View current configuration
ci config show

# Update configuration
ci config set agent_timeout 300
ci config set batch_size 50

# Reset to defaults
ci config reset
```

### Agent Configuration
```bash
# Configure agent-specific settings
ci config agent Agent_Name set memory_limit 100MB
ci config agent Agent_Name set learning_rate 0.1

# Export agent configuration
ci config agent Agent_Name export agent_config.json
```

### Environment Management
```bash
# Switch environments
ci env switch production
ci env switch development

# Create new environment
ci env create testing --copy-from development

# List environments
ci env list
```

## 🔧 Troubleshooting

### Common Issues

#### Agent Loading Problems
```bash
# Clear agent cache
ci cache clear --agents

# Rebuild agent index
ci agents --rebuild-index

# Force reload agent
ci load Agent_Name --force
```

#### Database Issues
```bash
# Repair database
ci-db repair

# Backup database
ci-db backup /path/to/backup

# Restore database
ci-db restore /path/to/backup
```

#### BRAIN Knowledge Issues
```bash
# Validate BRAIN structure
ci brain --validate --fix

# Rebuild BRAIN index
ci brain --reindex --force

# Check BRAIN consistency
ci brain --consistency-check
```

### Diagnostic Commands
```bash
# Generate diagnostic report
ci diagnose --full
ci diagnose --agents
ci diagnose --knowledge

# Test system components
ci test --all
ci test --agents
ci test --database
```

### Recovery Procedures
```bash
# Emergency system reset
ci reset --emergency

# Restore from backup
ci restore --backup /path/to/backup --confirm

# Rebuild system from scratch
ci rebuild --preserve-data
```

## 📈 Best Practices

### Efficient Agent Usage
1. **Load agents in batches** for related tasks
2. **Use specific agents** for specialized tasks
3. **Cache frequently used agents** for faster access
4. **Monitor agent performance** regularly

### Knowledge Management
1. **Submit knowledge regularly** to BRAIN intake
2. **Use descriptive categories** for submissions
3. **Validate submissions** before submitting
4. **Monitor intake processing** status

### System Maintenance
1. **Run health checks weekly**
2. **Update agents regularly**
3. **Clean up old sessions** periodically
4. **Backup database frequently**

### Performance Optimization
1. **Use batch operations** for multiple tasks
2. **Monitor system metrics** regularly
3. **Optimize configuration** based on usage patterns
4. **Keep BRAIN index updated**

## 🔐 Security Considerations

### Access Control
```bash
# Set agent permissions
ci permissions set Agent_Name read,write,execute

# View current permissions
ci permissions show

# Audit access logs
ci audit --access-logs
```

### Data Protection
```bash
# Enable encryption
ci security enable-encryption

# Backup with encryption
ci backup --encrypt --password

# Verify data integrity
ci verify --integrity
```

## 📚 Integration Examples

### IDE Integration
```bash
# Generate IDE configuration
ci generate --ide-config vscode
ci generate --ide-config intellij

# Setup development environment
ci setup --dev-env --editor vscode
```

### CI/CD Integration
```bash
# Generate CI/CD pipeline
ci generate --pipeline github-actions
ci generate --pipeline gitlab-ci

# Setup automated testing
ci setup --testing --framework pytest
```

### API Integration
```bash
# Start API server
ci serve --port 8080

# Generate API documentation
ci docs --api --format openapi

# Test API endpoints
ci test --api --endpoint /agents
```

## 📖 Advanced Workflows

### Research Workflow
```bash
# Start research session
ci research "AI memory architectures" --agents "Researcher,Analyst"

# Collect findings
ci research --collect --format report

# Synthesize results
ci research --synthesize --output research_summary.md
```

### Development Workflow
```bash
# Start development session
ci dev "new feature" --agents "Architect,Developer,Tester"

# Code review process
ci review --code --agents "Senior_Developer,Code_Reviewer"

# Quality assurance
ci qa --full --agents "Tester,Quality_Analyst"
```

### Documentation Workflow
```bash
# Generate documentation
ci docs --auto --agents "Documenter,Technical_Writer"

# Review documentation
ci docs --review --agents "Editor,Domain_Expert"

# Publish documentation
ci docs --publish --format web
```

---

## 📞 Support and Resources

### Getting Help
```bash
# View help for any command
ci help
ci help agents
ci help brain

# Check system status
ci status --verbose

# Generate support report
ci support --report
```

### Community Resources
- **Documentation**: `/BRAIN/Procedures/`
- **Examples**: `/BRAIN/Patterns/`
- **Troubleshooting**: `/BRAIN/Procedures/Troubleshooting/`
- **Best Practices**: `/BRAIN/Core/BestPractices.md`

---

*Last Updated: 2025-08-30*  
*Version: 1.0*  
*For system version compatibility, see SPECIFICATION.md*