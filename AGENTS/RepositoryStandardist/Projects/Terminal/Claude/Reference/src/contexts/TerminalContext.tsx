import React, { createContext, useContext, useReducer, useCallback } from 'react';
import { TerminalHistoryEntry, TerminalConfig } from '../types/terminal';

// Terminal Context State Interface
export interface TerminalContextState {
  history: TerminalHistoryEntry[];
  isProcessing: boolean;
  currentInput: string;
  inputHistory: string[];
  config: TerminalConfig;
}

// Terminal Context Actions
type TerminalAction = 
  | { type: 'ADD_HISTORY_ENTRY'; entry: TerminalHistoryEntry }
  | { type: 'ADD_HISTORY_ENTRIES'; entries: TerminalHistoryEntry[] }
  | { type: 'CLEAR_HISTORY' }
  | { type: 'SET_PROCESSING'; isProcessing: boolean }
  | { type: 'SET_CURRENT_INPUT'; input: string }
  | { type: 'ADD_INPUT_HISTORY'; input: string }
  | { type: 'UPDATE_CONFIG'; config: Partial<TerminalConfig> };

// Default configuration
const defaultConfig: TerminalConfig = {
  promptString: '> ',
  maxHistoryLength: 1000,
  autoScroll: true,
  enableHistoryNavigation: true,
  enableSyntaxHighlighting: true,
  defaultCodeLanguage: 'javascript',
};

// Initial state
const initialState: TerminalContextState = {
  history: [],
  isProcessing: false,
  currentInput: '',
  inputHistory: [],
  config: defaultConfig,
};

// Terminal reducer
const terminalReducer = (
  state: TerminalContextState,
  action: TerminalAction
): TerminalContextState => {
  switch (action.type) {
    case 'ADD_HISTORY_ENTRY':
      return {
        ...state,
        history: [
          ...state.history,
          action.entry
        ].slice(-state.config.maxHistoryLength),
      };
    
    case 'ADD_HISTORY_ENTRIES':
      return {
        ...state,
        history: [
          ...state.history,
          ...action.entries
        ].slice(-state.config.maxHistoryLength),
      };
    
    case 'CLEAR_HISTORY':
      return {
        ...state,
        history: [],
      };
    
    case 'SET_PROCESSING':
      return {
        ...state,
        isProcessing: action.isProcessing,
      };
    
    case 'SET_CURRENT_INPUT':
      return {
        ...state,
        currentInput: action.input,
      };
    
    case 'ADD_INPUT_HISTORY':
      // Only add to history if non-empty and not a duplicate of the most recent entry
      if (
        !action.input.trim() || 
        (state.inputHistory.length > 0 && state.inputHistory[0] === action.input)
      ) {
        return state;
      }
      
      return {
        ...state,
        inputHistory: [
          action.input,
          ...state.inputHistory.slice(0, 49), // Keep max 50 entries
        ],
      };
    
    case 'UPDATE_CONFIG':
      return {
        ...state,
        config: {
          ...state.config,
          ...action.config,
        },
      };
    
    default:
      return state;
  }
};

// Context interface
interface TerminalContextProps {
  state: TerminalContextState;
  addHistoryEntry: (entry: TerminalHistoryEntry) => void;
  addHistoryEntries: (entries: TerminalHistoryEntry[]) => void;
  clearHistory: () => void;
  setProcessing: (isProcessing: boolean) => void;
  setCurrentInput: (input: string) => void;
  addInputHistory: (input: string) => void;
  updateConfig: (config: Partial<TerminalConfig>) => void;
}

// Create context
const TerminalContext = createContext<TerminalContextProps | undefined>(undefined);

// Terminal Provider component
export const TerminalProvider: React.FC<{
  children: React.ReactNode;
  initialState?: Partial<TerminalContextState>;
}> = ({ children, initialState: initialStateOverrides }) => {
  // Merge default initial state with any overrides
  const mergedInitialState = {
    ...initialState,
    ...initialStateOverrides,
  };
  
  // Set up reducer
  const [state, dispatch] = useReducer(terminalReducer, mergedInitialState);
  
  // Actions
  const addHistoryEntry = useCallback((entry: TerminalHistoryEntry) => {
    dispatch({ type: 'ADD_HISTORY_ENTRY', entry });
  }, []);
  
  const addHistoryEntries = useCallback((entries: TerminalHistoryEntry[]) => {
    dispatch({ type: 'ADD_HISTORY_ENTRIES', entries });
  }, []);
  
  const clearHistory = useCallback(() => {
    dispatch({ type: 'CLEAR_HISTORY' });
  }, []);
  
  const setProcessing = useCallback((isProcessing: boolean) => {
    dispatch({ type: 'SET_PROCESSING', isProcessing });
  }, []);
  
  const setCurrentInput = useCallback((input: string) => {
    dispatch({ type: 'SET_CURRENT_INPUT', input });
  }, []);
  
  const addInputHistory = useCallback((input: string) => {
    dispatch({ type: 'ADD_INPUT_HISTORY', input });
  }, []);
  
  const updateConfig = useCallback((config: Partial<TerminalConfig>) => {
    dispatch({ type: 'UPDATE_CONFIG', config });
  }, []);
  
  // Combine state and actions to pass through the provider
  const contextValue: TerminalContextProps = {
    state,
    addHistoryEntry,
    addHistoryEntries,
    clearHistory,
    setProcessing,
    setCurrentInput,
    addInputHistory,
    updateConfig,
  };
  
  return (
    <TerminalContext.Provider value={contextValue}>
      {children}
    </TerminalContext.Provider>
  );
};

// Custom hook for accessing the terminal context
export const useTerminal = (): TerminalContextProps => {
  const context = useContext(TerminalContext);
  
  if (context === undefined) {
    throw new Error('useTerminal must be used within a TerminalProvider');
  }
  
  return context;
};

export default TerminalContext;