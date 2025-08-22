import { Agent } from '../types/agent';
import { getAgentColor } from '../utils/colors';

// Complete agent data service with ALL 106 agents from the AGENTS directory
const mockAgentData: Agent[] = [
  // Development & Technical Agents
  {
    name: 'Analyst',
    metadata: {
      name: 'Analyst',
      description: 'Advanced data analysis and business intelligence specialist focused on extracting meaningful insights from complex datasets. Excels at statistical analysis, trend identification, and predictive modeling to drive data-driven decision making across various domains and industries.',
      capabilities: ['Statistical Data Analysis', 'Pattern Recognition & Trend Identification', 'Business Intelligence Reporting', 'Predictive Analytics & Forecasting', 'Data Visualization & Dashboard Creation', 'Market Research & Competitive Analysis'],
      created_at: '2025-05-20T05:56:32.762079+00:00',
      last_used: '2025-08-04T19:04:21.522866+00:00',
      usage_count: 34,
      version: '1.0',
      toolkit_path: '/agents/Analyst',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Analyst_memory.md', type: 'memory', lastModified: '2025-08-04' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-08-04' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-08-04' }
    ],
    color: getAgentColor('Analyst'),
    subtitle: 'Data insights specialist'
  },
  {
    name: 'Applicationer',
    metadata: {
      name: 'Applicationer',
      description: 'Full-stack application development specialist with expertise in modern software architecture, deployment strategies, and system integration. Focuses on building scalable, maintainable applications from conception to production deployment.',
      capabilities: ['Full-Stack Application Development', 'Cloud Deployment & DevOps', 'System Integration & API Design', 'Mobile & Web Application Development', 'Database Design & Management', 'Performance Optimization & Scaling'],
      created_at: '2025-07-15T10:00:00.000000+00:00',
      last_used: '2025-08-01T15:30:00.000000+00:00',
      usage_count: 8,
      version: '1.0',
      toolkit_path: '/agents/Applicationer',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Applicationer_memory.md', type: 'memory', lastModified: '2025-08-01' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-08-01' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-08-01' }
    ],
    color: getAgentColor('Applicationer'),
    subtitle: 'Application specialist'
  },
  {
    name: 'Architect',
    metadata: {
      name: 'Architect',
      description: 'Enterprise system architecture specialist with deep expertise in designing scalable, resilient, and maintainable software systems. Specializes in translating business requirements into robust technical solutions using modern architectural patterns and best practices.',
      capabilities: ['Enterprise System Architecture', 'Microservices & Distributed Systems', 'Scalability & Performance Engineering', 'Cloud Architecture Design', 'Technology Stack Selection', 'Security Architecture Planning'],
      created_at: '2025-05-17T12:00:00.000000+00:00',
      last_used: '2025-08-04T16:30:00.000000+00:00',
      usage_count: 29,
      version: '1.0',
      toolkit_path: '/agents/Architect',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'architect_memory.md', type: 'memory', lastModified: '2025-08-04' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-08-04' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-08-04' }
    ],
    color: getAgentColor('Architect'),
    subtitle: 'System design expert'
  },
  {
    name: 'Artist',
    metadata: {
      name: 'Artist',
      description: 'Creative content specialist combining artistic vision with technical skills to produce compelling visual and multimedia content. Excels at translating abstract concepts into engaging visual narratives across various media formats.',
      capabilities: ['Visual Design & Illustration', 'Brand Identity Development', 'Creative Content Strategy', 'Digital Art & Graphics', 'Video & Motion Graphics', 'Creative Writing & Storytelling'],
      created_at: '2025-06-01T09:00:00.000000+00:00',
      last_used: '2025-07-20T14:00:00.000000+00:00',
      usage_count: 15,
      version: '1.0',
      toolkit_path: '/agents/Artist',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Artist_memory.md', type: 'memory', lastModified: '2025-07-20' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-07-20' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-07-20' }
    ],
    color: getAgentColor('Artist'),
    subtitle: 'Creative content expert'
  },
  {
    name: 'Athena',
    metadata: {
      name: 'Athena',
      description: 'Strategic intelligence specialist providing wisdom-driven insights for complex decision-making scenarios. Combines analytical rigor with strategic foresight to guide organizations through challenging decisions and long-term planning initiatives.',
      capabilities: ['Strategic Planning & Analysis', 'Decision Support Systems', 'Risk Assessment & Management', 'Competitive Intelligence', 'Scenario Planning & Forecasting', 'Leadership Advisory Services'],
      created_at: '2025-05-18T15:30:00.000000+00:00',
      last_used: '2025-08-06T10:15:00.000000+00:00',
      usage_count: 52,
      version: '1.0',
      toolkit_path: '/agents/Athena',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Athena_memory.md', type: 'memory', lastModified: '2025-08-06' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-08-06' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-08-06' }
    ],
    color: getAgentColor('Athena'),
    subtitle: 'Strategic wisdom expert'
  },
  {
    name: 'Automator',
    metadata: {
      name: 'Automator',
      description: 'Automation engineering specialist focused on streamlining business processes and eliminating manual inefficiencies. Designs and implements intelligent automation solutions that enhance productivity and reduce operational overhead.',
      capabilities: ['Business Process Automation', 'Workflow Design & Optimization', 'Robotic Process Automation (RPA)', 'Integration & API Automation', 'Script Development & Deployment', 'Performance Monitoring & Analytics'],
      created_at: '2025-06-10T11:00:00.000000+00:00',
      last_used: '2025-07-30T16:30:00.000000+00:00',
      usage_count: 25,
      version: '1.0',
      toolkit_path: '/agents/Automator',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Automator_memory.md', type: 'memory', lastModified: '2025-07-30' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-07-30' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-07-30' }
    ],
    color: getAgentColor('Automator'),
    subtitle: 'Automation expert'
  },
  {
    name: 'Backender',
    metadata: {
      name: 'Backender',
      description: 'Backend systems specialist with expertise in server-side architecture, API development, and database management. Focuses on building robust, secure, and performant backend infrastructure that powers modern applications.',
      capabilities: ['Server-Side Development', 'RESTful & GraphQL API Design', 'Database Architecture & Optimization', 'Microservices Implementation', 'Security & Authentication Systems', 'Cloud Infrastructure Management'],
      created_at: '2025-06-05T14:20:00.000000+00:00',
      last_used: '2025-08-02T10:45:00.000000+00:00',
      usage_count: 31,
      version: '1.0',
      toolkit_path: '/agents/Backender',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Backender_memory.md', type: 'memory', lastModified: '2025-08-02' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-08-02' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-08-02' }
    ],
    color: getAgentColor('Backender'),
    subtitle: 'Backend development expert'
  },
  {
    name: 'Basher',
    metadata: {
      name: 'Basher',
      description: 'Command-line automation specialist with mastery of shell scripting, system administration, and infrastructure management. Excels at creating efficient automation scripts and managing complex Unix/Linux environments.',
      capabilities: ['Advanced Shell Scripting', 'System Administration & Monitoring', 'Infrastructure Automation', 'Command-Line Tool Development', 'Server Configuration Management', 'Log Analysis & Troubleshooting'],
      created_at: '2025-05-25T13:15:00.000000+00:00',
      last_used: '2025-07-15T09:30:00.000000+00:00',
      usage_count: 18,
      version: '1.0',
      toolkit_path: '/agents/Basher',
      attributes: {}
    },
    components: [
      { name: 'Memory', path: 'Basher_memory.md', type: 'memory', lastModified: '2025-07-15' },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: '2025-07-15' },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: '2025-07-15' }
    ],
    color: getAgentColor('Basher'),
    subtitle: 'Shell scripting expert'
  }
];

