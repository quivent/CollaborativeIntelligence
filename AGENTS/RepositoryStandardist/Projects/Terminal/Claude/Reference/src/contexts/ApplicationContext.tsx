import React, { createContext, useContext, useReducer, useCallback } from 'react';

// Application modes
export type ApplicationMode = 'terminal' | 'chat' | 'session' | 'settings';

// Theme options
export type ThemeOption = 'light' | 'dark' | 'system';

// Application Context State Interface
export interface ApplicationContextState {
  currentMode: ApplicationMode;
  previousMode: ApplicationMode | null;
  theme: ThemeOption;
  isFullscreen: boolean;
  isMenuOpen: boolean;
  sessionId: string | null;
  lastActivity: number;
}

// Application Context Actions
type ApplicationAction = 
  | { type: 'SET_MODE'; mode: ApplicationMode }
  | { type: 'SET_THEME'; theme: ThemeOption }
  | { type: 'TOGGLE_FULLSCREEN' }
  | { type: 'SET_FULLSCREEN'; isFullscreen: boolean }
  | { type: 'TOGGLE_MENU' }
  | { type: 'SET_MENU_OPEN'; isOpen: boolean }
  | { type: 'SET_SESSION_ID'; sessionId: string | null }
  | { type: 'UPDATE_ACTIVITY' };

// Initial state
const initialState: ApplicationContextState = {
  currentMode: 'terminal',
  previousMode: null,
  theme: 'system',
  isFullscreen: false,
  isMenuOpen: false,
  sessionId: null,
  lastActivity: Date.now(),
};

// Application reducer
const applicationReducer = (
  state: ApplicationContextState,
  action: ApplicationAction
): ApplicationContextState => {
  switch (action.type) {
    case 'SET_MODE':
      return {
        ...state,
        previousMode: state.currentMode,
        currentMode: action.mode,
        lastActivity: Date.now(),
      };
    
    case 'SET_THEME':
      return {
        ...state,
        theme: action.theme,
      };
    
    case 'TOGGLE_FULLSCREEN':
      return {
        ...state,
        isFullscreen: !state.isFullscreen,
        lastActivity: Date.now(),
      };
    
    case 'SET_FULLSCREEN':
      return {
        ...state,
        isFullscreen: action.isFullscreen,
        lastActivity: Date.now(),
      };
    
    case 'TOGGLE_MENU':
      return {
        ...state,
        isMenuOpen: !state.isMenuOpen,
        lastActivity: Date.now(),
      };
    
    case 'SET_MENU_OPEN':
      return {
        ...state,
        isMenuOpen: action.isOpen,
        lastActivity: Date.now(),
      };
    
    case 'SET_SESSION_ID':
      return {
        ...state,
        sessionId: action.sessionId,
        lastActivity: Date.now(),
      };
    
    case 'UPDATE_ACTIVITY':
      return {
        ...state,
        lastActivity: Date.now(),
      };
    
    default:
      return state;
  }
};

// Context interface
interface ApplicationContextProps {
  state: ApplicationContextState;
  setMode: (mode: ApplicationMode) => void;
  setTheme: (theme: ThemeOption) => void;
  toggleFullscreen: () => void;
  setFullscreen: (isFullscreen: boolean) => void;
  toggleMenu: () => void;
  setMenuOpen: (isOpen: boolean) => void;
  setSessionId: (sessionId: string | null) => void;
  updateActivity: () => void;
}

// Create context
const ApplicationContext = createContext<ApplicationContextProps | undefined>(undefined);

// Application Provider component
export const ApplicationProvider: React.FC<{
  children: React.ReactNode;
  initialState?: Partial<ApplicationContextState>;
}> = ({ children, initialState: initialStateOverrides }) => {
  // Merge default initial state with any overrides
  const mergedInitialState = {
    ...initialState,
    ...initialStateOverrides,
  };
  
  // Set up reducer
  const [state, dispatch] = useReducer(applicationReducer, mergedInitialState);
  
  // Actions
  const setMode = useCallback((mode: ApplicationMode) => {
    dispatch({ type: 'SET_MODE', mode });
  }, []);
  
  const setTheme = useCallback((theme: ThemeOption) => {
    dispatch({ type: 'SET_THEME', theme });
  }, []);
  
  const toggleFullscreen = useCallback(() => {
    dispatch({ type: 'TOGGLE_FULLSCREEN' });
  }, []);
  
  const setFullscreen = useCallback((isFullscreen: boolean) => {
    dispatch({ type: 'SET_FULLSCREEN', isFullscreen });
  }, []);
  
  const toggleMenu = useCallback(() => {
    dispatch({ type: 'TOGGLE_MENU' });
  }, []);
  
  const setMenuOpen = useCallback((isOpen: boolean) => {
    dispatch({ type: 'SET_MENU_OPEN', isOpen });
  }, []);
  
  const setSessionId = useCallback((sessionId: string | null) => {
    dispatch({ type: 'SET_SESSION_ID', sessionId });
  }, []);
  
  const updateActivity = useCallback(() => {
    dispatch({ type: 'UPDATE_ACTIVITY' });
  }, []);
  
  // Combine state and actions to pass through the provider
  const contextValue: ApplicationContextProps = {
    state,
    setMode,
    setTheme,
    toggleFullscreen,
    setFullscreen,
    toggleMenu,
    setMenuOpen,
    setSessionId,
    updateActivity,
  };
  
  return (
    <ApplicationContext.Provider value={contextValue}>
      {children}
    </ApplicationContext.Provider>
  );
};

// Custom hook for accessing the application context
export const useApplication = (): ApplicationContextProps => {
  const context = useContext(ApplicationContext);
  
  if (context === undefined) {
    throw new Error('useApplication must be used within an ApplicationProvider');
  }
  
  return context;
};

export default ApplicationContext;