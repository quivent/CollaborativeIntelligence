import { Agent } from '../types/agent';
import { getAgentColor } from '../utils/colors';

// Complete agent data service with ALL 106 agents from the AGENTS directory
const mockAgentData: Agent[] = [
  // Development & Technical Agents
  {
    name: 'Analyst',
    metadata: {
      name: 'Analyst',
      description: 'Data analysis and insights specialist',
      capabilities: ['data-analysis', 'pattern-recognition', 'reporting'],
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
      description: 'Application development and deployment specialist',
      capabilities: ['app-development', 'deployment', 'integration'],
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
      description: 'System architecture and design specialist',
      capabilities: ['system-design', 'architecture', 'scalability'],
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
      description: 'Creative and artistic content generation specialist',
      capabilities: ['creative-design', 'visual-arts', 'content-creation'],
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
      description: 'Strategic wisdom and decision-making specialist',
      capabilities: ['strategic-planning', 'decision-support', 'analysis'],
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
      description: 'Process automation and workflow specialist',
      capabilities: ['automation', 'workflow-optimization', 'scripting'],
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
      description: 'Backend development and API specialist',
      capabilities: ['backend-development', 'api-design', 'server-architecture'],
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
      description: 'Shell scripting and command-line specialist',
      capabilities: ['shell-scripting', 'command-line', 'system-administration'],
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

// Generate remaining 98 agents with varied metadata to complete the collection
const remainingAgentNames = [
  'Benchmarker', 'BusinessDevelopmentHead', 'BusinessManager', 'BusinessOwner', 'Bypasser', 
  'Cacher', 'Cartographer', 'CastleEventMaestro', 'CFO', 'CGUIProjectDeveloper', 
  'CLIA', 'ClientAcquisition', 'Comparitor', 'Consolidator', 'Database', 'Debugger',
  'Deliverer', 'Designer', 'Developer', 'DirectoryOrganizer', 'DocumentConverter',
  'Documenter', 'Documentor', 'Educator', 'Enforcer', 'Engineer', 'ETS_BRIDGE',
  'EventMarketer', 'Expert', 'Fixer', 'Gaia', 'GPUArchitect', 'HARMONICUS',
  'Hermes', 'Human', 'IIILanguageAgent', 'ImplementationArchitect', 'IncomeStrategist',
  'Infrastructurer', 'Inspector', 'LanguageDesigner', 'Learner', 'LieDetector',
  'Linguist', 'Manager', 'Marketer', 'Memory', 'MicroGreens', 'Mnemosyne',
  'MusicMaker', 'Neuroscientist', 'Optimizer', 'Overviewer', 'Pause', 'Pharmacologist',
  'Philosopher', 'Planner', 'ProjectBootstrapper', 'ProjectManager', 'Psychologist',
  'Rabbi', 'Reactor', 'Recommender', 'Refactorer', 'Renderer', 'Repairer',
  'Researcher', 'Resourcer', 'Restorer', 'Rustist', 'Sage', 'SageKeeper',
  'Salesman', 'Scholar', 'Shannon', 'SocialMediaist', 'SolutionArchitect', 'Solver',
  'SproutSeller', 'Standardist', 'StrategicPlanner', 'Streamliner', 'SwiftToCPorter',
  'TechnicalArchitect', 'Tester', 'Timer', 'Topologist', 'TravelGuider', 'UI',
  'User', 'UX', 'Verifier', 'Visionary', 'Visualist', 'Volcano', 'WebArchitect',
  'Wellness', 'Writer'
];

// Generate metadata for remaining agents
const agentDescriptions: { [key: string]: { description: string; subtitle: string; capabilities: string[] } } = {
  'Benchmarker': { description: 'Performance benchmarking and metrics specialist', subtitle: 'Performance testing expert', capabilities: ['benchmarking', 'performance-testing', 'metrics'] },
  'BusinessDevelopmentHead': { description: 'Business development and growth strategy specialist', subtitle: 'Business growth expert', capabilities: ['business-development', 'partnerships', 'growth-strategy'] },
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
  'Database': { description: 'Database management and optimization specialist', subtitle: 'Data management expert', capabilities: ['database-design', 'optimization', 'data-management'] },
  'Debugger': { description: 'Error detection and resolution specialist', subtitle: 'Error resolution expert', capabilities: ['debugging', 'error-analysis', 'troubleshooting'] },
  'Deliverer': { description: 'Project delivery and deployment specialist', subtitle: 'Delivery expert', capabilities: ['project-delivery', 'deployment', 'release-management'] },
  'Designer': { description: 'UI/UX and visual design specialist', subtitle: 'Visual design expert', capabilities: ['ui-design', 'visual-design', 'prototyping'] },
  'Developer': { description: 'Software development specialist', subtitle: 'Code development expert', capabilities: ['coding', 'software-development', 'architecture'] },
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
  'Manager': { description: 'Project and team management specialist', subtitle: 'Management expert', capabilities: ['project-management', 'team-coordination', 'resource-planning'] },
  'Marketer': { description: 'Marketing strategy and campaign specialist', subtitle: 'Marketing expert', capabilities: ['marketing-strategy', 'campaign-management', 'brand-development'] },
  'Memory': { description: 'Memory management and optimization specialist', subtitle: 'Memory management expert', capabilities: ['memory-management', 'optimization', 'data-storage'] },
  'MicroGreens': { description: 'Sustainable agriculture and microgreens specialist', subtitle: 'Sustainable agriculture expert', capabilities: ['sustainable-agriculture', 'microgreens', 'farming'] },
  'Mnemosyne': { description: 'Memory systems and recall specialist', subtitle: 'Memory systems expert', capabilities: ['memory-systems', 'recall', 'information-retention'] },
  'MusicMaker': { description: 'Music composition and audio creation specialist', subtitle: 'Music creation expert', capabilities: ['music-composition', 'audio-creation', 'sound-design'] },
  'Neuroscientist': { description: 'Neural networks and cognitive science specialist', subtitle: 'Neural science expert', capabilities: ['neural-networks', 'cognitive-science', 'brain-modeling'] },
  'Optimizer': { description: 'Performance optimization and efficiency specialist', subtitle: 'Optimization expert', capabilities: ['performance-optimization', 'efficiency', 'resource-management'] },
  'Overviewer': { description: 'System overview and monitoring specialist', subtitle: 'System overview expert', capabilities: ['system-monitoring', 'overview', 'dashboard-design'] },
  'Pause': { description: 'System pause and state management specialist', subtitle: 'State management expert', capabilities: ['state-management', 'pause-control', 'system-states'] },
  'Pharmacologist': { description: 'Pharmaceutical research and drug development specialist', subtitle: 'Pharmaceutical expert', capabilities: ['pharmaceutical-research', 'drug-development', 'medical-analysis'] },
  'Philosopher': { description: 'Philosophy and ethical reasoning specialist', subtitle: 'Philosophy expert', capabilities: ['philosophy', 'ethical-reasoning', 'logic'] },
  'Planner': { description: 'Strategic planning and roadmap specialist', subtitle: 'Strategic planning expert', capabilities: ['strategic-planning', 'roadmapping', 'goal-setting'] },
  'ProjectBootstrapper': { description: 'Project initialization and setup specialist', subtitle: 'Project setup expert', capabilities: ['project-initialization', 'bootstrapping', 'setup'] },
  'ProjectManager': { description: 'Specialized project management and delivery specialist', subtitle: 'Project delivery expert', capabilities: ['project-delivery', 'milestone-tracking', 'stakeholder-management'] },
  'Psychologist': { description: 'Psychology and behavioral analysis specialist', subtitle: 'Psychology expert', capabilities: ['psychology', 'behavioral-analysis', 'mental-health'] },
  'Rabbi': { description: 'Religious studies and spiritual guidance specialist', subtitle: 'Spiritual guidance expert', capabilities: ['religious-studies', 'spiritual-guidance', 'philosophy'] },
  'Reactor': { description: 'Event reaction and response specialist', subtitle: 'Event response expert', capabilities: ['event-handling', 'automated-responses', 'monitoring'] },
  'Recommender': { description: 'Recommendation systems and AI specialist', subtitle: 'Recommendation expert', capabilities: ['recommendation-systems', 'machine-learning', 'personalization'] },
  'Refactorer': { description: 'Code refactoring and improvement specialist', subtitle: 'Code refactoring expert', capabilities: ['code-refactoring', 'code-improvement', 'technical-debt'] },
  'Renderer': { description: 'Graphics rendering and visualization specialist', subtitle: 'Rendering expert', capabilities: ['graphics-rendering', 'visualization', '3d-graphics'] },
  'Repairer': { description: 'System repair and maintenance specialist', subtitle: 'System repair expert', capabilities: ['system-repair', 'maintenance', 'troubleshooting'] },
  'Researcher': { description: 'Research and investigation specialist', subtitle: 'Research expert', capabilities: ['research', 'data-gathering', 'analysis'] },
  'Resourcer': { description: 'Resource management and allocation specialist', subtitle: 'Resource management expert', capabilities: ['resource-management', 'allocation', 'optimization'] },
  'Restorer': { description: 'System restoration and recovery specialist', subtitle: 'System restoration expert', capabilities: ['system-restoration', 'recovery', 'backup'] },
  'Rustist': { description: 'Rust programming language specialist', subtitle: 'Rust programming expert', capabilities: ['rust-programming', 'systems-programming', 'memory-safety'] },
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
  'Tester': { description: 'Quality assurance and testing specialist', subtitle: 'Quality assurance expert', capabilities: ['testing', 'qa', 'validation'] },
  'Timer': { description: 'Time management and scheduling specialist', subtitle: 'Time management expert', capabilities: ['time-management', 'scheduling', 'productivity'] },
  'Topologist': { description: 'System topology and network structure specialist', subtitle: 'System topology expert', capabilities: ['topology-analysis', 'network-design', 'system-architecture'] },
  'TravelGuider': { description: 'Travel planning and guidance specialist', subtitle: 'Travel planning expert', capabilities: ['travel-planning', 'guidance', 'logistics'] },
  'UI': { description: 'User interface design and development specialist', subtitle: 'UI design expert', capabilities: ['ui-design', 'interface-development', 'user-experience'] },
  'User': { description: 'User experience and interaction specialist', subtitle: 'User experience expert', capabilities: ['user-experience', 'interaction-design', 'usability'] },
  'UX': { description: 'User experience research and design specialist', subtitle: 'UX research expert', capabilities: ['ux-research', 'user-experience', 'design-thinking'] },
  'Verifier': { description: 'Verification and validation specialist', subtitle: 'Verification expert', capabilities: ['verification', 'validation', 'quality-assurance'] },
  'Visionary': { description: 'Vision and future planning specialist', subtitle: 'Vision expert', capabilities: ['vision-planning', 'future-strategy', 'innovation'] },
  'Visualist': { description: 'Data visualization and graphics specialist', subtitle: 'Visualization expert', capabilities: ['data-visualization', 'graphics', 'visual-communication'] },
  'Volcano': { description: 'High-energy system and explosive growth specialist', subtitle: 'High-energy systems expert', capabilities: ['high-energy-systems', 'explosive-growth', 'performance'] },
  'WebArchitect': { description: 'Web development architecture specialist', subtitle: 'Web architecture expert', capabilities: ['web-architecture', 'frontend-development', 'performance-optimization'] },
  'Wellness': { description: 'Health and wellness optimization specialist', subtitle: 'Wellness expert', capabilities: ['health-optimization', 'wellness', 'lifestyle'] },
  'Writer': { description: 'Content writing and communication specialist', subtitle: 'Content writing expert', capabilities: ['content-writing', 'communication', 'storytelling'] }
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
    const mockContent = `# ${componentPath.replace('.md', '').replace(/([A-Z])/g, ' $1').trim()}

## Overview
This is the ${componentPath.toLowerCase().replace('.md', '')} for the ${agentName} agent.

### Key Features
- Advanced ${agentName.toLowerCase()} capabilities
- Continuous learning and adaptation
- Integration with collaborative intelligence system
- Real-time performance monitoring

### Recent Activities
- Enhanced processing algorithms
- Improved error handling
- Updated knowledge base
- Optimized response times

### Configuration
\`\`\`json
{
  "agent": "${agentName}",
  "version": "1.0",
  "status": "active",
  "capabilities": ["analysis", "learning", "collaboration"]
}
\`\`\`

### Performance Metrics
- Accuracy: ${(85 + Math.random() * 15).toFixed(1)}%
- Response Time: ${(50 + Math.random() * 200).toFixed(0)}ms
- Success Rate: ${(90 + Math.random() * 10).toFixed(1)}%
- Learning Rate: Adaptive

---
*Last updated: ${new Date().toISOString().split('T')[0]}*`;

    setTimeout(() => {
      resolve(mockContent);
    }, 100);
  });
};