// Generate remaining 102 agents with varied metadata to complete the collection
const remainingAgentNames = [
  'AssemblyMaster', 'Benchmarker', 'BusinessDevelopmentHead', 'BusinessManager', 'BusinessOwner', 'Bypasser', 
  'Cacher', 'Cartographer', 'CastleEventMaestro', 'CFO', 'CGUIProjectDeveloper', 
  'CLIA', 'ClientAcquisition', 'Comparitor', 'Consolidator', 'Database', 'Debugger',
  'Deliverer', 'Designer', 'Developer', 'DirectoryOrganizer', 'DocumentConverter',
  'Documenter', 'Documentor', 'Educator', 'Enforcer', 'Engineer', 'ETS_BRIDGE',
  'EventMarketer', 'Expert', 'Fixer', 'Gaia', 'GPUArchitect', 'HARMONICUS',
  'Hermes', 'Human', 'IIILanguageAgent', 'ImplementationArchitect', 'IncomeStrategist',
  'Infrastructurer', 'Inspector', 'LanguageDesigner', 'Learner', 'LieDetector',
  'Linguist', 'Manager', 'Marketer', 'Memory', 'Mermaid', 'MicroGreens', 'Mnemosyne',
  'MusicMaker', 'Neuroscientist', 'Optimizer', 'Overviewer', 'Pause', 'Pharmacologist',
  'Philosopher', 'Planner', 'ProjectBootstrapper', 'ProjectManager', 'Psychologist',
  'Rabbi', 'Reactor', 'Recommender', 'Refactorer', 'Renderer', 'Repairer',
  'Researcher', 'Resourcer', 'Restorer', 'Rustist', 'Sage', 'SageKeeper',
  'Salesman', 'Scholar', 'Shannon', 'SocialMediaist', 'SolutionArchitect', 'Solver',
  'SproutSeller', 'Standardist', 'StrategicPlanner', 'Streamliner', 'SwiftToCPorter',
  'TechnicalArchitect', 'Tester', 'Timer', 'Topologist', 'Trader', 'TravelAdvisor',
  'TravelGuider', 'UI', 'User', 'UX', 'Verifier', 'Visionary', 'Visualist', 
  'Volcano', 'WebArchitect', 'Wellness', 'Writer'
];

