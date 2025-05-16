import React from 'react';
import '../../styles/TerminalHeader.css';

export interface TerminalHeaderProps {
  /**
   * Handler for the clear button click
   */
  onClear: () => void;
  
  /**
   * Handler for toggling fullscreen mode
   */
  onToggleFullscreen: () => void;
  
  /**
   * Whether the terminal is in fullscreen mode
   */
  isFullscreen: boolean;
  
  /**
   * Whether the terminal is currently processing a command
   */
  isProcessing: boolean;
  
  /**
   * Custom title for the terminal
   */
  title?: string;
  
  /**
   * Custom class name for styling
   */
  className?: string;
}

/**
 * TerminalHeader provides controls and status information
 * for the terminal container.
 */
export const TerminalHeader: React.FC<TerminalHeaderProps> = ({
  onClear,
  onToggleFullscreen,
  isFullscreen,
  isProcessing,
  title = 'Terminal',
  className = '',
}) => {
  return (
    <div className={`terminal-header ${className}`}>
      <div className="terminal-header-title">
        {title}
        {isProcessing && (
          <span className="terminal-header-status">Processing...</span>
        )}
      </div>
      
      <div className="terminal-header-controls">
        <button
          className="terminal-header-button terminal-button-clear"
          onClick={onClear}
          aria-label="Clear terminal"
          title="Clear terminal"
        >
          <span className="terminal-button-icon">✕</span>
        </button>
        
        <button
          className="terminal-header-button terminal-button-fullscreen"
          onClick={onToggleFullscreen}
          aria-label={isFullscreen ? "Exit fullscreen" : "Enter fullscreen"}
          title={isFullscreen ? "Exit fullscreen" : "Enter fullscreen"}
        >
          <span className="terminal-button-icon">
            {isFullscreen ? '↙' : '↗'}
          </span>
        </button>
      </div>
    </div>
  );
};

export default TerminalHeader;