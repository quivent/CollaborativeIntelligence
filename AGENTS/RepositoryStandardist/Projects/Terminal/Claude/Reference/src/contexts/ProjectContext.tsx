import React, { createContext, useContext, useReducer, useCallback } from 'react';

// Project Types
export interface Project {
  id: string;
  name: string;
  path: string;
  lastOpened: number;
  description?: string;
  tags?: string[];
  favorite?: boolean;
}

// Project Context State Interface
export interface ProjectContextState {
  projects: Record<string, Project>;
  activeProjectId: string | null;
  recentProjects: string[]; // Array of project IDs
  isLoading: boolean;
  error: string | null;
}

// Project Context Actions
type ProjectAction = 
  | { type: 'SET_PROJECTS'; projects: Record<string, Project> }
  | { type: 'ADD_PROJECT'; project: Project }
  | { type: 'UPDATE_PROJECT'; id: string; updates: Partial<Project> }
  | { type: 'REMOVE_PROJECT'; id: string }
  | { type: 'SET_ACTIVE_PROJECT'; id: string | null }
  | { type: 'SET_LOADING'; isLoading: boolean }
  | { type: 'SET_ERROR'; error: string | null }
  | { type: 'UPDATE_RECENT_PROJECTS'; id: string };

// Initial state
const initialState: ProjectContextState = {
  projects: {},
  activeProjectId: null,
  recentProjects: [],
  isLoading: false,
  error: null,
};

// Maximum number of recent projects to track
const MAX_RECENT_PROJECTS = 10;

// Project reducer
const projectReducer = (
  state: ProjectContextState,
  action: ProjectAction
): ProjectContextState => {
  switch (action.type) {
    case 'SET_PROJECTS':
      return {
        ...state,
        projects: action.projects,
        isLoading: false,
      };
    
    case 'ADD_PROJECT':
      return {
        ...state,
        projects: {
          ...state.projects,
          [action.project.id]: action.project,
        },
        // Add to recent projects if it's a new project
        recentProjects: state.recentProjects.includes(action.project.id)
          ? state.recentProjects
          : [action.project.id, ...state.recentProjects].slice(0, MAX_RECENT_PROJECTS),
      };
    
    case 'UPDATE_PROJECT':
      if (!state.projects[action.id]) {
        return state;
      }
      
      return {
        ...state,
        projects: {
          ...state.projects,
          [action.id]: {
            ...state.projects[action.id],
            ...action.updates,
            // Update the lastOpened timestamp if not explicitly provided
            lastOpened: action.updates.lastOpened || Date.now(),
          },
        },
      };
    
    case 'REMOVE_PROJECT':
      const { [action.id]: removed, ...remainingProjects } = state.projects;
      
      return {
        ...state,
        projects: remainingProjects,
        activeProjectId: state.activeProjectId === action.id ? null : state.activeProjectId,
        recentProjects: state.recentProjects.filter(id => id !== action.id),
      };
    
    case 'SET_ACTIVE_PROJECT':
      // Update recent projects when setting a new active project
      const updatedRecentProjects = action.id && state.projects[action.id]
        ? [action.id, ...state.recentProjects.filter(id => id !== action.id)].slice(0, MAX_RECENT_PROJECTS)
        : state.recentProjects;
      
      return {
        ...state,
        activeProjectId: action.id,
        recentProjects: updatedRecentProjects,
      };
    
    case 'SET_LOADING':
      return {
        ...state,
        isLoading: action.isLoading,
      };
    
    case 'SET_ERROR':
      return {
        ...state,
        error: action.error,
        isLoading: false,
      };
    
    case 'UPDATE_RECENT_PROJECTS':
      if (!state.projects[action.id]) {
        return state;
      }
      
      return {
        ...state,
        recentProjects: [
          action.id,
          ...state.recentProjects.filter(id => id !== action.id),
        ].slice(0, MAX_RECENT_PROJECTS),
      };
    
    default:
      return state;
  }
};

// Context interface
interface ProjectContextProps {
  state: ProjectContextState;
  setProjects: (projects: Record<string, Project>) => void;
  addProject: (project: Project) => void;
  updateProject: (id: string, updates: Partial<Project>) => void;
  removeProject: (id: string) => void;
  setActiveProject: (id: string | null) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  openProject: (id: string) => void;
}

// Create context
const ProjectContext = createContext<ProjectContextProps | undefined>(undefined);

// Project Provider component
export const ProjectProvider: React.FC<{
  children: React.ReactNode;
  initialState?: Partial<ProjectContextState>;
}> = ({ children, initialState: initialStateOverrides }) => {
  // Merge default initial state with any overrides
  const mergedInitialState = {
    ...initialState,
    ...initialStateOverrides,
  };
  
  // Set up reducer
  const [state, dispatch] = useReducer(projectReducer, mergedInitialState);
  
  // Actions
  const setProjects = useCallback((projects: Record<string, Project>) => {
    dispatch({ type: 'SET_PROJECTS', projects });
  }, []);
  
  const addProject = useCallback((project: Project) => {
    // Ensure lastOpened is set if not provided
    const projectWithTimestamp = {
      ...project,
      lastOpened: project.lastOpened || Date.now(),
    };
    dispatch({ type: 'ADD_PROJECT', project: projectWithTimestamp });
  }, []);
  
  const updateProject = useCallback((id: string, updates: Partial<Project>) => {
    dispatch({ type: 'UPDATE_PROJECT', id, updates });
  }, []);
  
  const removeProject = useCallback((id: string) => {
    dispatch({ type: 'REMOVE_PROJECT', id });
  }, []);
  
  const setActiveProject = useCallback((id: string | null) => {
    dispatch({ type: 'SET_ACTIVE_PROJECT', id });
  }, []);
  
  const setLoading = useCallback((isLoading: boolean) => {
    dispatch({ type: 'SET_LOADING', isLoading });
  }, []);
  
  const setError = useCallback((error: string | null) => {
    dispatch({ type: 'SET_ERROR', error });
  }, []);
  
  const openProject = useCallback((id: string) => {
    if (state.projects[id]) {
      dispatch({ type: 'SET_ACTIVE_PROJECT', id });
      dispatch({ type: 'UPDATE_PROJECT', id, updates: { lastOpened: Date.now() } });
    }
  }, [state.projects]);
  
  // Combine state and actions to pass through the provider
  const contextValue: ProjectContextProps = {
    state,
    setProjects,
    addProject,
    updateProject,
    removeProject,
    setActiveProject,
    setLoading,
    setError,
    openProject,
  };
  
  return (
    <ProjectContext.Provider value={contextValue}>
      {children}
    </ProjectContext.Provider>
  );
};

// Custom hook for accessing the project context
export const useProject = (): ProjectContextProps => {
  const context = useContext(ProjectContext);
  
  if (context === undefined) {
    throw new Error('useProject must be used within a ProjectProvider');
  }
  
  return context;
};

export default ProjectContext;