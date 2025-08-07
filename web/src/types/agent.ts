export interface AgentMetadata {
  name: string;
  description: string;
  capabilities: string[];
  created_at: string;
  last_used: string;
  usage_count: number;
  version: string;
  toolkit_path: string;
  memory_path?: string;
  learning_path?: string;
  attributes: Record<string, any>;
}

export interface AgentComponent {
  name: string;
  path: string;
  type: 'memory' | 'learning' | 'readme' | 'protocol' | 'session' | 'other';
  lastModified: string;
}

export interface Agent {
  name: string;
  metadata: AgentMetadata;
  components: AgentComponent[];
  color: string;
  subtitle?: string;
}