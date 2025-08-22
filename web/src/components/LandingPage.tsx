import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Agent } from '../types/agent';
import { getAgents } from '../services/agentService';

export const LandingPage: React.FC = () => {
  const navigate = useNavigate();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadAgents();
  }, []);

  const loadAgents = async () => {
    try {
      const agentData = await getAgents();
      setAgents(agentData);
    } catch (error) {
      console.error('Failed to load agents:', error);
    } finally {
      setLoading(false);
    }
  };

  const totalUsage = agents.reduce((sum, agent) => sum + agent.metadata.usage_count, 0);
  const totalComponents = agents.reduce((sum, agent) => sum + agent.components.length, 0);
  const activeAgents = agents.filter(agent => {
    const daysSinceUsed = Math.floor((Date.now() - new Date(agent.metadata.last_used).getTime()) / (1000 * 60 * 60 * 24));
    return daysSinceUsed <= 30;
  }).length;

  const features = [
    {
      title: "110+ Specialized Agents",
      description: "Comprehensive collection of AI agents for development, design, analysis, and specialized tasks",
      icon: "🤖",
      stats: `${agents.length} active agents`
    },
    {
      title: "Multi-View Interface",
      description: "Switch between Directory, Grid, and Chat views to interact with agents in the way that works best",
      icon: "📱",
      stats: "3 interface modes"
    },
    {
      title: "Real-time Collaboration",
      description: "Start conversations with any agent instantly and get specialized assistance for your projects",
      icon: "💬",
      stats: `${totalUsage.toLocaleString()} total interactions`
    },
    {
      title: "Component System",
      description: "Each agent comes with memory, learning systems, and comprehensive documentation",
      icon: "🧩",
      stats: `${totalComponents} total components`
    },
    {
      title: "Smart Categories",
      description: "Filter agents by category - development, testing, design, architecture, and specialized domains",
      icon: "🏷️",
      stats: "8+ categories"
    },
    {
      title: "Continuous Learning",
      description: "Agents adapt and improve through usage, maintaining memory and learning from interactions",
      icon: "📚",
      stats: `${activeAgents} recently active`
    }
  ];

  const categories = [
    { name: "Development", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('development') || c.includes('coding'))).length, color: "bg-blue-500" },
    { name: "Testing", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('testing') || c.includes('qa'))).length, color: "bg-green-500" },
    { name: "Architecture", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('architecture') || c.includes('design'))).length, color: "bg-purple-500" },
    { name: "Design", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('ui') || c.includes('visual'))).length, color: "bg-pink-500" },
    { name: "Business", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('business') || c.includes('strategy'))).length, color: "bg-orange-500" },
    { name: "Analysis", count: agents.filter(a => a.metadata.capabilities.some(c => c.includes('analysis') || c.includes('research'))).length, color: "bg-cyan-500" },
  ];

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="bg-gray-50 min-h-screen">
      {/* Hero Section */}
      <div className="bg-gradient-to-r from-blue-600 to-purple-700 text-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
          <div className="text-center">
            <h1 className="text-5xl font-bold mb-6">
              Collaborative Intelligence
            </h1>
            <p className="text-xl mb-8 text-blue-100 max-w-3xl mx-auto">
              Access 110+ specialized AI agents designed for development, design, analysis, and business intelligence. 
              Each agent brings unique capabilities and continuous learning to accelerate your projects.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <button
                onClick={() => navigate('/grid')}
                className="bg-white text-blue-600 px-8 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors flex items-center justify-center gap-2"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                </svg>
                Explore Agent Grid
              </button>
              <button
                onClick={() => navigate('/directory')}
                className="bg-transparent border-2 border-white text-white px-8 py-3 rounded-lg font-semibold hover:bg-white hover:text-blue-600 transition-colors flex items-center justify-center gap-2"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 5a2 2 0 012-2h4a2 2 0 012 2v.01" />
                </svg>
                Browse Directory
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Stats Section */}
      <div className="bg-white py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="text-4xl font-bold text-blue-600 mb-2">{agents.length}</div>
              <div className="text-gray-600">AI Agents</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-green-600 mb-2">{totalUsage.toLocaleString()}</div>
              <div className="text-gray-600">Total Interactions</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-purple-600 mb-2">{totalComponents}</div>
              <div className="text-gray-600">Components</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-orange-600 mb-2">{activeAgents}</div>
              <div className="text-gray-600">Recently Active</div>
            </div>
          </div>
        </div>
      </div>

      {/* Features Section */}
      <div className="py-16 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-gray-900 mb-4">Platform Features</h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Discover what makes our collaborative intelligence platform the perfect solution for your project needs
            </p>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => (
              <div key={index} className="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                <div className="text-3xl mb-4">{feature.icon}</div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">{feature.title}</h3>
                <p className="text-gray-600 mb-4 leading-relaxed">{feature.description}</p>
                <div className="text-sm font-medium text-blue-600">{feature.stats}</div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Categories Section */}
      <div className="py-16 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-gray-900 mb-4">Agent Categories</h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Our agents are organized into specialized categories to help you find the right expertise quickly
            </p>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            {categories.map((category, index) => (
              <button
                key={index}
                onClick={() => navigate('/grid')}
                className="p-4 bg-gray-50 rounded-xl hover:shadow-md transition-all duration-200 group text-center"
              >
                <div className={`w-12 h-12 ${category.color} rounded-full mx-auto mb-3 opacity-80 group-hover:opacity-100 transition-opacity`}></div>
                <div className="font-semibold text-gray-900 mb-1">{category.name}</div>
                <div className="text-sm text-gray-500">{category.count} agents</div>
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* CTA Section */}
      <div className="bg-gradient-to-r from-gray-800 to-gray-900 text-white py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl font-bold mb-4">Ready to Get Started?</h2>
          <p className="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
            Choose your preferred interface and start collaborating with our specialized AI agents today
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <button
              onClick={() => navigate('/grid')}
              className="bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors"
            >
              Start with Grid View
            </button>
            <button
              onClick={() => navigate('/directory')}
              className="bg-transparent border-2 border-gray-400 text-white px-8 py-3 rounded-lg font-semibold hover:bg-gray-400 hover:text-gray-900 transition-colors"
            >
              Browse Directory
            </button>
            <button
              onClick={() => navigate('/chat')}
              className="bg-green-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-green-700 transition-colors"
            >
              Start Chatting
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};