import React, { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import { Agent, AgentComponent } from '../types/agent';
import { getAgents, getMarkdownContent } from '../services/agentService';
import { useTheme } from '../contexts/ThemeContext';

interface GridViewProps {
  onAgentSelect?: (agent: Agent) => void;
}

export const GridView: React.FC<GridViewProps> = ({ onAgentSelect }) => {
  const { theme } = useTheme();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [selectedComponent, setSelectedComponent] = useState<AgentComponent | null>(null);
  const [markdownContent, setMarkdownContent] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [loadingMarkdown, setLoadingMarkdown] = useState(false);
  const [showAgentDetail, setShowAgentDetail] = useState(false);

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
    setSelectedAgent(agent);
    setShowAgentDetail(true);
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

  const closeAgentDetail = () => {
    setShowAgentDetail(false);
    setSelectedAgent(null);
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
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-800 mb-2">Agent Grid</h1>
          <p className="text-gray-600">{agents.length} agents organized in a visual grid</p>
        </div>

        {/* Agent Grid - 4 per row */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {agents.map((agent) => (
            <button
              key={agent.name}
              onClick={() => handleAgentClick(agent)}
              className={`group p-6 rounded-xl border-2 text-left transition-all duration-300 hover:shadow-lg hover:scale-105 ${
                theme === 'color' ? agent.color : 
                theme === 'light' ? 'bg-white border-gray-200 hover:border-gray-300 hover:shadow-xl' :
                'bg-gray-800 border-gray-600 hover:border-gray-500 hover:shadow-xl text-white'
              }`}
            >
              {/* Agent Icon/Avatar */}
              <div className="flex items-center justify-center w-16 h-16 rounded-full bg-white bg-opacity-50 mb-4 mx-auto">
                <div className="w-8 h-8 rounded-full bg-current opacity-60"></div>
              </div>

              {/* Agent Name */}
              <h3 className="font-bold text-lg text-center mb-2 truncate">
                {agent.name}
              </h3>

              {/* Agent Subtitle */}
              <p className="text-sm opacity-80 text-center mb-3 line-clamp-2 min-h-[2.5rem]">
                {agent.subtitle}
              </p>

              {/* Stats */}
              <div className="flex justify-between items-center text-xs opacity-70 border-t border-current border-opacity-20 pt-3">
                <span>{agent.metadata.usage_count} uses</span>
                <span>{agent.components.length} parts</span>
              </div>

              {/* Hover indicator */}
              <div className="mt-3 text-center">
                <div className="inline-flex items-center text-xs opacity-0 group-hover:opacity-100 transition-opacity">
                  <span>Click to explore</span>
                  <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </div>
            </button>
          ))}
        </div>

        {/* Empty state if no agents */}
        {agents.length === 0 && (
          <div className="text-center py-12">
            <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
            <h3 className="text-lg font-medium text-gray-600 mb-2">No Agents Available</h3>
            <p className="text-gray-500">Check back later for available agents.</p>
          </div>
        )}
      </div>

      {/* Agent Detail Modal */}
      {showAgentDetail && selectedAgent && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-40">
          <div className="bg-white rounded-xl shadow-2xl max-w-4xl max-h-[90vh] w-full overflow-hidden">
            {/* Header */}
            <div className={`p-6 border-b border-gray-200 ${
              theme === 'color' ? selectedAgent.color : 
              theme === 'light' ? 'bg-blue-50' :
              'bg-blue-900 bg-opacity-20 text-white'
            }`}>
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="text-2xl font-bold">{selectedAgent.name}</h2>
                  <p className="opacity-80 mt-1">{selectedAgent.metadata.description}</p>
                </div>
                <button
                  onClick={closeAgentDetail}
                  className="p-2 hover:bg-black hover:bg-opacity-10 rounded-lg transition-colors"
                  aria-label="Close"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>

            {/* Content */}
            <div className="p-6 overflow-y-auto max-h-[calc(90vh-140px)]">
              {/* Agent Stats */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <div className="text-2xl font-bold text-gray-800">{selectedAgent.metadata.usage_count}</div>
                  <div className="text-sm text-gray-600">Total Uses</div>
                </div>
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <div className="text-2xl font-bold text-gray-800">{selectedAgent.components.length}</div>
                  <div className="text-sm text-gray-600">Components</div>
                </div>
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <div className="text-2xl font-bold text-gray-800">{selectedAgent.metadata.version}</div>
                  <div className="text-sm text-gray-600">Version</div>
                </div>
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <div className="text-2xl font-bold text-gray-800">
                    {Math.floor((Date.now() - new Date(selectedAgent.metadata.last_used).getTime()) / (1000 * 60 * 60 * 24))}d
                  </div>
                  <div className="text-sm text-gray-600">Last Used</div>
                </div>
              </div>

              {/* Default Agent Overview - shown when no component is selected */}
              {!selectedComponent && (
                <div className="mb-6">
                  <div className="flex items-center gap-2 mb-4">
                    <div className={`w-4 h-4 rounded-full ${
                      theme === 'color' ? 'bg-current opacity-60' :
                      theme === 'light' ? 'bg-blue-500' : 'bg-blue-400'
                    }`} />
                    <h3 className="text-lg font-semibold text-gray-800">Agent Overview</h3>
                  </div>
                  
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                      <h4 className="font-medium text-gray-800 mb-2">About This Agent</h4>
                      <p className="text-gray-700 text-sm leading-relaxed mb-3">{selectedAgent.metadata.description}</p>
                      
                      <h4 className="font-medium text-gray-800 mb-2">Key Capabilities</h4>
                      <div className="flex flex-wrap gap-1">
                        {selectedAgent.metadata.capabilities.slice(0, 6).map((capability, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-xs font-medium border border-blue-200"
                          >
                            {capability.replace('-', ' ')}
                          </span>
                        ))}
                        {selectedAgent.metadata.capabilities.length > 6 && (
                          <span className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs">
                            +{selectedAgent.metadata.capabilities.length - 6} more
                          </span>
                        )}
                      </div>
                    </div>
                    
                    <div>
                      <h4 className="font-medium text-gray-800 mb-2">Agent Statistics</h4>
                      <div className="grid grid-cols-2 gap-2">
                        <div className="text-center p-2 bg-gray-50 rounded">
                          <div className="text-lg font-bold text-gray-800">{selectedAgent.metadata.usage_count}</div>
                          <div className="text-xs text-gray-600">Uses</div>
                        </div>
                        <div className="text-center p-2 bg-gray-50 rounded">
                          <div className="text-lg font-bold text-gray-800">{selectedAgent.components.length}</div>
                          <div className="text-xs text-gray-600">Components</div>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  <div className="text-center">
                    <p className="text-gray-600 text-sm italic">
                      Select a component below to view detailed documentation.
                    </p>
                  </div>
                </div>
              )}

              {/* Components Grid */}
              <div>
                <h3 className="text-lg font-semibold text-gray-800 mb-4">Components</h3>
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                  {selectedAgent.components.map((component, index) => (
                    <button
                      key={`${component.path}-${index}`}
                      onClick={() => handleComponentClick(component)}
                      className={`p-4 rounded-lg border-2 hover:shadow-md transition-all duration-200 text-left group ${
                        selectedComponent?.path === component.path
                          ? 'bg-blue-50 border-blue-500 shadow-md'
                          : 'bg-gray-50 hover:bg-gray-100 border-gray-200 hover:border-gray-300'
                      }`}
                    >
                      <div className="flex items-start justify-between mb-2">
                        <h4 className={`font-medium transition-colors ${
                          selectedComponent?.path === component.path 
                            ? 'text-blue-700' 
                            : 'text-gray-800 group-hover:text-blue-600'
                        }`}>
                          {component.name}
                        </h4>
                        <div className={`w-3 h-3 rounded-full flex-shrink-0 ${
                          component.type === 'memory' ? 'bg-blue-400' :
                          component.type === 'learning' ? 'bg-green-400' :
                          component.type === 'readme' ? 'bg-purple-400' :
                          component.type === 'protocol' ? 'bg-orange-400' :
                          'bg-gray-400'
                        }`} />
                      </div>
                      <p className={`text-sm capitalize mb-2 ${
                        selectedComponent?.path === component.path ? 'text-blue-600' : 'text-gray-600'
                      }`}>{component.type}</p>
                      <p className="text-xs text-gray-500">Updated {component.lastModified}</p>
                    </button>
                  ))}
                </div>

                {/* Inline Markdown Content */}
                {selectedComponent && markdownContent && (
                  <div className="inline-markdown-container mt-6">
                    <div className="flex items-center justify-between mb-4">
                      <h4 className="text-lg font-semibold text-gray-800 flex items-center gap-2">
                        <div className={`w-4 h-4 rounded-full ${
                          selectedComponent.type === 'memory' ? 'bg-blue-400' :
                          selectedComponent.type === 'learning' ? 'bg-green-400' :
                          selectedComponent.type === 'readme' ? 'bg-purple-400' :
                          selectedComponent.type === 'protocol' ? 'bg-orange-400' :
                          'bg-gray-400'
                        }`} />
                        {selectedComponent.name}
                      </h4>
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
                      <div className="markdown-content max-h-96 overflow-y-auto">
                        <ReactMarkdown>{markdownContent}</ReactMarkdown>
                      </div>
                    )}
                  </div>
                )}
              </div>

              {/* Capabilities */}
              {selectedAgent.metadata.capabilities.length > 0 && (
                <div className="mt-6">
                  <h3 className="text-lg font-semibold text-gray-800 mb-4">Capabilities</h3>
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
          </div>
        </div>
      )}

    </div>
  );
};