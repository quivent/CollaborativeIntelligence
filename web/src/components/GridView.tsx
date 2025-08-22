import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Agent, AgentComponent } from '../types/agent';
import { getAgents, getMarkdownContent } from '../services/agentService';
import { useTheme } from '../contexts/ThemeContext';
import { useSearch } from '../contexts/SearchContext';
import { CategoryFilter } from './CategoryFilter';
import { categorizeAgent } from '../types/categories';

interface GridViewProps {
  onAgentSelect?: (agent: Agent) => void;
}

export const GridView: React.FC<GridViewProps> = ({ onAgentSelect }) => {
  const { theme } = useTheme();
  const { filterAgents } = useSearch();
  const navigate = useNavigate();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [selectedComponent, setSelectedComponent] = useState<AgentComponent | null>(null);
  const [markdownContent, setMarkdownContent] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [loadingMarkdown, setLoadingMarkdown] = useState(false);
  const [showAgentDetail, setShowAgentDetail] = useState(false);
  const [selectedCategories, setSelectedCategories] = useState<string[]>([]);

  useEffect(() => {
    loadAgents();
  }, []);

  // Clear category filters when component mounts (i.e., when navigating to this view)
  useEffect(() => {
    setSelectedCategories([]);
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

  const filteredAgents = useMemo(() => {
    // First apply search filter
    let filtered = filterAgents(agents);
    
    // Then apply category filter
    if (selectedCategories.length > 0) {
      filtered = filtered.filter(agent => {
        const agentCategories = categorizeAgent(agent);
        return selectedCategories.every(category => agentCategories.includes(category));
      });
    }
    
    return filtered;
  }, [agents, selectedCategories, filterAgents]);

  const handleCategoryToggle = (categoryId: string) => {
    setSelectedCategories(prev => 
      prev.includes(categoryId)
        ? prev.filter(id => id !== categoryId)
        : [...prev, categoryId]
    );
  };

  const handleClearAllCategories = () => {
    setSelectedCategories([]);
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

  const handleStartChat = (agent: Agent, e: React.MouseEvent) => {
    e.stopPropagation();
    navigate(`/chat?agent=${encodeURIComponent(agent.name)}`);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-300"></div>
      </div>
    );
  }

  return (
    <div className="p-4 bg-gray-50 min-h-screen">
      <div className="max-w-full mx-auto px-4">

        {/* Category Filter */}
        <div className="mb-8 p-6 bg-white rounded-xl shadow-sm">
          <CategoryFilter
            selectedCategories={selectedCategories}
            onCategoryToggle={handleCategoryToggle}
            onClearAll={handleClearAllCategories}
            className="grid-view"
          />
        </div>

        {/* Agent Grid - Optimized for more agents per row */}
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4">
          {filteredAgents.map((agent) => (
            <div
              key={agent.name}
              onClick={() => handleAgentClick(agent)}
              className={`group p-4 rounded-lg border-2 text-left transition-all duration-300 hover:shadow-lg hover:scale-105 cursor-pointer ${
                theme === 'color' ? `${agent.color} hover:brightness-110 hover:saturate-110` : 
                theme === 'light' ? 'bg-white border-gray-200 hover:border-gray-300 hover:shadow-xl hover:bg-blue-50' :
                'bg-gray-800 border-gray-600 hover:border-gray-500 hover:shadow-xl hover:bg-gray-700 text-white'
              }`}
            >
              {/* Agent Icon/Avatar */}
              <div className="flex items-center justify-center w-12 h-12 rounded-full bg-white bg-opacity-50 mb-3 mx-auto">
                <div className="w-6 h-6 rounded-full bg-current opacity-60"></div>
              </div>

              {/* Agent Name */}
              <h3 className="font-bold text-base text-center mb-2 truncate">
                {agent.name}
              </h3>

              {/* Agent Subtitle */}
              <p className="text-xs opacity-80 text-center mb-2 line-clamp-2 min-h-[2rem]">
                {agent.subtitle}
              </p>

              {/* Stats */}
              <div className="flex justify-between items-center text-xs opacity-70 border-t border-current border-opacity-20 pt-2 mb-3">
                <span>{agent.metadata.usage_count} uses</span>
                <span>{agent.components.length} parts</span>
              </div>

              {/* Action Buttons */}
              <div className="flex gap-1">
                <button
                  onClick={(e) => handleStartChat(agent, e)}
                  className={`flex-1 px-2 py-1.5 text-xs font-medium rounded-lg border transition-all duration-200 flex items-center justify-center gap-1 ${
                    theme === 'color' 
                      ? 'bg-white bg-opacity-90 text-current border-white border-opacity-30 hover:bg-opacity-100 hover:border-opacity-50'
                      : theme === 'light'
                      ? 'bg-blue-100 text-blue-700 border-blue-200 hover:bg-blue-200 hover:border-blue-300'
                      : 'bg-gray-700 text-white border-gray-600 hover:bg-gray-600 hover:border-gray-500'
                  }`}
                >
                  <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                  </svg>
                  <span>Chat</span>
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Empty state if no agents */}
        {filteredAgents.length === 0 && (
          <div className="text-center py-12">
            <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
            <h3 className="text-lg font-medium text-gray-600 mb-2">
              {selectedCategories.length > 0 ? 'No Agents Match Filter' : 'No Agents Available'}
            </h3>
            <p className="text-gray-500">
              {selectedCategories.length > 0 
                ? 'Try adjusting your category filters to see more agents.'
                : 'Check back later for available agents.'
              }
            </p>
            {selectedCategories.length > 0 && (
              <button
                onClick={handleClearAllCategories}
                className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
              >
                Clear Filters
              </button>
            )}
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
                      <div className="space-y-2">
                        {selectedAgent.metadata.capabilities.slice(0, 4).map((capability, index) => (
                          <div
                            key={index}
                            className="flex items-start gap-2 p-2 bg-blue-50 rounded-lg border border-blue-100"
                          >
                            <div className="w-1.5 h-1.5 bg-blue-500 rounded-full mt-1.5 flex-shrink-0"></div>
                            <div>
                              <div className="font-medium text-blue-900 text-xs">{capability}</div>
                              <div className="text-xs text-blue-700 mt-0.5">
                                Advanced proficiency with proven expertise
                              </div>
                            </div>
                          </div>
                        ))}
                        {selectedAgent.metadata.capabilities.length > 4 && (
                          <div className="text-xs text-gray-500 text-center pt-1">
                            +{selectedAgent.metadata.capabilities.length - 4} additional capabilities
                          </div>
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
                      className={`p-4 rounded-lg border transition-all duration-200 text-left group ${
                        selectedComponent?.path === component.path
                          ? 'bg-blue-100 border-blue-300 shadow-md text-blue-900'
                          : 'bg-white hover:bg-gray-50 border-gray-200 hover:border-gray-300'
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
                          component.type === 'memory' ? 'bg-blue-300' :
                          component.type === 'learning' ? 'bg-green-300' :
                          component.type === 'readme' ? 'bg-purple-300' :
                          component.type === 'protocol' ? 'bg-orange-300' :
                          'bg-gray-300'
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
                        className="p-2 text-gray-400 hover:text-gray-600 hover:bg-blue-50 border border-transparent hover:border-blue-200 rounded-lg transition-all duration-200"
                        title="Close document"
                      >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                      </button>
                    </div>
                    
                    {loadingMarkdown ? (
                      <div className="flex items-center justify-center py-8">
                        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-300"></div>
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
                  <h3 className="text-lg font-semibold text-gray-800 mb-4">All Capabilities</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {selectedAgent.metadata.capabilities.map((capability, index) => (
                      <div
                        key={index}
                        className="flex items-start gap-3 p-3 bg-gradient-to-r from-blue-50 to-purple-50 rounded-lg border border-blue-100 hover:border-blue-200 transition-colors"
                      >
                        <div className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></div>
                        <div>
                          <h4 className="font-medium text-blue-900 mb-1">{capability}</h4>
                          <p className="text-sm text-blue-700">
                            Professional expertise with comprehensive knowledge and practical experience in {capability.toLowerCase().replace(/[&-]/g, ' and ').replace(/  +/g, ' ')}.
                          </p>
                        </div>
                      </div>
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