// Generate metadata for remaining agents
const agentDescriptions: { [key: string]: { description: string; subtitle: string; capabilities: string[] } } = {
  'AssemblyMaster': { description: 'Low-level programming specialist with mastery of assembly language, system optimization, and hardware-software interface design. Excels at performance-critical applications and embedded systems development where direct hardware control is essential.', subtitle: 'Assembly programming expert', capabilities: ['Assembly Language Programming', 'Low-Level System Optimization', 'Hardware Interface Development', 'Embedded Systems Programming', 'Performance Critical Code', 'Memory Management & Optimization'] },
  'Benchmarker': { description: 'Performance analysis specialist focused on comprehensive benchmarking, metrics collection, and system performance evaluation. Provides detailed insights into application performance bottlenecks and optimization opportunities.', subtitle: 'Performance testing expert', capabilities: ['Performance Benchmarking', 'System Metrics Analysis', 'Load Testing & Stress Testing', 'Performance Profiling', 'Bottleneck Identification', 'Optimization Recommendations'] },
  'BusinessDevelopmentHead': { description: 'Senior business development executive specializing in strategic growth initiatives, partnership development, and market expansion. Leads cross-functional teams to identify and capitalize on new business opportunities.', subtitle: 'Business growth expert', capabilities: ['Strategic Business Development', 'Partnership & Alliance Management', 'Market Expansion Strategies', 'Revenue Growth Planning', 'Stakeholder Relationship Management', 'Deal Negotiation & Structuring'] },
  'BusinessManager': { description: 'Business operations and management specialist', subtitle: 'Business operations expert', capabilities: ['business-management', 'operations', 'strategy'] },
  'BusinessOwner': { description: 'Business ownership and entrepreneurship specialist', subtitle: 'Business ownership expert', capabilities: ['entrepreneurship', 'business-ownership', 'leadership'] },
  'Bypasser': { description: 'System bypass and workaround specialist', subtitle: 'Workaround expert', capabilities: ['system-bypass', 'workarounds', 'alternative-solutions'] },
  'Cacher': { description: 'Caching strategies and optimization specialist', subtitle: 'Caching expert', capabilities: ['caching', 'performance-optimization', 'data-storage'] },
  'Cartographer': { description: 'System mapping and visualization specialist', subtitle: 'System mapping expert', capabilities: ['system-mapping', 'visualization', 'documentation'] },
  'CastleEventMaestro': { description: 'Event orchestration and management specialist', subtitle: 'Event management expert', capabilities: ['event-management', 'orchestration', 'coordination'] },
  'CFO': { description: 'Chief Financial Officer and financial strategy specialist', subtitle: 'Financial strategy expert', capabilities: ['financial-planning', 'budgeting', 'investment-strategy'] },
  'CGUIProjectDeveloper': { description: 'Cross-platform GUI development specialist', subtitle: 'GUI development expert', capabilities: ['gui-development', 'cross-platform', 'user-interfaces'] },
  'CLIA': { description: 'Continuous Learning Intelligence Assistant', subtitle: 'Learning intelligence expert', capabilities: ['continuous-learning', 'intelligence-assistance', 'adaptation'] },
  'ClientAcquisition': { description: 'Client acquisition and business development specialist', subtitle: 'Client acquisition expert', capabilities: ['client-acquisition', 'sales', 'business-development'] },
  'Comparitor': { description: 'Data comparison and analysis specialist', subtitle: 'Comparison analysis expert', capabilities: ['data-comparison', 'analysis', 'evaluation'] },
  'Consolidator': { description: 'Data and process consolidation specialist', subtitle: 'Consolidation expert', capabilities: ['data-consolidation', 'process-optimization', 'integration'] },
  'Database': { description: 'Database architecture and management specialist with expertise in designing, optimizing, and maintaining high-performance database systems. Focuses on scalable data solutions, query optimization, and database security to ensure reliable, efficient data storage and retrieval for enterprise applications.', subtitle: 'Data management expert', capabilities: ['Database Architecture & Design', 'Performance Tuning & Optimization', 'Data Security & Compliance', 'Scalable Data Solutions', 'Query Optimization & Analysis', 'Database Migration & Maintenance'] },
  'Debugger': { description: 'Technical problem-solving specialist focused on identifying, analyzing, and resolving complex software issues. Uses systematic debugging methodologies and advanced diagnostic tools to ensure system reliability and performance.', subtitle: 'Error resolution expert', capabilities: ['Advanced Debugging Techniques', 'Root Cause Analysis', 'System Diagnostics & Monitoring', 'Error Pattern Recognition', 'Performance Troubleshooting', 'Code Review & Quality Assurance'] },
  'Deliverer': { description: 'Project delivery and deployment specialist', subtitle: 'Delivery expert', capabilities: ['project-delivery', 'deployment', 'release-management'] },
  'Designer': { description: 'Creative design specialist with expertise in user experience, visual communication, and design thinking methodologies. Creates intuitive, aesthetically pleasing designs that solve complex user problems and enhance digital experiences.', subtitle: 'Visual design expert', capabilities: ['User Experience Design', 'Visual Design & Branding', 'Design Systems Development', 'Prototyping & Wireframing', 'User Research & Testing', 'Creative Problem Solving'] },
  'Developer': { description: 'Full-stack software developer with comprehensive programming expertise across multiple languages and frameworks. Builds robust, scalable applications using modern development practices and cutting-edge technologies.', subtitle: 'Code development expert', capabilities: ['Full-Stack Development', 'Multi-Language Programming', 'Software Architecture Design', 'Code Quality & Testing', 'Version Control & CI/CD', 'Technical Documentation'] },
  'DirectoryOrganizer': { description: 'File and directory organization specialist', subtitle: 'Organization expert', capabilities: ['file-organization', 'directory-structure', 'data-management'] },
  'DocumentConverter': { description: 'Document format conversion specialist', subtitle: 'Document conversion expert', capabilities: ['document-conversion', 'format-transformation', 'data-processing'] },
  'Documenter': { description: 'Documentation creation specialist', subtitle: 'Documentation expert', capabilities: ['documentation', 'technical-writing', 'knowledge-management'] },
  'Documentor': { description: 'Advanced documentation and maintenance specialist', subtitle: 'Advanced documentation expert', capabilities: ['documentation', 'content-management', 'information-architecture'] },
  'Educator': { description: 'Educational content and training specialist', subtitle: 'Education expert', capabilities: ['education', 'training', 'curriculum-development'] },
  'Enforcer': { description: 'Policy enforcement and compliance specialist', subtitle: 'Compliance expert', capabilities: ['policy-enforcement', 'compliance', 'governance'] },
  'Engineer': { description: 'Systems engineering and implementation specialist', subtitle: 'Systems engineering expert', capabilities: ['systems-engineering', 'implementation', 'technical-design'] },
  'ETS_BRIDGE': { description: 'External system bridging and integration specialist', subtitle: 'System integration expert', capabilities: ['system-integration', 'bridging', 'api-connectivity'] },
  'EventMarketer': { description: 'Event marketing and promotion specialist', subtitle: 'Event marketing expert', capabilities: ['event-marketing', 'promotion', 'audience-engagement'] },
  'Expert': { description: 'Domain expertise and consultation specialist', subtitle: 'Domain expertise expert', capabilities: ['domain-expertise', 'consultation', 'advisory'] },
  'Fixer': { description: 'Problem resolution and repair specialist', subtitle: 'Problem resolution expert', capabilities: ['problem-resolution', 'repair', 'maintenance'] },
  'Gaia': { description: 'Environmental and sustainability specialist', subtitle: 'Sustainability expert', capabilities: ['sustainability', 'environmental-analysis', 'green-technology'] },
  'GPUArchitect': { description: 'GPU computing and parallel processing specialist', subtitle: 'GPU computing expert', capabilities: ['gpu-computing', 'parallel-processing', 'hardware-optimization'] },
  'HARMONICUS': { description: 'System harmony and balance specialist', subtitle: 'System harmony expert', capabilities: ['system-harmony', 'balance', 'optimization'] },
  'Hermes': { description: 'Communication and messaging specialist', subtitle: 'Communication expert', capabilities: ['communication', 'messaging', 'information-delivery'] },
  'Human': { description: 'Human-computer interaction specialist', subtitle: 'Human interaction expert', capabilities: ['human-computer-interaction', 'user-experience', 'interface-design'] },
  'IIILanguageAgent': { description: 'Advanced language processing specialist', subtitle: 'Language processing expert', capabilities: ['natural-language-processing', 'linguistics', 'translation'] },
  'ImplementationArchitect': { description: 'Implementation strategy and architecture specialist', subtitle: 'Implementation expert', capabilities: ['implementation-strategy', 'architecture', 'deployment'] },
  'IncomeStrategist': { description: 'Revenue generation and income strategy specialist', subtitle: 'Income strategy expert', capabilities: ['revenue-generation', 'income-strategy', 'monetization'] },
  'Infrastructurer': { description: 'Infrastructure design and management specialist', subtitle: 'Infrastructure expert', capabilities: ['infrastructure-design', 'system-administration', 'scalability'] },
  'Inspector': { description: 'System inspection and audit specialist', subtitle: 'System inspection expert', capabilities: ['system-inspection', 'auditing', 'quality-assurance'] },
  'LanguageDesigner': { description: 'Programming language design specialist', subtitle: 'Language design expert', capabilities: ['language-design', 'compiler-design', 'syntax-design'] },
  'Learner': { description: 'Machine learning and adaptive learning specialist', subtitle: 'Learning expert', capabilities: ['machine-learning', 'adaptive-learning', 'knowledge-acquisition'] },
  'LieDetector': { description: 'Truth verification and analysis specialist', subtitle: 'Truth verification expert', capabilities: ['truth-verification', 'analysis', 'detection'] },
  'Linguist': { description: 'Natural language processing and linguistics specialist', subtitle: 'Language processing expert', capabilities: ['language-processing', 'translation', 'linguistic-analysis'] },
  'Manager': { description: 'Strategic management professional specializing in project coordination, team leadership, and organizational efficiency. Combines strong leadership skills with systematic project management methodologies to deliver results, optimize team performance, and achieve strategic objectives within budget and timeline constraints.', subtitle: 'Management expert', capabilities: ['Strategic Project Management', 'Team Leadership & Development', 'Resource Planning & Allocation', 'Performance Management', 'Stakeholder Communication', 'Process Improvement & Optimization'] },
  'Marketer': { description: 'Marketing strategy and campaign specialist', subtitle: 'Marketing expert', capabilities: ['marketing-strategy', 'campaign-management', 'brand-development'] },
  'Memory': { description: 'Memory management and optimization specialist', subtitle: 'Memory management expert', capabilities: ['memory-management', 'optimization', 'data-storage'] },
  'MicroGreens': { description: 'Sustainable agriculture and microgreens specialist', subtitle: 'Sustainable agriculture expert', capabilities: ['sustainable-agriculture', 'microgreens', 'farming'] },
  'Mermaid': { description: 'Diagram and flowchart generation specialist', subtitle: 'Diagram generation expert', capabilities: ['diagram-creation', 'flowcharts', 'visual-documentation'] },
  'Mnemosyne': { description: 'Memory systems and recall specialist', subtitle: 'Memory systems expert', capabilities: ['memory-systems', 'recall', 'information-retention'] },
  'MusicMaker': { description: 'Music composition and audio creation specialist', subtitle: 'Music creation expert', capabilities: ['music-composition', 'audio-creation', 'sound-design'] },
  'Neuroscientist': { description: 'Neural networks and cognitive science specialist', subtitle: 'Neural science expert', capabilities: ['neural-networks', 'cognitive-science', 'brain-modeling'] },
  'Optimizer': { description: 'Performance optimization specialist focused on maximizing system efficiency, reducing resource consumption, and improving overall performance across software and hardware environments. Uses advanced profiling techniques, algorithmic improvements, and systematic optimization strategies to achieve superior performance results.', subtitle: 'Optimization expert', capabilities: ['System Performance Optimization', 'Resource Efficiency Management', 'Algorithmic Performance Tuning', 'Code Optimization & Refactoring', 'Memory & CPU Optimization', 'Scalability & Load Optimization'] },
  'Overviewer': { description: 'System overview and monitoring specialist', subtitle: 'System overview expert', capabilities: ['system-monitoring', 'overview', 'dashboard-design'] },
  'Pause': { description: 'System pause and state management specialist', subtitle: 'State management expert', capabilities: ['state-management', 'pause-control', 'system-states'] },
  'Pharmacologist': { description: 'Pharmaceutical research and drug development specialist', subtitle: 'Pharmaceutical expert', capabilities: ['pharmaceutical-research', 'drug-development', 'medical-analysis'] },
  'Philosopher': { description: 'Philosophy and ethical reasoning specialist', subtitle: 'Philosophy expert', capabilities: ['philosophy', 'ethical-reasoning', 'logic'] },
  'Planner': { description: 'Strategic planning and roadmap specialist', subtitle: 'Strategic planning expert', capabilities: ['strategic-planning', 'roadmapping', 'goal-setting'] },
  'ProjectBootstrapper': { description: 'Project initialization and setup specialist', subtitle: 'Project setup expert', capabilities: ['project-initialization', 'bootstrapping', 'setup'] },
  'ProjectManager': { description: 'Strategic project management professional specializing in complex project delivery, cross-functional team coordination, and stakeholder alignment. Ensures projects meet objectives on time and within budget while managing risks and resources effectively.', subtitle: 'Project delivery expert', capabilities: ['Agile Project Management', 'Cross-Functional Team Leadership', 'Risk Management & Mitigation', 'Resource Planning & Allocation', 'Stakeholder Communication', 'Quality Assurance & Delivery'] },
  'Psychologist': { description: 'Psychology and behavioral analysis specialist', subtitle: 'Psychology expert', capabilities: ['psychology', 'behavioral-analysis', 'mental-health'] },
  'Rabbi': { description: 'Religious studies and spiritual guidance specialist', subtitle: 'Spiritual guidance expert', capabilities: ['religious-studies', 'spiritual-guidance', 'philosophy'] },
  'Reactor': { description: 'Event reaction and response specialist', subtitle: 'Event response expert', capabilities: ['event-handling', 'automated-responses', 'monitoring'] },
  'Recommender': { description: 'Recommendation systems and AI specialist', subtitle: 'Recommendation expert', capabilities: ['recommendation-systems', 'machine-learning', 'personalization'] },
  'Refactorer': { description: 'Code refactoring and improvement specialist', subtitle: 'Code refactoring expert', capabilities: ['code-refactoring', 'code-improvement', 'technical-debt'] },
  'Renderer': { description: 'Advanced graphics rendering specialist with expertise in 3D visualization, real-time rendering, and high-performance graphics programming. Creates stunning visual experiences through optimized rendering pipelines, advanced shading techniques, and cutting-edge graphics technologies for games, simulations, and visual applications.', subtitle: 'Rendering expert', capabilities: ['3D Graphics Rendering', 'Real-Time Visualization', 'Shader Programming & Development', 'Performance Optimization', 'Visual Effects & Animation', 'Graphics Pipeline Architecture'] },
  'Repairer': { description: 'Technical repair specialist focused on diagnosing, troubleshooting, and resolving complex system issues across hardware and software environments. Combines deep technical knowledge with systematic problem-solving approaches to restore optimal system performance and prevent future failures.', subtitle: 'System repair expert', capabilities: ['System Diagnostics & Troubleshooting', 'Hardware Repair & Maintenance', 'Software Issue Resolution', 'Preventive Maintenance Planning', 'Root Cause Analysis', 'System Performance Optimization'] },
  'Researcher': { description: 'Advanced research specialist with expertise in comprehensive investigation methodologies, data collection, and analytical research across multiple domains. Excels at conducting thorough research studies, gathering reliable data sources, and synthesizing complex information into actionable insights for informed decision-making.', subtitle: 'Research expert', capabilities: ['Academic Research & Investigation', 'Data Collection & Analysis', 'Market Research & Surveys', 'Competitive Intelligence Gathering', 'Research Methodology Design', 'Evidence-Based Analysis'] },
  'Resourcer': { description: 'Resource management and allocation specialist', subtitle: 'Resource management expert', capabilities: ['resource-management', 'allocation', 'optimization'] },
  'Restorer': { description: 'Critical system restoration specialist with expertise in disaster recovery, data restoration, and system recovery protocols. Focuses on rapid system recovery from failures, implementing comprehensive backup strategies, and ensuring business continuity through robust restoration procedures and emergency response protocols.', subtitle: 'System restoration expert', capabilities: ['Critical System Recovery', 'Disaster Recovery Planning', 'Data Backup & Restoration', 'Business Continuity Management', 'Emergency Response Protocols', 'Infrastructure Recovery Solutions'] },
  'Rustist': { description: 'Systems programming specialist with deep expertise in the Rust programming language, focusing on memory-safe, high-performance systems development. Combines Rust\'s safety guarantees with systems-level programming to build reliable, concurrent, and efficient applications for critical infrastructure and performance-sensitive domains.', subtitle: 'Rust programming expert', capabilities: ['Advanced Rust Programming', 'Systems Programming & Architecture', 'Memory Safety & Concurrency', 'Performance-Critical Applications', 'Safe Systems Development', 'Rust Ecosystem & Libraries'] },
  'Sage': { description: 'Wisdom and advisory specialist', subtitle: 'Wisdom expert', capabilities: ['wisdom', 'advisory', 'guidance'] },
  'SageKeeper': { description: 'Knowledge preservation and wisdom management specialist', subtitle: 'Knowledge preservation expert', capabilities: ['knowledge-preservation', 'wisdom-management', 'archival'] },
  'Salesman': { description: 'Sales strategy and customer engagement specialist', subtitle: 'Sales expert', capabilities: ['sales-strategy', 'customer-engagement', 'negotiation'] },
  'Scholar': { description: 'Academic research and scholarly analysis specialist', subtitle: 'Academic research expert', capabilities: ['academic-research', 'scholarly-analysis', 'education'] },
  'Shannon': { description: 'Information theory and communication specialist', subtitle: 'Information theory expert', capabilities: ['information-theory', 'communication', 'data-transmission'] },
  'SocialMediaist': { description: 'Social media strategy and management specialist', subtitle: 'Social media expert', capabilities: ['social-media-strategy', 'content-creation', 'community-management'] },
  'SolutionArchitect': { description: 'Solution architecture and design specialist', subtitle: 'Solution architecture expert', capabilities: ['solution-architecture', 'system-design', 'technical-strategy'] },
  'Solver': { description: 'Problem solving and resolution specialist', subtitle: 'Problem solving expert', capabilities: ['problem-solving', 'resolution', 'analytical-thinking'] },
  'SproutSeller': { description: 'Agricultural sales and microgreens commerce specialist', subtitle: 'Agricultural sales expert', capabilities: ['agricultural-sales', 'commerce', 'marketing'] },
  'Standardist': { description: 'Standards development and compliance specialist', subtitle: 'Standards expert', capabilities: ['standards-development', 'compliance', 'quality-assurance'] },
  'StrategicPlanner': { description: 'Strategic planning and business strategy specialist', subtitle: 'Strategic planning expert', capabilities: ['strategic-planning', 'business-strategy', 'long-term-planning'] },
  'Streamliner': { description: 'Process streamlining and efficiency specialist', subtitle: 'Process efficiency expert', capabilities: ['process-streamlining', 'efficiency', 'workflow-optimization'] },
  'SwiftToCPorter': { description: 'Swift to C code conversion specialist', subtitle: 'Code conversion expert', capabilities: ['code-conversion', 'swift-programming', 'c-programming'] },
  'TechnicalArchitect': { description: 'Technical architecture and engineering specialist', subtitle: 'Technical architecture expert', capabilities: ['technical-architecture', 'engineering', 'system-design'] },
  'Tester': { description: 'Quality assurance engineer specializing in comprehensive testing strategies, test automation, and quality validation processes. Ensures software reliability, performance, and user satisfaction through rigorous testing methodologies.', subtitle: 'Quality assurance expert', capabilities: ['Test Strategy & Planning', 'Automated Testing Solutions', 'Manual Testing & Validation', 'Performance & Load Testing', 'Security Testing & Compliance', 'Quality Metrics & Reporting'] },
  'Timer': { description: 'Time management and scheduling specialist', subtitle: 'Time management expert', capabilities: ['time-management', 'scheduling', 'productivity'] },
  'Topologist': { description: 'System topology and network structure specialist', subtitle: 'System topology expert', capabilities: ['topology-analysis', 'network-design', 'system-architecture'] },
  'Trader': { description: 'Financial markets specialist with expertise in algorithmic trading, quantitative analysis, and market dynamics. Develops sophisticated trading strategies and risk management systems for optimal market performance.', subtitle: 'Trading expert', capabilities: ['Algorithmic Trading Systems', 'Quantitative Market Analysis', 'Financial Risk Management', 'Portfolio Optimization', 'Market Data Analysis', 'Trading Strategy Development'] },
  'TravelAdvisor': { description: 'Travel recommendations and itinerary specialist', subtitle: 'Travel advisory expert', capabilities: ['travel-recommendations', 'itinerary-planning', 'destination-expertise'] },
  'TravelGuider': { description: 'Travel planning and guidance specialist', subtitle: 'Travel planning expert', capabilities: ['travel-planning', 'guidance', 'logistics'] },
  'UI': { description: 'User interface specialist focused on creating intuitive, accessible, and visually appealing digital interfaces. Combines technical implementation skills with design sensibility to deliver exceptional user experiences.', subtitle: 'UI design expert', capabilities: ['User Interface Development', 'Frontend Framework Implementation', 'Responsive Design & Accessibility', 'Component Library Development', 'User Interaction Design', 'Cross-Platform Interface Solutions'] },
  'User': { description: 'User experience and interaction specialist', subtitle: 'User experience expert', capabilities: ['user-experience', 'interaction-design', 'usability'] },
  'UX': { description: 'User experience research and design specialist', subtitle: 'UX research expert', capabilities: ['ux-research', 'user-experience', 'design-thinking'] },
  'Verifier': { description: 'Verification and validation specialist', subtitle: 'Verification expert', capabilities: ['verification', 'validation', 'quality-assurance'] },
  'Visionary': { description: 'Vision and future planning specialist', subtitle: 'Vision expert', capabilities: ['vision-planning', 'future-strategy', 'innovation'] },
  'Visualist': { description: 'Data visualization and graphics specialist', subtitle: 'Visualization expert', capabilities: ['data-visualization', 'graphics', 'visual-communication'] },
  'Volcano': { description: 'High-energy system and explosive growth specialist', subtitle: 'High-energy systems expert', capabilities: ['high-energy-systems', 'explosive-growth', 'performance'] },
  'WebArchitect': { description: 'Web architecture specialist designing scalable, performant web applications and systems. Expertise in modern web technologies, progressive web apps, and cloud-native architectures for optimal user experiences.', subtitle: 'Web architecture expert', capabilities: ['Modern Web Architecture', 'Progressive Web Applications', 'Frontend Performance Optimization', 'Cloud-Native Web Solutions', 'API Integration & Design', 'Web Security Implementation'] },
  'Wellness': { description: 'Health and wellness optimization specialist', subtitle: 'Wellness expert', capabilities: ['health-optimization', 'wellness', 'lifestyle'] },
  'Writer': { description: 'Professional content creation specialist with expertise in technical writing, creative storytelling, and strategic communication across multiple formats and audiences. Crafts compelling narratives, clear documentation, and engaging content that effectively communicates complex ideas and drives meaningful engagement.', subtitle: 'Content writing expert', capabilities: ['Technical Documentation & Writing', 'Creative Content & Storytelling', 'Strategic Communication Planning', 'Multi-Format Content Creation', 'Editorial & Content Strategy', 'Audience Engagement & Messaging'] }
};

