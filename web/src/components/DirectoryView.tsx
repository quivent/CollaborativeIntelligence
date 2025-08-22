import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Agent, AgentComponent } from '../types/agent';
import { getAgents, getMarkdownContent } from '../services/agentService';
import { useTheme } from '../contexts/ThemeContext';
import { useSearch } from '../contexts/SearchContext';
import { CategoryFilter } from './CategoryFilter';
import { categorizeAgent } from '../types/categories';
import '../styles/pastel.css';

interface DirectoryViewProps {
  onAgentSelect?: (agent: Agent) => void;
}

export const DirectoryView: React.FC<DirectoryViewProps> = ({ onAgentSelect }) => {
  const { theme } = useTheme();
  const { filterAgents } = useSearch();
  const navigate = useNavigate();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [selectedComponent, setSelectedComponent] = useState<AgentComponent | null>(null);
  const [markdownContent, setMarkdownContent] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [loadingMarkdown, setLoadingMarkdown] = useState(false);
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
    <div className="flex h-full bg-gray-50">
      {/* Left Sidebar - Agent Directory */}
      <div className="w-80 bg-white shadow-lg overflow-y-auto">
        
        <div className="p-4 space-y-2">
          {filteredAgents.map((agent) => (
            <div
              key={agent.name}
              onClick={() => handleAgentClick(agent)}
              className={`w-full text-left p-4 rounded-lg border-2 transition-all duration-300 cursor-pointer transform ${
                theme === 'color' ? agent.color : 
                theme === 'light' ? 'bg-white border-gray-200 hover:border-blue-300 hover:bg-blue-50' :
                'bg-gray-800 border-gray-600 hover:border-gray-400 hover:bg-gray-750'
              } ${
                selectedAgent?.name === agent.name
                  ? theme === 'color' 
                    ? 'ring-2 ring-white ring-opacity-50 shadow-lg scale-105'
                    : theme === 'light' 
                    ? 'ring-2 ring-blue-500 ring-opacity-50 shadow-lg scale-105' 
                    : 'ring-2 ring-blue-400 ring-opacity-50 shadow-lg scale-105'
                  : 'hover:shadow-xl hover:scale-105 hover:-translate-y-1'
              } ${
                theme === 'color' ? 'hover:brightness-110 hover:saturate-110 hover:shadow-2xl' : ''
              }`}
            >
              <div className="flex items-center justify-between mb-3">
                <div className="flex-1">
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
              
              {/* Action Buttons */}
              <div className="flex gap-2 pt-3 border-t border-gray-200">
                <div className="flex-1 text-xs text-gray-500 flex items-center justify-center gap-1">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                  <span>Click anywhere to explore</span>
                </div>
                <button
                  onClick={(e) => handleStartChat(agent, e)}
                  className="px-3 py-2 text-xs font-medium rounded-lg bg-blue-100 text-blue-700 border border-blue-200 hover:bg-blue-200 hover:border-blue-300 transition-all duration-200 flex items-center justify-center gap-1"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                  </svg>
                  <span>Start Chat</span>
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Right Content Area */}
      <div className="flex-1 overflow-y-auto">
        {selectedAgent ? (
          <div className="p-6">
            {/* Category Filter Bar - Sticky */}
            <div className="mb-6">
              <div className="bg-white p-4 rounded-lg border border-gray-200 shadow-sm">
                <CategoryFilter
                  selectedCategories={selectedCategories}
                  onCategoryToggle={handleCategoryToggle}
                  onClearAll={handleClearAllCategories}
                  className="compact"
                />
              </div>
            </div>
            
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
                      <div className="space-y-3">
                        {selectedAgent.metadata.capabilities.map((capability, index) => (
                          <div
                            key={index}
                            className="flex items-start gap-3 p-3 bg-blue-50 rounded-lg border border-blue-100"
                          >
                            <div className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></div>
                            <div>
                              <h5 className="font-medium text-blue-900 mb-1">{capability}</h5>
                              <p className="text-sm text-blue-700">
                                Specialized expertise in {capability.toLowerCase().replace(/[&-]/g, ' and ').replace(/  +/g, ' ')} with advanced proficiency and proven results.
                              </p>
                            </div>
                          </div>
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
                    className={`p-4 rounded-lg border transition-all duration-200 text-left group ${
                      selectedComponent?.path === component.path 
                        ? 'border-blue-300 shadow-md bg-blue-100 text-blue-900' 
                        : 'border-gray-200 bg-white hover:border-gray-300 hover:bg-gray-50'
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
                        component.type === 'memory' ? 'bg-blue-300' :
                        component.type === 'learning' ? 'bg-green-300' :
                        component.type === 'readme' ? 'bg-purple-300' :
                        component.type === 'protocol' ? 'bg-orange-300' :
                        'bg-gray-300'
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
                      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-300"></div>
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
          <div className="p-6 h-full overflow-y-auto">
            {/* Compact Header with Filters */}
            <div className="mb-6">
              <div className="mb-4">
                <div className="flex items-center space-x-3">
                  <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                    <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
                    </svg>
                  </div>
                  <div>
                    <h1 className="text-2xl font-bold text-gray-800">AI Agent Directory</h1>
                    <p className="text-sm text-gray-600">Discover and interact with {agents.length} specialized AI agents</p>
                  </div>
                </div>
              </div>
              
              {/* Integrated Category Filter */}
              <div className="bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
                <CategoryFilter
                  selectedCategories={selectedCategories}
                  onCategoryToggle={handleCategoryToggle}
                  onClearAll={handleClearAllCategories}
                  className="directory-main"
                />
              </div>
            </div>


            {/* Getting Started Section */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
              <div className="bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
                <div className="flex items-center mb-3">
                  <div className="w-8 h-8 pastel-icon-blue rounded-lg flex items-center justify-center mr-2">
                    <svg className="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                    </svg>
                  </div>
                  <h3 className="text-lg font-semibold text-pastel-blue">How to Get Started</h3>
                </div>
                <div className="space-y-3">
                  <div className="flex items-start">
                    <div className="w-6 h-6 pastel-icon-blue text-white rounded-full flex items-center justify-center text-xs font-bold mr-3 mt-0.5">1</div>
                    <div>
                      <p className="font-medium text-gray-700">Browse Agents</p>
                      <p className="text-sm text-gray-600">Click on any agent card to explore its capabilities and components</p>
                    </div>
                  </div>
                  <div className="flex items-start">
                    <div className="w-6 h-6 pastel-icon-green text-white rounded-full flex items-center justify-center text-xs font-bold mr-3 mt-0.5">2</div>
                    <div>
                      <p className="font-medium text-gray-700">Filter by Category</p>
                      <p className="text-sm text-gray-600">Use category filters to find agents specialized for your needs</p>
                    </div>
                  </div>
                  <div className="flex items-start">
                    <div className="w-6 h-6 pastel-icon-pink text-white rounded-full flex items-center justify-center text-xs font-bold mr-3 mt-0.5">3</div>
                    <div>
                      <p className="font-medium text-gray-700">Start Chatting</p>
                      <p className="text-sm text-gray-600">Click "Start Chat" to begin interacting with any agent</p>
                    </div>
                  </div>
                </div>
              </div>

              <div className="bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
                <div className="flex items-center mb-3">
                  <div className="w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center mr-2">
                    <svg className="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                  </div>
                  <h3 className="text-lg font-semibold text-pastel-green">Quick Actions</h3>
                </div>
                <div className="space-y-2">
                  <button className="w-full p-2 pastel-blue rounded-lg text-left transition-colors group">
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium text-gray-700">View Most Popular Agents</p>
                        <p className="text-sm text-gray-600">See which agents are used most frequently</p>
                      </div>
                      <svg className="w-5 h-5 text-gray-500 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </div>
                  </button>
                  <button className="w-full p-2 pastel-purple rounded-lg text-left transition-colors group">
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium text-gray-700">Switch to Grid View</p>
                        <p className="text-sm text-gray-600">See all agents in a visual grid layout</p>
                      </div>
                      <svg className="w-5 h-5 text-gray-500 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </div>
                  </button>
                  <button className="w-full p-2 pastel-green rounded-lg text-left transition-colors group">
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium text-gray-700">Start Instant Chat</p>
                        <p className="text-sm text-gray-600">Jump directly to the chat interface</p>
                      </div>
                      <svg className="w-5 h-5 text-gray-500 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </div>
                  </button>
                </div>
              </div>
            </div>

            {/* Top Agents Preview */}
            <div className="bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-pastel-purple">Most Popular Agents</h3>
                <span className="text-sm text-gray-500">Click any agent to explore</span>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                {agents
                  .sort((a, b) => b.metadata.usage_count - a.metadata.usage_count)
                  .slice(0, 6)
                  .map((agent) => (
                    <div
                      key={agent.name}
                      onClick={() => handleAgentClick(agent)}
                      className="p-3 border border-gray-200 rounded-lg hover:border-purple-200 hover:bg-purple-50 cursor-pointer transition-all duration-200 group pastel-pink"
                    >
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="font-medium text-pastel-pink group-hover:text-pastel-purple">{agent.name}</h4>
                        <div className="w-3 h-3 rounded-full bg-green-400" />
                      </div>
                      <p className="text-sm text-gray-600 mb-3 line-clamp-2">{agent.subtitle}</p>
                      <div className="flex items-center justify-between text-xs text-gray-500">
                        <span>{agent.metadata.usage_count} uses</span>
                        <span>{agent.components.length} components</span>
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          </div>
        )}
      </div>

    </div>
  );
};