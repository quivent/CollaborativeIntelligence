import React, { useState, useEffect, useRef } from 'react';
import { TerminalHistoryEntry } from '../../types/terminal';

export interface BaseTerminalProps {
  /**
   * Initial command history to populate the terminal with
   */
  initialHistory?: TerminalHistoryEntry[];
  
  /**
   * Whether the terminal is currently accepting input
   */
  isAcceptingInput?: boolean;
  
  /**
   * Custom prompt string to display before input
   */
  promptString?: string;
  
  /**
   * Handler for when a command is executed
   */
  onCommandExecute?: (command: string) => Promise<void>;
  
  /**
   * Handler for when the terminal state changes
   */
  onStateChange?: (history: TerminalHistoryEntry[]) => void;
  
  /**
   * Custom class names to apply to the terminal container
   */
  className?: string;
  
  /**
   * Children to render inside the terminal (for composition)
   */
  children?: React.ReactNode;
}

/**
 * BaseTerminal provides core terminal functionality without any styling or
 * layout assumptions. It handles command history, input management, and
 * the basic terminal interaction flow.
 */
export const BaseTerminal: React.FC<BaseTerminalProps> = ({
  initialHistory = [],
  isAcceptingInput = true,
  promptString = '> ',
  onCommandExecute,
  onStateChange,
  className = '',
  children,
}) => {
  // Terminal state
  const [history, setHistory] = useState<TerminalHistoryEntry[]>(initialHistory);
  const [currentInput, setCurrentInput] = useState<string>('');
  const [isProcessing, setIsProcessing] = useState<boolean>(false);
  
  // Input history navigation
  const [historyPosition, setHistoryPosition] = useState<number>(-1);
  const [inputHistory, setInputHistory] = useState<string[]>([]);
  
  // Refs for DOM manipulation
  const terminalRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  
  // Auto-focus the input field when the component mounts
  useEffect(() => {
    if (isAcceptingInput && inputRef.current && !isProcessing) {
      inputRef.current.focus();
    }
  }, [isAcceptingInput, isProcessing]);
  
  // Auto-scroll to the bottom when history changes
  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
    
    // Notify parent of state changes
    if (onStateChange) {
      onStateChange(history);
    }
  }, [history, onStateChange]);
  
  // Reset history position when input changes
  useEffect(() => {
    setHistoryPosition(-1);
  }, [currentInput]);
  
  /**
   * Handles command execution
   */
  const executeCommand = async () => {
    if (!currentInput.trim() || isProcessing) return;
    
    // Store command in history
    const newEntry: TerminalHistoryEntry = {
      type: 'command',
      content: currentInput,
      timestamp: new Date().toISOString(),
    };
    
    // Update history
    setHistory(prev => [...prev, newEntry]);
    
    // Add to input history for up/down navigation
    setInputHistory(prev => [currentInput, ...prev.slice(0, 49)]);
    
    // Clear current input
    setCurrentInput('');
    
    // Execute command if handler provided
    if (onCommandExecute) {
      setIsProcessing(true);
      try {
        await onCommandExecute(newEntry.content);
      } catch (error) {
        // Add error to history
        setHistory(prev => [
          ...prev,
          {
            type: 'error',
            content: `Error: ${error instanceof Error ? error.message : String(error)}`,
            timestamp: new Date().toISOString(),
          },
        ]);
      } finally {
        setIsProcessing(false);
      }
    }
  };
  
  /**
   * Handles key press events for command submission and history navigation
   */
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      executeCommand();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      const nextPosition = Math.min(inputHistory.length - 1, historyPosition + 1);
      if (nextPosition >= 0 && inputHistory[nextPosition]) {
        setHistoryPosition(nextPosition);
        setCurrentInput(inputHistory[nextPosition]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      const nextPosition = Math.max(-1, historyPosition - 1);
      if (nextPosition >= 0) {
        setHistoryPosition(nextPosition);
        setCurrentInput(inputHistory[nextPosition]);
      } else {
        setHistoryPosition(-1);
        setCurrentInput('');
      }
    }
  };
  
  /**
   * Adds content to the terminal history
   * (Exposed for composition)
   */
  const addToHistory = (entry: TerminalHistoryEntry) => {
    setHistory(prev => [...prev, entry]);
  };
  
  /**
   * Clears the terminal history
   * (Exposed for composition)
   */
  const clearHistory = () => {
    setHistory([]);
  };
  
  /**
   * Focus the terminal input
   * (Exposed for composition)
   */
  const focusInput = () => {
    if (inputRef.current) {
      inputRef.current.focus();
    }
  };
  
  // Expose API through ref for parent components
  React.useImperativeHandle(
    terminalRef,
    () => ({
      addToHistory,
      clearHistory,
      focusInput,
      history,
    }),
  );
  
  return (
    <div 
      className={`base-terminal ${className}`}
      ref={terminalRef}
      onClick={focusInput}
    >
      {/* Terminal output/history section */}
      <div className="terminal-history">
        {history.map((entry, index) => (
          <div 
            key={`terminal-entry-${index}`}
            className={`terminal-entry terminal-entry-${entry.type}`}
          >
            {entry.type === 'command' && (
              <span className="terminal-prompt">{promptString}</span>
            )}
            <span className="terminal-content">{entry.content}</span>
          </div>
        ))}
      </div>
      
      {/* Input section */}
      {isAcceptingInput && !isProcessing && (
        <div className="terminal-input-container">
          <span className="terminal-prompt">{promptString}</span>
          <input
            ref={inputRef}
            className="terminal-input"
            type="text"
            value={currentInput}
            onChange={(e) => setCurrentInput(e.target.value)}
            onKeyDown={handleKeyDown}
            autoComplete="off"
            spellCheck="false"
          />
        </div>
      )}
      
      {/* Processing indicator */}
      {isProcessing && (
        <div className="terminal-processing">Processing...</div>
      )}
      
      {/* Children for composition */}
      {children}
    </div>
  );
};

export default BaseTerminal;