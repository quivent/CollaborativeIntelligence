import React from 'react';
import '../../styles/TerminalFooter.css';

export interface TerminalFooterProps {
  /**
   * Number of history entries
   */
  historyLength: number;
  
  /**
   * Whether the terminal is currently processing
   */
  isProcessing: boolean;
  
  /**
   * Current terminal mode
   */
  mode?: string;
  
  /**
   * Custom class name for styling
   */
  className?: string;
}

/**
 * TerminalFooter provides status information and
 * additional controls at the bottom of the terminal.
 */
export const TerminalFooter: React.FC<TerminalFooterProps> = ({
  historyLength,
  isProcessing,
  mode = 'standard',
  className = '',
}) => {
  // Get current timestamp for status display
  const timestamp = new Date().toLocaleTimeString();
  
  return (
    <div className={`terminal-footer ${className}`}>
      <div className="terminal-footer-status">
        <span className="terminal-footer-history-count">
          {historyLength} {historyLength === 1 ? 'entry' : 'entries'}
        </span>
        
        <span className="terminal-footer-divider">|</span>
        
        <span className={`terminal-footer-state ${isProcessing ? 'processing' : 'ready'}`}>
          {isProcessing ? 'Processing' : 'Ready'}
        </span>
        
        <span className="terminal-footer-divider">|</span>
        
        <span className="terminal-footer-mode">
          Mode: {mode}
        </span>
      </div>
      
      <div className="terminal-footer-timestamp">
        {timestamp}
      </div>
    </div>
  );
};

export default TerminalFooter;