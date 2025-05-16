import React, { useState, useEffect, useRef } from 'react';
import BaseTerminal from '../base/BaseTerminal';
import TerminalHeader from './TerminalHeader';
import TerminalFooter from './TerminalFooter';
import { TerminalHistoryEntry, TerminalConfig } from '../../types/terminal';
import '../../styles/TerminalContainer.css';

export interface TerminalContainerProps {
  /**
   * Initial history entries to populate the terminal with
   */
  initialHistory?: TerminalHistoryEntry[];
  
  /**
   * Terminal configuration options
   */
  config?: Partial<TerminalConfig>;
  
  /**
   * Function to execute commands
   */
  commandExecutor?: (command: string) => Promise<TerminalHistoryEntry | TerminalHistoryEntry[]>;
  
  /**
   * Event handler for when terminal state changes
   */
  onStateChange?: (history: TerminalHistoryEntry[]) => void;
  
  /**
   * Custom class name for styling
   */
  className?: string;
  
  /**
   * Whether the terminal header should be visible
   */
  showHeader?: boolean;
  
  /**
   * Whether the terminal footer should be visible
   */
  showFooter?: boolean;
  
  /**
   * Terminal mode (fullscreen, split, embedded, etc.)
   */
  mode?: 'fullscreen' | 'split' | 'embedded' | 'minimal';
}

/**
 * TerminalContainer wraps the BaseTerminal with layout components
 * and manages the terminal's state and configuration.
 */
export const TerminalContainer: React.FC<TerminalContainerProps> = ({
  initialHistory = [],
  config = {},
  commandExecutor,
  onStateChange,
  className = '',
  showHeader = true,
  showFooter = true,
  mode = 'fullscreen',
}) => {
  // Default configuration with overrides from props
  const terminalConfig: TerminalConfig = {
    promptString: '> ',
    maxHistoryLength: 1000,
    autoScroll: true,
    enableHistoryNavigation: true,
    enableSyntaxHighlighting: true,
    defaultCodeLanguage: 'javascript',
    ...config,
  };
  
  // Terminal state
  const [history, setHistory] = useState<TerminalHistoryEntry[]>(initialHistory);
  const [isProcessing, setIsProcessing] = useState<boolean>(false);
  const [isFullscreen, setIsFullscreen] = useState<boolean>(mode === 'fullscreen');
  
  // Ref for the BaseTerminal component
  const terminalRef = useRef<any>(null);
  
  // Handle command execution
  const handleCommandExecute = async (command: string): Promise<void> => {
    if (!commandExecutor) return;
    
    setIsProcessing(true);
    
    try {
      const result = await commandExecutor(command);
      
      if (Array.isArray(result)) {
        // Add multiple entries
        const newHistory = [...history, ...result];
        
        // Respect max history length
        const trimmedHistory = newHistory.slice(
          Math.max(0, newHistory.length - terminalConfig.maxHistoryLength)
        );
        
        setHistory(trimmedHistory);
      } else {
        // Add single entry
        const newHistory = [...history, result];
        
        // Respect max history length
        const trimmedHistory = newHistory.slice(
          Math.max(0, newHistory.length - terminalConfig.maxHistoryLength)
        );
        
        setHistory(trimmedHistory);
      }
    } catch (error) {
      // Add error to history
      const errorEntry: TerminalHistoryEntry = {
        type: 'error',
        content: `Error: ${error instanceof Error ? error.message : String(error)}`,
        timestamp: new Date().toISOString(),
      };
      
      setHistory(prev => [...prev, errorEntry]);
    } finally {
      setIsProcessing(false);
    }
  };
  
  // Handle history changes
  const handleHistoryChange = (newHistory: TerminalHistoryEntry[]) => {
    setHistory(newHistory);
    
    if (onStateChange) {
      onStateChange(newHistory);
    }
  };
  
  // Handle clear command
  const handleClear = () => {
    if (terminalRef.current) {
      terminalRef.current.clearHistory();
      setHistory([]);
    }
  };
  
  // Toggle fullscreen mode
  const toggleFullscreen = () => {
    setIsFullscreen(prev => !prev);
  };
  
  // Update mode when prop changes
  useEffect(() => {
    setIsFullscreen(mode === 'fullscreen');
  }, [mode]);
  
  return (
    <div 
      className={`terminal-container terminal-mode-${mode} ${isFullscreen ? 'fullscreen' : ''} ${className}`}
    >
      {showHeader && (
        <TerminalHeader 
          onClear={handleClear}
          onToggleFullscreen={toggleFullscreen}
          isFullscreen={isFullscreen}
          isProcessing={isProcessing}
        />
      )}
      
      <div className="terminal-container-content">
        <BaseTerminal
          ref={terminalRef}
          initialHistory={history}
          isAcceptingInput={!isProcessing}
          promptString={terminalConfig.promptString}
          onCommandExecute={handleCommandExecute}
          onStateChange={handleHistoryChange}
          className="terminal-container-base"
        />
      </div>
      
      {showFooter && (
        <TerminalFooter
          historyLength={history.length}
          isProcessing={isProcessing}
        />
      )}
    </div>
  );
};

export default TerminalContainer;