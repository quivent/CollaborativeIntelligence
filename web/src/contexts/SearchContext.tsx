import React, { createContext, useContext, useState, ReactNode } from 'react';
import { Agent } from '../types/agent';

interface SearchContextType {
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  filterAgents: (agents: Agent[]) => Agent[];
}

const SearchContext = createContext<SearchContextType | undefined>(undefined);

export const useSearch = () => {
  const context = useContext(SearchContext);
  if (context === undefined) {
    throw new Error('useSearch must be used within a SearchProvider');
  }
  return context;
};

interface SearchProviderProps {
  children: ReactNode;
}

export const SearchProvider: React.FC<SearchProviderProps> = ({ children }) => {
  const [searchQuery, setSearchQuery] = useState('');

  const filterAgents = (agents: Agent[]): Agent[] => {
    if (!searchQuery.trim()) {
      return agents;
    }

    const query = searchQuery.toLowerCase();
    
    return agents.filter(agent => {
      // Search in agent name
      if (agent.name.toLowerCase().includes(query)) {
        return true;
      }
      
      // Search in agent subtitle
      if (agent.subtitle && agent.subtitle.toLowerCase().includes(query)) {
        return true;
      }
      
      // Search in agent description
      if (agent.metadata.description.toLowerCase().includes(query)) {
        return true;
      }
      
      // Search in capabilities
      if (agent.metadata.capabilities.some(cap => 
        cap.toLowerCase().includes(query)
      )) {
        return true;
      }
      
      // Search in component names
      if (agent.components.some(comp => 
        comp.name.toLowerCase().includes(query)
      )) {
        return true;
      }
      
      return false;
    });
  };

  const value = {
    searchQuery,
    setSearchQuery,
    filterAgents,
  };

  return (
    <SearchContext.Provider value={value}>
      {children}
    </SearchContext.Provider>
  );
};