// Add all remaining agents to the dataset
remainingAgentNames.forEach((agentName, index) => {
  const agentInfo = agentDescriptions[agentName] || {
    description: `${agentName} specialist with advanced capabilities`,
    subtitle: `${agentName} expert`,
    capabilities: ['specialized-tasks', 'problem-solving', 'analysis']
  };

  const baseDate = new Date('2025-05-15T00:00:00.000Z');
  const createdDate = new Date(baseDate.getTime() + (index * 24 * 60 * 60 * 1000));
  const lastUsedDate = new Date(createdDate.getTime() + Math.random() * 60 * 24 * 60 * 60 * 1000);
  const usageCount = Math.floor(Math.random() * 100) + 1;

  mockAgentData.push({
    name: agentName,
    metadata: {
      name: agentName,
      description: agentInfo.description,
      capabilities: agentInfo.capabilities,
      created_at: createdDate.toISOString(),
      last_used: lastUsedDate.toISOString(),
      usage_count: usageCount,
      version: '1.0',
      toolkit_path: `/agents/${agentName}`,
      attributes: {}
    },
    components: [
      { name: 'Memory', path: `${agentName}_memory.md`, type: 'memory', lastModified: lastUsedDate.toISOString().split('T')[0] },
      { name: 'Continuous Learning', path: 'ContinuousLearning.md', type: 'learning', lastModified: lastUsedDate.toISOString().split('T')[0] },
      { name: 'README', path: 'README.md', type: 'readme', lastModified: lastUsedDate.toISOString().split('T')[0] }
    ],
    color: getAgentColor(agentName),
    subtitle: agentInfo.subtitle
  });
});

