/**
 * Types of entries that can appear in the terminal history
 */
export type TerminalEntryType = 
  | 'command'    // User input commands
  | 'output'     // Standard command output
  | 'error'      // Error messages
  | 'system'     // System messages
  | 'markdown'   // Markdown formatted content
  | 'code'       // Code blocks with syntax highlighting
  | 'image';     // Image content

/**
 * Represents a single entry in the terminal history
 */
export interface TerminalHistoryEntry {
  /**
   * The type of terminal entry
   */
  type: TerminalEntryType;
  
  /**
   * The content of the entry (text, markdown, etc.)
   */
  content: string;
  
  /**
   * ISO timestamp of when the entry was created
   */
  timestamp: string;
  
  /**
   * Optional metadata for specialized entry types
   */
  metadata?: {
    /**
     * For code entries, the programming language for syntax highlighting
     */
    language?: string;
    
    /**
     * For image entries, the image source or data URI
     */
    imageSource?: string;
    
    /**
     * For output entries, whether the output is complete or partial
     */
    isComplete?: boolean;
    
    /**
     * Any additional type-specific metadata
     */
    [key: string]: any;
  };
}

/**
 * The current state of the terminal
 */
export interface TerminalState {
  /**
   * Complete history of terminal entries
   */
  history: TerminalHistoryEntry[];
  
  /**
   * Current user input
   */
  currentInput: string;
  
  /**
   * Whether the terminal is currently processing a command
   */
  isProcessing: boolean;
  
  /**
   * Whether the terminal is accepting user input
   */
  isAcceptingInput: boolean;
}

/**
 * Configuration options for the terminal
 */
export interface TerminalConfig {
  /**
   * The prompt string to display before input
   */
  promptString: string;
  
  /**
   * Maximum number of history entries to keep
   */
  maxHistoryLength: number;
  
  /**
   * Whether to auto-scroll to the bottom on new entries
   */
  autoScroll: boolean;
  
  /**
   * Whether to enable command history navigation (up/down arrows)
   */
  enableHistoryNavigation: boolean;
  
  /**
   * Whether to enable syntax highlighting for code blocks
   */
  enableSyntaxHighlighting: boolean;
  
  /**
   * Default language for syntax highlighting when not specified
   */
  defaultCodeLanguage: string;
}

/**
 * Commands available to the terminal
 */
export interface TerminalCommands {
  [key: string]: {
    /**
     * Function to execute when the command is run
     */
    execute: (args: string[]) => Promise<TerminalHistoryEntry | TerminalHistoryEntry[]>;
    
    /**
     * Description of what the command does (for help text)
     */
    description: string;
    
    /**
     * Usage examples for the command
     */
    examples?: string[];
  };
}