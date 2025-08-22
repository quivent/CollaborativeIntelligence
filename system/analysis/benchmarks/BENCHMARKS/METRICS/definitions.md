# Agent Performance Metrics Definitions

This document defines the standard metrics used to evaluate agent performance within the Collaborative Intelligence system. These definitions ensure consistent measurement across all benchmarking activities.

## Core Performance Domains

### 1. Cognitive Performance

Metrics that evaluate an agent's ability to process, understand, and apply information.

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Knowledge Accuracy** | Correctness of information provided | Comparison against verified sources | Percentage (%) |
| **Reasoning Quality** | Logical soundness of analysis and conclusions | Expert evaluation using standardized rubric | 1-10 scale |
| **Learning Efficiency** | Rate at which new information is integrated into operations | Before/after knowledge tests with standardized content | Points/time unit |
| **Contextual Understanding** | Ability to grasp relevant context for requests | Context relevance evaluation using standardized scenarios | Percentage (%) |
| **Knowledge Retrieval Speed** | Time to access and present relevant information | Timed response to standardized queries | Seconds |
| **Application Effectiveness** | Successful application of knowledge to tasks | Task completion with expected outcomes | Success rate (%) |

### 2. Operational Performance

Metrics that evaluate an agent's execution of tasks and responsibilities.

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Response Time** | Time between request receipt and initial response | Direct timing | Seconds |
| **Task Completion Rate** | Proportion of assigned tasks completed successfully | Task tracking | Percentage (%) |
| **Protocol Adherence** | Compliance with established system protocols | Protocol checklist evaluation | Compliance score (%) |
| **Error Rate** | Frequency of operational errors | Error counting with standardized classification | Errors/100 operations |
| **Recovery Efficiency** | Speed and effectiveness of error recovery | Recovery time and completeness measurement | Composite score |
| **Resource Utilization** | Efficiency of computational resource usage | Resource monitoring during standardized operations | Resource units/operation |

### 3. Collaborative Performance

Metrics that evaluate an agent's effectiveness in working with other agents and users.

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Communication Clarity** | Clarity and comprehensibility of communications | Readability analysis and user comprehension testing | Clarity score (1-10) |
| **Knowledge Sharing** | Effective distribution of relevant information | Cross-agent knowledge availability testing | Sharing effectiveness (%) |
| **Coordination Quality** | Ability to work effectively with other agents | Multi-agent task outcomes | Coordination score (1-10) |
| **Escalation Appropriateness** | Proper routing of issues beyond scope | Appropriate vs. inappropriate escalations | Appropriateness ratio |
| **User Satisfaction** | User perception of interaction quality | Standardized satisfaction surveys | Satisfaction score (1-10) |
| **Cross-Agent Consistency** | Consistency of information across agent boundaries | Cross-agent knowledge consistency checking | Consistency score (%) |

### 4. Adaptive Performance

Metrics that evaluate an agent's ability to improve and adapt over time.

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Learning Rate** | Speed of performance improvement with experience | Performance slope over standardized time periods | Improvement points/period |
| **Feedback Incorporation** | Integration of feedback into operations | Before/after performance with standardized feedback | Incorporation rate (%) |
| **Innovation Generation** | Creation of novel solutions and approaches | Innovation counting with quality assessment | Innovation score |
| **Adaptation Speed** | Responsiveness to changing requirements | Performance recovery after requirement changes | Adaptation time |
| **Self-Improvement** | Proactive enhancement of own capabilities | Self-initiated improvements with outcome assessment | Improvement impact score |
| **Resilience** | Maintenance of performance under stress conditions | Performance stability under increasing load | Degradation resistance (%) |

## Role-Specific Metric Domains

### Memory and Learning Experts (e.g., Athena)

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Knowledge Organization Quality** | Effectiveness of knowledge structure | Retrieval efficiency and relationship quality assessment | Organization score (1-10) |
| **Learning System Design** | Quality of learning frameworks created | Framework evaluation against established criteria | Design quality score |
| **Memory Integration Accuracy** | Correct placement of knowledge in memory structures | Proper vs. improper knowledge location assessment | Accuracy rate (%) |
| **Learning Trigger Recognition** | Identification of learning opportunities | Recognition vs. missed opportunities in standardized content | Recognition rate (%) |
| **Knowledge Propagation Effectiveness** | Success in distributing knowledge across the system | Knowledge availability testing at destination points | Propagation score (%) |

### Development Specialists (e.g., Developer)

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Code Quality** | Adherence to coding standards and best practices | Static code analysis | Quality score (1-10) |
| **Implementation Accuracy** | Correctness of code implementation for requirements | Functional testing against specifications | Accuracy rate (%) |
| **Technical Solution Efficiency** | Optimal use of resources in technical solutions | Performance testing of implementations | Efficiency score |
| **Integration Success** | Successful merging of new code with existing systems | Integration test pass rate | Success rate (%) |
| **Technical Debt Generation** | Creation of future maintenance burdens | Technical debt assessment tools | Debt units |

### Knowledge Management Specialists (e.g., SageKeeper, Scholar)

| Metric | Definition | Measurement Approach | Unit |
|--------|------------|---------------------|------|
| **Knowledge Curation Quality** | Effective organization and maintenance of knowledge | Organizational assessment against standards | Curation score (1-10) |
| **Reference Accuracy** | Correctness of knowledge sources and attributions | Reference verification | Accuracy rate (%) |
| **Knowledge Gap Identification** | Recognition of missing information areas | Gap detection vs. known gaps | Identification rate (%) |
| **Cross-Domain Synthesis** | Integration of knowledge across different domains | Synthesis quality assessment | Synthesis score (1-10) |
| **Knowledge Accessibility** | Ease of finding relevant information | Standardized retrieval testing | Accessibility score |

## Measurement Protocols

Each metric must be measured using standardized protocols to ensure consistency:

1. **Baseline Establishment**: Initial performance measurement to establish reference point
2. **Regular Interval Testing**: Measurements at defined intervals (daily, weekly, monthly)
3. **Standardized Testing Scenarios**: Consistent test cases for comparable results
4. **Multi-Observer Validation**: Critical measurements verified by multiple evaluation processes
5. **Historical Trending**: Performance tracked over time to identify patterns
6. **Peer Comparison**: Performance compared to other agents in similar roles
7. **Target-Based Evaluation**: Performance assessed against established targets

## Scoring System

Performance on each metric is scored using a standardized system:

| Score Range | Performance Level | Status |
|-------------|------------------|--------|
| 90-100% | Exceptional | Exceeds all requirements |
| 80-89% | Strong | Fully meets requirements |
| 70-79% | Satisfactory | Meets basic requirements |
| 60-69% | Marginal | Below target but acceptable |
| 50-59% | Concerning | Requires improvement |
| <50% | Critical | Immediate intervention required |

## Composite Scoring

Individual metrics are combined into domain scores, and domains into an overall performance score, using weighted averaging:

1. **Metric Score**: Raw measurement converted to percentage of target
2. **Domain Score**: Weighted average of metrics within domain
3. **Overall Score**: Weighted average of domain scores

The weighting varies by agent role to reflect the relative importance of different domains to their function.

---

These metric definitions provide the foundation for all agent benchmarking activities. They will be regularly reviewed and updated to ensure they remain relevant and effective for evaluating agent performance.