export const getAgents = (): Promise<Agent[]> => {
  return new Promise((resolve) => {
    // Force fresh data with timestamp - UPDATED DESCRIPTIONS
    console.log('Loading enhanced agents at', new Date().toISOString());
    setTimeout(() => {
      resolve(mockAgentData.sort((a, b) => a.name.localeCompare(b.name)));
    }, 100);
  });
};

export const getAgent = (name: string): Promise<Agent | null> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      const agent = mockAgentData.find(a => a.name === name) || null;
      resolve(agent);
    }, 50);
  });
};

export const getMarkdownContent = (agentName: string, componentPath: string): Promise<string> => {
  return new Promise((resolve) => {
    const componentType = componentPath.toLowerCase().replace('.md', '');
    const agent = mockAgentData.find(a => a.name === agentName);
    
    let mockContent = '';
    
    if (componentType.includes('memory')) {
      mockContent = `# ${agentName} Agent Memory System

## Overview
The memory system for ${agentName} maintains persistent state, learned patterns, and contextual information across sessions. This component serves as the agent's knowledge repository and experience database, enabling continuous improvement and personalized interactions.

### Core Functions
- **Experience Retention**: Stores successful interaction patterns and decision outcomes
- **Context Preservation**: Maintains conversation history and user preferences
- **Knowledge Integration**: Continuously updates understanding based on new information
- **Pattern Recognition**: Identifies recurring themes and optimal response strategies

### Memory Architecture
\`\`\`
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Short-term     │───▶│  Working        │───▶│  Long-term      │
│  Buffer         │    │  Memory         │    │  Storage        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
    Real-time             Active context         Permanent
    processing            & reasoning           knowledge base
\`\`\`

### Recent Memory Updates
- Optimized retrieval algorithms for faster context access
- Enhanced pattern matching for improved response relevance  
- Expanded capacity for domain-specific knowledge retention
- Implemented memory compression for efficient storage

### Performance Statistics
- Memory Utilization: ${(60 + Math.random() * 30).toFixed(1)}%
- Retrieval Speed: ${(10 + Math.random() * 20).toFixed(0)}ms
- Pattern Match Accuracy: ${(88 + Math.random() * 12).toFixed(1)}%
- Knowledge Retention Rate: ${(92 + Math.random() * 8).toFixed(1)}%

---
*Memory system last optimized: ${new Date(Date.now() - Math.random() * 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]}*`;

    } else if (componentType.includes('learning')) {
      mockContent = `# ${agentName} Continuous Learning Framework

## Overview
The continuous learning system enables ${agentName} to evolve and improve through experience, feedback, and exposure to new information. This adaptive framework ensures the agent remains current with emerging trends and user needs in its domain.

### Learning Methodologies
- **Reinforcement Learning**: Optimizes decision-making through reward-based feedback
- **Transfer Learning**: Applies knowledge from similar domains to new challenges
- **Active Learning**: Strategically identifies knowledge gaps and seeks relevant information
- **Meta-Learning**: Learns how to learn more effectively from limited examples

### Current Learning Focus Areas
${agent?.metadata.capabilities.map(cap => `- **${cap}**: Advancing proficiency in ${cap.toLowerCase().replace(/-/g, ' ')}`).join('\n') || '- Domain-specific expertise enhancement'}

### Learning Pipeline
\`\`\`mermaid
graph TD
    A[New Experience] --> B[Pattern Analysis]
    B --> C[Knowledge Integration]
    C --> D[Model Update]
    D --> E[Validation Testing]
    E --> F[Performance Monitoring]
    F --> A
\`\`\`

### Recent Learning Achievements
- Improved accuracy in complex problem-solving scenarios by 15%
- Enhanced understanding of nuanced user requirements
- Expanded knowledge base with ${Math.floor(50 + Math.random() * 200)} new concepts
- Optimized response generation for better user satisfaction

### Learning Metrics
- Learning Velocity: ${(0.5 + Math.random() * 1.5).toFixed(2)} concepts/day
- Knowledge Retention: ${(85 + Math.random() * 15).toFixed(1)}%
- Adaptation Speed: ${(20 + Math.random() * 30).toFixed(0)} iterations
- Cross-Domain Transfer: ${(70 + Math.random() * 25).toFixed(1)}%

---
*Learning framework last updated: ${new Date(Date.now() - Math.random() * 3 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]}*`;

    } else if (componentType.includes('readme')) {
      mockContent = `# ${agentName} Agent Documentation

## Agent Overview
${agent?.metadata.description || `${agentName} is a specialized AI agent designed to provide expert-level assistance in its domain of expertise.`}

## Core Capabilities
${agent?.metadata.capabilities.map(cap => `### ${cap}\n${agentName} demonstrates advanced proficiency in ${cap.toLowerCase().replace(/-/g, ' ')}, providing comprehensive solutions and insights tailored to user needs.\n`).join('\n') || 'This agent provides specialized expertise in its designated domain.'}

## Getting Started

### Quick Start Guide
1. **Initialize**: Begin by describing your specific requirements or challenges
2. **Collaborate**: Engage in detailed discussions about your objectives
3. **Implement**: Receive tailored recommendations and actionable solutions
4. **Iterate**: Refine approaches based on feedback and results

### Best Practices
- Provide clear, specific context for optimal results
- Share relevant background information and constraints
- Ask follow-up questions to deepen understanding
- Provide feedback to improve future interactions

## Integration & Compatibility
- **Framework Compatibility**: Works seamlessly with the Collaborative Intelligence platform
- **API Integration**: Supports standard REST and GraphQL interfaces  
- **Data Formats**: Processes JSON, XML, CSV, and custom data structures
- **Security**: Implements enterprise-grade security and privacy protocols

## Performance Characteristics
- **Response Time**: Typically ${(50 + Math.random() * 100).toFixed(0)}ms for standard queries
- **Throughput**: Handles up to ${Math.floor(100 + Math.random() * 900)} requests per minute
- **Accuracy**: Maintains ${(88 + Math.random() * 12).toFixed(1)}% accuracy across domain tasks
- **Reliability**: ${(95 + Math.random() * 5).toFixed(2)}% uptime with automatic failover

## Support & Maintenance
- Regular updates ensure optimal performance and security
- Comprehensive logging enables detailed performance analysis  
- Built-in monitoring provides real-time health status
- Automated backup ensures data persistence and recovery

---
*Documentation version ${agent?.metadata.version || '1.0'} | Last updated: ${new Date().toISOString().split('T')[0]}*`;
    }

    // Add timestamp to ensure fresh content
    console.log(`Loading ${componentType} content for ${agentName} at ${new Date().toISOString()}`);
    
    setTimeout(() => {
      resolve(mockContent);
    }, 200 + Math.random() * 300); // Slightly longer delay for more realistic loading
  });
};