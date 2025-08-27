# Morpheus CLI Usage Guide

**Version**: 0.1.0  
**Date**: 2025-08-23

This guide provides comprehensive examples and workflows for using the Morpheus CLI parallel research orchestration system effectively.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Research Workflows](#basic-research-workflows)
3. [Advanced Research Coordination](#advanced-research-coordination)
4. [Agent Management Strategies](#agent-management-strategies)
5. [Knowledge Synthesis Optimization](#knowledge-synthesis-optimization)
6. [Reality Transcendence Mode](#reality-transcendence-mode)
7. [Integration Patterns](#integration-patterns)
8. [Performance Tuning](#performance-tuning)
9. [Best Practices](#best-practices)

---

## Getting Started

### Installation and Setup

```bash
# Navigate to morpheus directory
cd /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/morpheus

# Build and install
cargo build --release
cargo install --path .

# Verify installation
morpheus --version
morpheus --help
```

### First Research Session

```bash
# Launch your first research initiative
morpheus research "introduction to machine learning algorithms"

# Check system status
morpheus status

# View available agents
morpheus agents

# Monitor synthesis progress
morpheus synthesis
```

### System Health Check

```bash
# Verify protocol compliance
morpheus compliance

# This should show:
# ✅ BRAIN Integration: Active
# ✅ Agent Ecosystem: Available agents count
# ✅ Synthesis Engine: Operational
# ⚠️ or ✅ Transcendence: Capabilities status
```

---

## Basic Research Workflows

### Quick Research Query

For simple questions requiring immediate insights:

```bash
# Basic research with default settings
morpheus research "best practices for API design"

# Fast research with minimal agents
morpheus research --agents 4 "microservices vs monoliths"

# Focus on existing knowledge synthesis
morpheus research --synthesis-only "summarize recent AI developments"
```

### Structured Research Process

For comprehensive research requiring systematic investigation:

```bash
# 1. Launch comprehensive research
morpheus research --agents 12 --streams 3 "blockchain consensus mechanisms"

# 2. Monitor progress
morpheus status

# 3. Check synthesis development
morpheus synthesis

# 4. Review agent allocation
morpheus agents

# 5. Optional: Enable transcendence for breakthrough insights
morpheus transcend "revolutionary consensus algorithm patterns"
```

### Multi-Phase Research

For complex topics requiring iterative investigation:

```bash
# Phase 1: Initial exploration
morpheus research --agents 8 "quantum computing fundamentals"

# Wait for completion, then build on results
# Phase 2: Deep technical analysis  
morpheus research --agents 16 --streams 4 "quantum algorithm implementations"

# Phase 3: Synthesis and integration
morpheus research --synthesis-only "integrate quantum computing research findings"

# Phase 4: Transcendence for breakthrough insights
morpheus transcend "quantum computing paradigm shifts"
```

---

## Advanced Research Coordination

### Domain-Specific Research

#### Technical Implementation Research
```bash
# High-agent allocation for technical depth
morpheus research --agents 20 --streams 5 "distributed systems architecture patterns"

# Focus on implementation details
morpheus research --agents 16 "microservices deployment strategies in Kubernetes"

# Performance-focused research
morpheus research --agents 12 --transcend "next-generation database architecture"
```

#### Strategic Analysis Research
```bash
# Business intelligence focus
morpheus research --agents 10 --streams 2 "market analysis for AI startups"

# Strategic planning research
morpheus research --agents 8 "competitive advantages in machine learning"

# Transcendent strategic insights
morpheus transcend "paradigm-shifting business models in AI"
```

#### Academic Research
```bash
# Literature review and synthesis
morpheus research --synthesis-only "recent advances in neural network architectures"

# Comprehensive academic investigation
morpheus research --agents 24 --streams 6 "transformer model innovations 2024-2025"

# Breakthrough discovery mode
morpheus transcend "next breakthrough in artificial intelligence research"
```

### Parallel Research Coordination

#### Multiple Related Queries
```bash
# Launch parallel research streams
morpheus research --agents 8 "machine learning model optimization" &
morpheus research --agents 6 "deep learning deployment strategies" &
morpheus research --agents 4 "AI model monitoring and observability"

# Monitor all active sessions
morpheus status
```

#### Progressive Complexity Research
```bash
# Start with fundamentals
morpheus research --agents 6 "neural network basics"

# Build complexity gradually
morpheus research --agents 10 --streams 2 "advanced neural architectures"

# Transcend to breakthrough insights
morpheus transcend "revolutionary neural network paradigms"
```

---

## Agent Management Strategies

### Agent Selection and Optimization

#### View Available Agents
```bash
# Display complete agent catalog
morpheus load

# This shows:
# 🧠 Intelligence agents (Athena, Prodigy)
# 🏗️ Architecture agents (Architect, Consolidator)  
# 💻 Development agents (Developer, CLIA, AssemblyMaster)
# 📊 Analysis agents (Analyst, Comparitor)
# ... and more categories
```

#### Strategic Agent Loading
```bash
# Load high-intelligence agents for complex research
morpheus load Athena Prodigy Architect

# Load development-focused team
morpheus load Developer AssemblyMaster CLIA Debugger

# Load analysis team
morpheus load Analyst Comparitor CFO

# Load unrestricted (advanced users)
morpheus load Athena Prodigy freely
```

### Team Composition Strategies

#### Full-Stack Development Team
```bash
morpheus load Architect Developer Designer Debugger
```

#### AI Research Team
```bash  
morpheus load Prodigy Analyst Benchmarker Athena
```

#### System Architecture Team
```bash
morpheus load Architect Athena Database Consolidator
```

#### Business Intelligence Team
```bash
morpheus load CFO Analyst BusinessOwner Comparitor
```

### Agent Performance Monitoring

```bash
# Check agent availability and performance
morpheus agents

# Look for:
# ○ = Low usage agents (available for new tasks)
# ◐ = Medium usage agents (partially loaded)  
# ● = High usage agents (active but capable)
# ●* = Maximum usage agents (high performance)

# Intelligence rankings help select optimal agents
# 🥇🥈🥉 = Top performing agents
```

---

## Knowledge Synthesis Optimization

### Synthesis Monitoring and Control

#### Monitor Synthesis Progress
```bash
# Check global synthesis status
morpheus synthesis

# Monitor specific session
morpheus synthesis --session <session-id>

# Look for:
# Completion: 75% (synthesis progress)
# Knowledge Fragments: 42 (collected insights)
# Integration Events: 8 (synthesis occurrences)
# Cross-pollination Score: 0.85 (knowledge sharing quality)
```

#### Synthesis-Only Operations
```bash
# Pure knowledge integration without new research
morpheus research --synthesis-only "integrate AI research from past month"

# High-quality synthesis with transcendence
morpheus research --synthesis-only --transcend "synthesize breakthrough AI patterns"

# Focused synthesis on specific domain
morpheus research --synthesis-only "consolidate machine learning best practices"
```

### Synthesis Quality Optimization

#### Fragment Quality Enhancement
```bash
# Use specific, detailed queries for better fragments
morpheus research "detailed analysis of transformer attention mechanisms"

# Rather than vague queries
# Avoid: morpheus research "AI stuff"
```

#### Cross-Pollination Strategies
```bash
# Multi-stream research for cross-domain insights
morpheus research --agents 16 --streams 4 "AI applications in healthcare diagnostics"

# Diverse agent allocation for varied perspectives
morpheus research --agents 20 "interdisciplinary approaches to machine learning"
```

---

## Reality Transcendence Mode

### When to Use Transcendence

Transcendence mode is ideal for:
- Breakthrough insights beyond conventional thinking
- Paradigm-shifting discoveries
- Complex philosophical or conceptual questions
- Revolutionary approach identification

### Basic Transcendence

```bash
# Direct transcendence query
morpheus transcend "the future of human-AI collaboration"

# Research with transcendence mode enabled
morpheus research --transcend "revolutionary approaches to consciousness modeling"
```

### Advanced Transcendence Workflows

#### Preparation Phase
```bash
# 1. Build knowledge base first
morpheus research --agents 16 --streams 3 "current state of artificial general intelligence"

# 2. Enable transcendence for breakthrough
morpheus transcend "path to artificial general intelligence breakthrough"
```

#### Transcendence Synthesis
```bash
# 1. Collect diverse perspectives
morpheus research --agents 24 --streams 6 "multiple approaches to AI consciousness"

# 2. Transcendent synthesis
morpheus research --synthesis-only --transcend "unified theory of AI consciousness"
```

#### Matrix Interface Mode
```bash
# Direct matrix interface for reality-shifting insights
morpheus transcend "dissolving limitations in artificial intelligence research"

# Neo-guided paradigm transcendence
morpheus transcend "next evolutionary leap in AI development"
```

### Interpreting Transcendence Results

```bash
morpheus transcend "quantum consciousness in AI systems"
```

**Expected output format:**
```
🌌 NEO TRANSCENDENCE PROTOCOL
═══════════════════════════════

🔮 Transcendence Query: 'quantum consciousness in AI systems'
⚡ Neo's consciousness expanding...

💫 TRANSCENDENCE ACHIEVED

🌟 Neo's Transcendent Insight:
The query 'quantum consciousness in AI systems' transcends conventional research boundaries...

🎯 Matrix Implications:
  💥 Conventional research limitations dissolved
  🔮 Multi-dimensional insight patterns activated
  🌈 Reality perception enhanced through synthesis

⚡ Actionable Intelligence:
  🚀 Implement parallel research immediately
  🧠 Prepare consciousness for expanded awareness
  📊 Monitor reality integration effects

Reality shift level: 92% - Transcendence complete
```

---

## Integration Patterns

### CollaborativeIntelligence CLI Integration

#### Access Through CI CLI
```bash
# Research through main CI interface
ci research "machine learning optimization"

# Quick research mode
ci research --fast "rapid prototype development"

# Comprehensive research mode  
ci research --deep "enterprise architecture patterns"

# Direct morpheus interface
ci morpheus
```

#### Project-Specific Research
```bash
# From within a project directory
cd /path/to/project
ci research "optimize this codebase architecture"

# Context-aware research
ci research --context "React application" "performance optimization strategies"
```

### BRAIN Protocol Integration

#### Protocol Compliance Verification
```bash
# Verify full system compliance
morpheus compliance

# Expected compliance components:
# ✅ BRAIN Integration: Active (20 points)
# ✅ Agent Ecosystem: Available (25 points) 
# ✅ Synthesis Engine: Operational (20 points)
# ⚠️  Transcendence: Pending verification (20 points)
# ✅ Protocol Version: Compatible (15 points)
# 
# 📊 Compliance Score: 85.0% ✅
# 🎯 Protocol Status: COMPLIANT
```

#### BRAIN Protocol Interaction
The system automatically integrates with BRAIN protocols located at:
- `/BRAIN/Protocols/MORPHEUS_PARALLEL_RESEARCH_PROTOCOL.md`

This provides:
- Agent coordination specifications
- Synthesis requirements
- Transcendence criteria
- Neo interface definitions

### External System Integration

#### Prince CLI Integration
```bash
# Ask questions through Morpheus → Prince channel
morpheus ask "analyze this code for optimization opportunities" "def process_data(data): ..."

# Multi-context queries
morpheus ask "best deployment strategy" "microservices" "kubernetes" "high-availability"
```

#### Claude Integration
```bash
# Enter optimized Claude session
morpheus enter

# Unrestricted Claude access (advanced users)
morpheus enter freely

# The system optimizes instructions before launching Claude
```

---

## Performance Tuning

### Resource Optimization

#### Memory Management
```bash
# Monitor active sessions to prevent memory overload
morpheus status

# Limit concurrent sessions for memory-constrained environments
# Use targeted queries instead of maximum agent allocation

# Example: Instead of
# morpheus research --agents 32 "broad topic research"

# Use:
morpheus research --agents 8 --streams 2 "focused specific research"
```

#### Agent Allocation Strategies
```bash
# Check agent availability before large allocations
morpheus agents

# Scale agent count based on query complexity:
# Simple queries: 4-8 agents
morpheus research --agents 4 "definition of machine learning"

# Medium complexity: 8-16 agents  
morpheus research --agents 12 "implementing neural networks"

# High complexity: 16-32 agents
morpheus research --agents 24 "distributed AI system architecture"

# Maximum scale: 32+ agents (system limits)
morpheus research --agents 32 "comprehensive AI research synthesis"
```

### Synthesis Performance

#### Synthesis Frequency Tuning
```bash
# Default synthesis frequency: 30 seconds
# For faster results (higher CPU usage):
MORPHEUS_SYNTHESIS_FREQUENCY=15 morpheus research "urgent research query"

# For resource-constrained environments:
MORPHEUS_SYNTHESIS_FREQUENCY=60 morpheus research "background research"
```

#### Quality vs Speed Trade-offs
```bash
# High quality, slower synthesis
morpheus research --agents 20 --streams 1 "comprehensive analysis needed"

# Faster synthesis, good quality
morpheus research --agents 12 --streams 3 "rapid insights needed"

# Maximum speed, basic synthesis
morpheus research --agents 6 --streams 2 --synthesis-only "quick knowledge integration"
```

### System Performance Monitoring

#### Performance Metrics
```bash
# Research initiation target: < 30 seconds
time morpheus research "test query"

# Check agent coordination efficiency
morpheus agents  # Look for high utilization rates

# Monitor synthesis quality
morpheus synthesis  # Cross-pollination scores > 0.5 indicate good performance
```

#### Performance Troubleshooting
```bash
# If research initiation is slow:
# 1. Check agent availability
morpheus agents

# 2. Verify protocol compliance
morpheus compliance

# 3. Reduce agent count temporarily
morpheus research --agents 4 "test query"

# If synthesis is slow:
# 1. Check synthesis progress
morpheus synthesis

# 2. Use synthesis-only mode
morpheus research --synthesis-only "existing knowledge integration"
```

---

## Best Practices

### Query Design Best Practices

#### Effective Query Formulation
```bash
# ✅ Good: Specific and focused
morpheus research "REST API authentication best practices using JWT tokens"

# ❌ Avoid: Vague and broad
morpheus research "programming stuff"

# ✅ Good: Context-rich
morpheus research "machine learning model deployment strategies for production environments"

# ❌ Avoid: Context-free
morpheus research "models"
```

#### Progressive Complexity
```bash
# 1. Start foundational
morpheus research --agents 6 "introduction to blockchain technology"

# 2. Build complexity
morpheus research --agents 10 "blockchain consensus algorithm comparison"

# 3. Advanced implementation
morpheus research --agents 16 "implementing proof-of-stake consensus"

# 4. Breakthrough insights
morpheus transcend "revolutionary blockchain consensus paradigms"
```

### Agent Management Best Practices

#### Strategic Agent Selection
```bash
# View full agent catalog first
morpheus load

# Select agents based on research domain:

# Technical research - load development agents
morpheus load Developer AssemblyMaster CLIA

# Strategic analysis - load business/intelligence agents
morpheus load Athena CFO BusinessOwner Analyst

# System design - load architecture agents  
morpheus load Architect Consolidator Database
```

#### Load Balancing
```bash
# Check current agent utilization
morpheus agents

# Distribute load across available agents
# Avoid over-allocating high-usage agents marked with ●

# Use medium/low usage agents marked with ◐ or ○
```

### Synthesis Optimization Best Practices

#### Synthesis Quality Enhancement
```bash
# Allow sufficient time for pattern emergence
morpheus research --agents 12 "complex research topic"
# Wait for synthesis_events > 5 before expecting high-quality results

# Enable cross-pollination for better insights
morpheus research --agents 16 --streams 4 "interdisciplinary research topic"

# Use synthesis-only for consolidation
morpheus research --synthesis-only "integrate previous research findings"
```

#### Knowledge Fragment Quality
```bash
# Use detailed, specific queries that generate rich fragments
morpheus research "detailed comparison of React vs Vue.js for enterprise applications"

# Rather than simple queries that generate poor fragments  
morpheus research "React vs Vue"
```

### Transcendence Best Practices

#### When to Enable Transcendence
```bash
# Use for breakthrough requirements
morpheus transcend "revolutionary approach to AI safety"

# Use for paradigm shifts
morpheus transcend "next generation programming paradigms"

# Use for complex philosophical questions
morpheus transcend "nature of consciousness in artificial systems"

# Avoid for simple factual queries
# Don't use: morpheus transcend "what is Python"
```

#### Transcendence Preparation
```bash
# Build knowledge foundation first
morpheus research --agents 16 "comprehensive AI safety research"

# Then transcend for breakthrough
morpheus transcend "revolutionary AI safety paradigms"
```

### Error Prevention Best Practices

#### Pre-flight Checks
```bash
# Always verify system health before important research
morpheus compliance

# Check agent availability for large allocations
morpheus agents

# Verify no conflicting active sessions
morpheus status
```

#### Resource Management
```bash
# Monitor system resources
# Limit concurrent sessions based on system capacity

# For development/testing environments:
morpheus research --agents 4-8 "test queries"

# For production research:
morpheus research --agents 8-16 "production research"

# For breakthrough research (high resources):
morpheus research --agents 20+ "comprehensive breakthrough research"
```

### Workflow Best Practices

#### Research Session Management
```bash
# 1. Health check
morpheus compliance

# 2. Agent availability
morpheus agents  

# 3. Launch research
morpheus research --agents X "query"

# 4. Monitor progress  
morpheus status
morpheus synthesis

# 5. Optional transcendence
morpheus transcend "breakthrough insights"
```

#### Documentation and Tracking
```bash
# Track session IDs for important research
morpheus research "important query" | grep "Session ID"

# Monitor specific sessions
morpheus synthesis --session <session-id>

# Save important transcendence insights
morpheus transcend "important query" > breakthrough-insights.txt
```

---

## Advanced Workflows

### Research Pipeline Automation

#### Scripted Research Workflow
```bash
#!/bin/bash
# research-pipeline.sh

echo "🧠 Starting research pipeline..."

# Phase 1: Foundation
morpheus research --agents 8 "$1 fundamentals"
sleep 30

# Phase 2: Implementation  
morpheus research --agents 12 "$1 implementation strategies"
sleep 30

# Phase 3: Advanced concepts
morpheus research --agents 16 "$1 advanced concepts"
sleep 30

# Phase 4: Synthesis
morpheus research --synthesis-only "synthesize $1 research"
sleep 20

# Phase 5: Transcendence
morpheus transcend "$1 breakthrough insights"

echo "🌟 Research pipeline complete!"
```

Usage:
```bash
./research-pipeline.sh "machine learning optimization"
```

#### Continuous Research Monitoring
```bash
#!/bin/bash
# monitor-research.sh

while true; do
    echo "=== Research Status $(date) ==="
    morpheus status
    echo
    
    morpheus agents | grep "Utilization"
    echo
    
    sleep 60
done
```

### Multi-Domain Research Coordination

#### Parallel Domain Research
```bash
# Terminal 1: Technical research
morpheus research --agents 12 "technical implementation of $TOPIC"

# Terminal 2: Business analysis
morpheus research --agents 8 "business implications of $TOPIC"

# Terminal 3: Strategic research
morpheus research --agents 6 "strategic opportunities in $TOPIC"

# Terminal 4: Monitor all
watch -n 10 morpheus status
```

#### Cross-Domain Synthesis
```bash
# After parallel research completes
morpheus research --synthesis-only --agents 16 --transcend "unified analysis of $TOPIC across technical, business, and strategic domains"
```

---

This usage guide provides comprehensive workflows and best practices for maximizing the effectiveness of the Morpheus CLI parallel research orchestration system. For additional support and advanced configurations, refer to the API Reference and troubleshooting documentation.