// TAURI_DETECTION_IMPROVED.ts
// Enhanced Tauri detection with exponential backoff, connection states, and reconnection logic

import { appWindow } from '@tauri-apps/api/window';

// Connection states for better lifecycle management
export enum TauriConnectionState {
  INITIALIZING = 'initializing',
  CONNECTING = 'connecting',
  CONNECTED = 'connected',
  RECONNECTING = 'reconnecting',
  FAILED = 'failed',
  WEB_ONLY = 'web_only'
}

// Custom event names for connection state changes
export const TAURI_BACKEND_STATE_CHANGE = 'tauri-backend-state-change';

// Initial state
let connectionState: TauriConnectionState = TauriConnectionState.INITIALIZING;
let connectionDetails: any = null;
let tauriDetected = false;
let tauriReady = false;
let detectionAttempts = 0;

/**
 * Update the connection state and emit an event
 */
const updateConnectionState = (newState: TauriConnectionState, details?: any) => {
  const previousState = connectionState;
  connectionState = newState;
  connectionDetails = details || connectionDetails;
  
  // Emit custom event for connection state change
  window.dispatchEvent(new CustomEvent(TAURI_BACKEND_STATE_CHANGE, {
    detail: {
      state: newState,
      previous: previousState,
      details: connectionDetails
    }
  }));
  
  console.log(`Tauri connection state changed: ${previousState} -> ${newState}`, details || {});
}

/**
 * Deep Tauri detection with exponential backoff
 * This function performs multiple checks to verify Tauri is available and ready
 */
const detectTauriWithBackoff = async () => {
  const maxAttempts = 10;
  const initialDelay = 100; // Start with 100ms
  detectionAttempts++;
  
  updateConnectionState(
    detectionAttempts === 1 ? TauriConnectionState.CONNECTING : TauriConnectionState.RECONNECTING,
    { attempt: detectionAttempts, maxAttempts }
  );
  
  try {
    // First check - window.__TAURI__ object existence
    if (typeof window.__TAURI__ === 'undefined') {
      throw new Error('Tauri object not found');
    }
    
    // Second check - Try to access a Tauri API method
    try {
      // This is a lighter-weight API call to verify Tauri is working
      const appVersion = await window.__TAURI__.app.getVersion();
      console.log('Tauri version:', appVersion);
    } catch (apiError) {
      throw new Error(`Tauri API error: ${apiError.message}`);
    }
    
    // Third check - Subscribe to a test event to verify event system
    try {
      const unlistenPromise = new Promise<void>((resolve, reject) => {
        let timeout: NodeJS.Timeout;
        
        // Set a timeout to detect if the event system is not responding
        timeout = setTimeout(() => {
          reject(new Error('Tauri event system timeout'));
        }, 1000);
        
        // Try to subscribe to the backend-ready event
        const unlisten = window.__TAURI__.event.listen('backend-ready', (event) => {
          clearTimeout(timeout);
          console.log('Backend ready event received:', event);
          tauriReady = true;
          unlisten(); // Remove the listener after getting the event
          resolve();
        });
      });
      
      // Wait for the event subscription to complete
      await unlistenPromise;
    } catch (eventError) {
      throw new Error(`Tauri event system error: ${eventError.message}`);
    }
    
    // If we get here, Tauri is fully detected and working
    tauriDetected = true;
    updateConnectionState(TauriConnectionState.CONNECTED, {
      attempts: detectionAttempts,
      timestamp: Date.now()
    });
    
    return true;
  } catch (error) {
    console.warn(`Attempt ${detectionAttempts}/${maxAttempts} to detect Tauri failed:`, error.message);
    
    if (detectionAttempts >= maxAttempts) {
      updateConnectionState(TauriConnectionState.FAILED, {
        error: error.message,
        attempts: detectionAttempts
      });
      
      // Final fallback - try to recover by setting web-only mode
      if (!isElectron()) {
        updateConnectionState(TauriConnectionState.WEB_ONLY);
      }
      
      return false;
    }
    
    // Calculate delay with exponential backoff and jitter
    const delay = Math.min(
      5000, // Cap at 5 seconds max
      initialDelay * Math.pow(1.5, detectionAttempts) * (0.9 + Math.random() * 0.2)
    );
    
    console.log(`Retrying Tauri detection in ${delay.toFixed(0)}ms...`);
    
    // Wait and retry
    await new Promise(resolve => setTimeout(resolve, delay));
    return detectTauriWithBackoff();
  }
};

/**
 * Check if running in Electron
 */
const isElectron = () => {
  return typeof navigator === 'object' && 
         typeof navigator.userAgent === 'string' && 
         navigator.userAgent.indexOf('Electron') >= 0;
};

/**
 * Initialize Tauri detection when the document is ready
 */
const initializeTauriDetection = () => {
  if (document.readyState === 'complete' || 
      document.readyState === 'interactive') {
    detectTauriWithBackoff();
  } else {
    // Wait for document to be ready before attempting detection
    document.addEventListener('DOMContentLoaded', () => {
      detectTauriWithBackoff();
    });
  }
  
  // Listen for Tauri backend heartbeat events to maintain connection status
  if (typeof window.__TAURI__ !== 'undefined') {
    window.__TAURI__.event.listen('backend-heartbeat', (event) => {
      // If previously disconnected, update state to connected
      if (connectionState === TauriConnectionState.RECONNECTING ||
          connectionState === TauriConnectionState.FAILED) {
        updateConnectionState(TauriConnectionState.CONNECTED, {
          heartbeat: event.payload,
          recovered: true,
          timestamp: Date.now()
        });
      }
    });
  }
  
  // Setup window focus/blur handlers to detect when to check connection
  window.addEventListener('focus', () => {
    // When window regains focus, check if we need to reconnect
    if (tauriDetected && 
        (connectionState === TauriConnectionState.FAILED || 
         connectionState === TauriConnectionState.RECONNECTING)) {
      console.log('Window focused, attempting to reconnect to Tauri backend...');
      detectionAttempts = 0; // Reset counter for fresh reconnection attempt
      detectTauriWithBackoff();
    }
  });
};

/**
 * Public API to get current connection state
 */
export const getTauriConnectionState = () => {
  return {
    state: connectionState,
    details: connectionDetails,
    detected: tauriDetected,
    ready: tauriReady
  };
};

/**
 * Manual reconnection attempt function
 */
export const reconnectToTauri = async () => {
  if (connectionState === TauriConnectionState.CONNECTED) {
    console.log('Already connected to Tauri backend');
    return true;
  }
  
  console.log('Manually attempting to reconnect to Tauri backend...');
  detectionAttempts = 0; // Reset counter for fresh attempt
  return detectTauriWithBackoff();
};

// Auto-initialize when this module is imported
initializeTauriDetection();

export default {
  getTauriConnectionState,
  reconnectToTauri,
  connectionState: () => connectionState,
  isConnected: () => connectionState === TauriConnectionState.CONNECTED,
  isReady: () => tauriReady
};