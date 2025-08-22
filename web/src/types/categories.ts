export interface Category {
  id: string;
  name: string;
  color: string;
  description: string;
  keywords: string[];
}

export const AGENT_CATEGORIES: Category[] = [
  {
    id: 'development',
    name: 'Development',
    color: 'bg-blue-300 text-blue-900',
    description: 'Code development, programming, and software engineering',
    keywords: ['development', 'coding', 'programming', 'software', 'frontend', 'backend', 'web', 'app']
  },
  {
    id: 'testing',
    name: 'Testing',
    color: 'bg-green-300 text-green-900',
    description: 'Quality assurance, testing, and validation',
    keywords: ['testing', 'qa', 'quality-assurance', 'validation', 'verification', 'debugging', 'benchmarking']
  },
  {
    id: 'architecture',
    name: 'Architecture',
    color: 'bg-purple-300 text-purple-900',
    description: 'System architecture, design patterns, and technical planning',
    keywords: ['architecture', 'system-design', 'technical-architecture', 'solution-architecture', 'scalability', 'infrastructure']
  },
  {
    id: 'design',
    name: 'Design',
    color: 'bg-pink-300 text-pink-900',
    description: 'UI/UX design, visual design, and user experience',
    keywords: ['design', 'ui', 'ux', 'visual', 'interface', 'user-experience', 'prototyping', 'graphics']
  },
  {
    id: 'business',
    name: 'Business',
    color: 'bg-orange-300 text-orange-900',
    description: 'Business strategy, management, and operations',
    keywords: ['business', 'strategy', 'management', 'operations', 'marketing', 'sales', 'entrepreneurship', 'finance']
  },
  {
    id: 'analysis',
    name: 'Analysis',
    color: 'bg-teal-300 text-teal-900',
    description: 'Data analysis, research, and insights',
    keywords: ['analysis', 'research', 'data', 'insights', 'analytics', 'pattern-recognition', 'investigation']
  },
  {
    id: 'automation',
    name: 'Automation',
    color: 'bg-indigo-300 text-indigo-900',
    description: 'Process automation, scripting, and workflow optimization',
    keywords: ['automation', 'scripting', 'workflow', 'optimization', 'process', 'streamlining', 'efficiency']
  },
  {
    id: 'specialized',
    name: 'Specialized',
    color: 'bg-slate-300 text-slate-900',
    description: 'Domain-specific and specialized expertise',
    keywords: ['specialized', 'domain', 'expert', 'consultant', 'advisor', 'wisdom', 'knowledge']
  }
];

export const categorizeAgent = (agent: { metadata: { capabilities: string[]; description: string } }): string[] => {
  const categories: string[] = [];
  const allText = [...agent.metadata.capabilities, agent.metadata.description.toLowerCase()].join(' ');
  
  AGENT_CATEGORIES.forEach(category => {
    const hasMatch = category.keywords.some(keyword => 
      allText.includes(keyword.toLowerCase()) || 
      allText.includes(keyword.toLowerCase().replace('-', ' '))
    );
    
    if (hasMatch) {
      categories.push(category.id);
    }
  });
  
  // If no categories match, assign to specialized
  if (categories.length === 0) {
    categories.push('specialized');
  }
  
  return categories;
};