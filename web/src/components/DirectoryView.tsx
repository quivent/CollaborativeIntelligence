import React, { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import { Agent, AgentComponent } from '../types/agent';
import { getAgents, getMarkdownContent } from '../services/agentService';
import { useTheme } from '../contexts/ThemeContext';

interface DirectoryViewProps {
  onAgentSelect?: (agent: Agent) => void;
}

export const DirectoryView: React.FC<DirectoryViewProps> = ({ onAgentSelect }) => {
  const { theme } = useTheme();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [selectedComponent, setSelectedComponent] = useState<AgentComponent | null>(null);
  const [markdownContent, setMarkdownContent] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [loadingMarkdown, setLoadingMarkdown] = useState(false);

  useEffect(() => {
    loadAgents();
  }, []);

  const loadAgents = async () => {
    try {
      setLoading(true);
      const agentData = await getAgents();
      setAgents(agentData);
    } catch (error) {
      console.error('Failed to load agents:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAgentClick = (agent: Agent) => {
    // Clear any selected component when changing agents
    setSelectedComponent(null);
    setMarkdownContent('');
    setSelectedAgent(selectedAgent?.name === agent.name ? null : agent);
    onAgentSelect?.(agent);
  };

  const handleComponentClick = async (component: AgentComponent) => {
    if (!selectedAgent) return;

    // If clicking the same component, close it
    if (selectedComponent?.path === component.path) {
      closeMarkdownViewer();
      return;
    }

    setLoadingMarkdown(true);
    try {
      const content = await getMarkdownContent(selectedAgent.name, component.path);
      setMarkdownContent(content);
      setSelectedComponent(component);
    } catch (error) {
      console.error('Failed to load markdown content:', error);
      setMarkdownContent('Failed to load content.');
      setSelectedComponent(component);
    } finally {
      setLoadingMarkdown(false);
    }
  };

  const closeMarkdownViewer = () => {
    setSelectedComponent(null);
    setMarkdownContent('');
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="flex h-full bg-gray-50">
      {/* Left Sidebar - Agent Directory */}
      <div className="w-80 bg-white shadow-lg overflow-y-auto">
        <div className="p-6 border-b border-gray-200">
          <h2 className="text-2xl font-bold text-gray-800">Agent Directory</h2>
          <p className="text-sm text-gray-600 mt-1">{agents.length} agents available</p>
        </div>
        
        <div className="p-4 space-y-2">
          {agents.map((agent) => (
            <button
              key={agent.name}
              onClick={() => handleAgentClick(agent)}
              className={`w-full text-left p-4 rounded-lg border-2 transition-all duration-200 ${
                theme === 'color' ? agent.color : 
                theme === 'light' ? 'bg-white border-gray-200 hover:border-gray-300' :
                'bg-gray-800 border-gray-600 hover:border-gray-500'
              } ${
                selectedAgent?.name === agent.name
                  ? theme === 'color' 
                    ? 'ring-2 ring-white ring-opacity-50 shadow-lg scale-105'
                    : theme === 'light' 
                    ? 'ring-2 ring-blue-500 ring-opacity-50 shadow-lg scale-105' 
                    : 'ring-2 ring-blue-400 ring-opacity-50 shadow-lg scale-105'
                  : 'hover:shadow-md hover:scale-102'
              }`}
            >
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="font-semibold text-gray-800">{agent.name}</h3>
                  <p className="text-sm text-gray-600 mt-1">{agent.subtitle}</p>
                  <div className="flex items-center mt-2 text-xs text-gray-500">
                    <span>Used {agent.metadata.usage_count} times</span>
                    <span className="mx-2">•</span>
                    <span>{agent.components.length} components</span>
                  </div>
                </div>
                <div className="text-gray-400">
                  {selectedAgent?.name === agent.name ? (
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                    </svg>
                  ) : (
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                    </svg>
                  )}
                </div>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Right Content Area */}
      <div className="flex-1 overflow-y-auto">
        {selectedAgent ? (
          <div className="p-6">
            {/* Agent Header */}
            <div className={`p-6 rounded-xl border-2 mb-6 ${
              theme === 'color' ? selectedAgent.color : 
              theme === 'light' ? 'bg-blue-50 border-blue-200' :
              'bg-blue-900 bg-opacity-20 border-blue-500'
            }`}>
              <h1 className="text-3xl font-bold mb-2">{selectedAgent.name}</h1>
              <p className="text-lg opacity-80 mb-4">{selectedAgent.metadata.description}</p>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <span className="font-medium">Version:</span>
                  <div>{selectedAgent.metadata.version}</div>
                </div>
                <div>
                  <span className="font-medium">Usage:</span>
                  <div>{selectedAgent.metadata.usage_count} times</div>
                </div>
                <div>
                  <span className="font-medium">Created:</span>
                  <div>{new Date(selectedAgent.metadata.created_at).toLocaleDateString()}</div>
                </div>
                <div>
                  <span className="font-medium">Last Used:</span>
                  <div>{new Date(selectedAgent.metadata.last_used).toLocaleDateString()}</div>
                </div>
              </div>
            </div>

            {/* Default Agent Overview - shown when no component is selected */}
            {!selectedComponent && (
              <div className="inline-markdown-container mb-6">
                <div className="flex items-center gap-2 mb-4">
                  <div className={`w-4 h-4 rounded-full ${
                    theme === 'color' ? 'bg-current opacity-60' :
                    theme === 'light' ? 'bg-blue-500' : 'bg-blue-400'
                  }`} />
                  <h3 className="text-xl font-semibold text-gray-800">Agent Overview</h3>
                </div>
                
                <div className="markdown-content">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                    <div>
                      <h4 className="text-lg font-medium text-gray-800 mb-3">About This Agent</h4>
                      <p className="text-gray-700 leading-relaxed mb-4">{selectedAgent.metadata.description}</p>
                      
                      <h4 className="text-lg font-medium text-gray-800 mb-3">Key Capabilities</h4>
                      <div className="flex flex-wrap gap-2">
                        {selectedAgent.metadata.capabilities.map((capability, index) => (
                          <span
                            key={index}
                            className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium border border-blue-200"
                          >
                            {capability.replace('-', ' ')}
                          </span>
                        ))}
                      </div>
                    </div>
                    
                    <div>
                      <h4 className="text-lg font-medium text-gray-800 mb-3">Agent Statistics</h4>
                      <div className="space-y-3">
                        <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                          <span className="text-gray-600">Total Usage</span>
                          <span className="font-semibold text-gray-800">{selectedAgent.metadata.usage_count} times</span>
                        </div>
                        <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                          <span className="text-gray-600">Components</span>
                          <span className="font-semibold text-gray-800">{selectedAgent.components.length} available</span>
                        </div>
                        <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                          <span className="text-gray-600">Version</span>
                          <span className="font-semibold text-gray-800">{selectedAgent.metadata.version}</span>
                        </div>
                        <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                          <span className="text-gray-600">Last Active</span>
                          <span className="font-semibold text-gray-800">{new Date(selectedAgent.metadata.last_used).toLocaleDateString()}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  <div className="border-t border-gray-200 pt-4">
                    <p className="text-gray-600 text-center italic">
                      Select a component above to view detailed documentation and learn more about this agent's functionality.
                    </p>
                  </div>
                </div>
              </div>
            )}

            {/* Components Grid */}
            <div>
              <h2 className="text-xl font-semibold text-gray-800 mb-4">Components</h2>
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                {selectedAgent.components.map((component, index) => (
                  <button
                    key={`${component.path}-${index}`}
                    onClick={() => handleComponentClick(component)}
                    className={`p-4 bg-white rounded-lg border-2 hover:shadow-md transition-all duration-200 text-left group ${
                      selectedComponent?.path === component.path 
                        ? 'border-blue-500 shadow-md bg-blue-50' 
                        : 'border-gray-200 hover:border-gray-300'
                    }`}
                  >
                    <div className="flex items-start justify-between mb-2">
                      <h3 className={`font-medium transition-colors ${
                        selectedComponent?.path === component.path 
                          ? 'text-blue-700' 
                          : 'text-gray-800 group-hover:text-blue-600'
                      }`}>
                        {component.name}
                      </h3>
                      <div className={`w-3 h-3 rounded-full flex-shrink-0 ${
                        component.type === 'memory' ? 'bg-blue-400' :
                        component.type === 'learning' ? 'bg-green-400' :
                        component.type === 'readme' ? 'bg-purple-400' :
                        component.type === 'protocol' ? 'bg-orange-400' :
                        'bg-gray-400'
                      }`} />
                    </div>
                    <p className={`text-sm capitalize ${
                      selectedComponent?.path === component.path ? 'text-blue-600' : 'text-gray-600'
                    }`}>{component.type}</p>
                    <p className="text-xs text-gray-500 mt-2">Updated {component.lastModified}</p>
                  </button>
                ))}
              </div>

              {/* Inline Markdown Content */}
              {selectedComponent && markdownContent && (
                <div className="inline-markdown-container">
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-xl font-semibold text-gray-800 flex items-center gap-2">
                      <div className={`w-4 h-4 rounded-full ${
                        selectedComponent.type === 'memory' ? 'bg-blue-400' :
                        selectedComponent.type === 'learning' ? 'bg-green-400' :
                        selectedComponent.type === 'readme' ? 'bg-purple-400' :
                        selectedComponent.type === 'protocol' ? 'bg-orange-400' :
                        'bg-gray-400'
                      }`} />
                      {selectedComponent.name}
                    </h3>
                    <button
                      onClick={closeMarkdownViewer}
                      className="p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
                      title="Close document"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  
                  {loadingMarkdown ? (
                    <div className="flex items-center justify-center py-8">
                      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                    </div>
                  ) : (
                    <div className="markdown-content">
                      <ReactMarkdown>{markdownContent}</ReactMarkdown>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Capabilities */}
            {selectedAgent.metadata.capabilities.length > 0 && (
              <div className="mt-8">
                <h2 className="text-xl font-semibold text-gray-800 mb-4">Capabilities</h2>
                <div className="flex flex-wrap gap-2">
                  {selectedAgent.metadata.capabilities.map((capability, index) => (
                    <span
                      key={index}
                      className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium"
                    >
                      {capability.replace('-', ' ')}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              <h3 className="text-lg font-medium text-gray-600 mb-2">Select an Agent</h3>
              <p className="text-gray-500">Choose an agent from the directory to view its details and components.</p>
            </div>
          </div>
        )}
      </div>

    </div>
  );
};