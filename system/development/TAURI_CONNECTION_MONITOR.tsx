// TAURI_CONNECTION_MONITOR.tsx
// React component for monitoring and managing Tauri connection state

import React, { useEffect, useState } from 'react';
import { toast } from 'react-toastify'; // Assuming this is used for notifications
import { TauriConnectionState, TAURI_BACKEND_STATE_CHANGE } from './TAURI_DETECTION_IMPROVED';

// Import the clients that need to be reinitialized on reconnection
// Note: These would be your actual client imports in your application
import { configurationClient } from '../services/configuration';
import { processClient } from '../services/process';
import { sessionClient } from '../services/session';

interface ConnectionStateProps {
  showNotifications?: boolean;
  onConnected?: () => void;
  onDisconnected?: () => void;
  onReconnected?: () => void;
}

/**
 * Component that monitors Tauri connection state and manages reconnection
 * This component doesn't render anything visible by default
 */
export const TauriConnectionMonitor: React.FC<ConnectionStateProps> = ({
  showNotifications = true,
  onConnected,
  onDisconnected,
  onReconnected,
}) => {
  const [currentState, setCurrentState] = useState<TauriConnectionState>(TauriConnectionState.INITIALIZING);
  const [wasEverConnected, setWasEverConnected] = useState(false);
  
  useEffect(() => {
    // Handle Tauri connection state changes
    const handleTauriStateChange = async (event: CustomEvent) => {
      const { state, previous, details } = event.detail;
      
      console.log(`Connection state changed: ${previous} → ${state}`);
      setCurrentState(state);
      
      // Only handle meaningful transitions
      if (previous === state) return;
      
      // Handle different state transitions
      if (state === TauriConnectionState.CONNECTED) {
        if (!wasEverConnected) {
          setWasEverConnected(true);
          
          if (showNotifications) {
            toast.success('Connected to terminal backend', { autoClose: 2000 });
          }
          
          onConnected?.();
        } else if (previous === TauriConnectionState.RECONNECTING || 
                   previous === TauriConnectionState.FAILED) {
          if (showNotifications) {
            toast.success('Reconnected to terminal backend', { autoClose: 2000 });
          }
          
          // Restore application state
          await restoreApplicationState();
          onReconnected?.();
        }
      } 
      else if (state === TauriConnectionState.FAILED) {
        if (previous === TauriConnectionState.CONNECTED || 
            previous === TauriConnectionState.RECONNECTING) {
          if (showNotifications) {
            toast.error(
              'Connection to terminal backend lost. Will keep trying to reconnect...', 
              { autoClose: false }
            );
          }
          
          onDisconnected?.();
        }
      }
      else if (state === TauriConnectionState.WEB_ONLY) {
        if (showNotifications) {
          toast.warning(
            'Running in web-only mode. Some features may be unavailable.', 
            { autoClose: 5000 }
          );
        }
      }
    };
    
    // Add event listener for connection state changes
    window.addEventListener(
      TAURI_BACKEND_STATE_CHANGE, 
      handleTauriStateChange as EventListener
    );
    
    // Clean up event listener on unmount
    return () => {
      window.removeEventListener(
        TAURI_BACKEND_STATE_CHANGE, 
        handleTauriStateChange as EventListener
      );
    };
  }, [showNotifications, onConnected, onDisconnected, onReconnected, wasEverConnected]);
  
  /**
   * Restore application state after reconnection
   * This function should reload any necessary data from the backend
   */
  const restoreApplicationState = async () => {
    try {
      console.log('Restoring application state after reconnection...');
      
      // Load configuration first as other services might depend on it
      await configurationClient.loadConfig();
      
      // Load processes
      await processClient.listProcesses();
      
      // Load sessions
      await sessionClient.listSessions();
      
      console.log('Application state restored successfully');
    } catch (error) {
      console.error('Error restoring application state:', error);
      
      if (showNotifications) {
        toast.error('Failed to restore some application data', { 
          autoClose: 5000 
        });
      }
    }
  };
  
  // Optional: render a visual indicator based on connection state
  if (process.env.NODE_ENV === 'development') {
    return (
      <div 
        style={{ 
          position: 'fixed', 
          bottom: 5, 
          right: 5, 
          zIndex: 9999,
          padding: '2px 8px',
          borderRadius: '4px',
          fontSize: '10px',
          opacity: 0.8,
          backgroundColor: getBackgroundColorForState(currentState),
          color: '#fff'
        }}
      >
        {currentState}
      </div>
    );
  }
  
  // In production, don't render anything visible
  return null;
};

// Helper function to get background color based on state
const getBackgroundColorForState = (state: TauriConnectionState): string => {
  switch (state) {
    case TauriConnectionState.CONNECTED:
      return '#4caf50'; // Green
    case TauriConnectionState.RECONNECTING:
      return '#ff9800'; // Orange
    case TauriConnectionState.FAILED:
      return '#f44336'; // Red
    case TauriConnectionState.WEB_ONLY:
      return '#9c27b0'; // Purple
    default:
      return '#2196f3'; // Blue for initializing/connecting
  }
};

export default TauriConnectionMonitor;