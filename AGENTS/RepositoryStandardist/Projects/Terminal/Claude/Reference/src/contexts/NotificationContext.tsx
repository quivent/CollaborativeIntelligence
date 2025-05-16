import React, { createContext, useContext, useReducer, useCallback } from 'react';

// Notification Types
export type NotificationType = 'info' | 'success' | 'warning' | 'error';

// Notification Interface
export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  createdAt: number;
  duration?: number;
  autoClose?: boolean;
  onAction?: () => void;
  actionLabel?: string;
}

// Notification Context State Interface
export interface NotificationContextState {
  notifications: Notification[];
  pausedNotifications: boolean;
}

// Notification Context Actions
type NotificationAction = 
  | { type: 'ADD_NOTIFICATION'; notification: Notification }
  | { type: 'REMOVE_NOTIFICATION'; id: string }
  | { type: 'CLEAR_NOTIFICATIONS' }
  | { type: 'CLEAR_NOTIFICATIONS_BY_TYPE'; notificationType: NotificationType }
  | { type: 'PAUSE_NOTIFICATIONS'; paused: boolean };

// Initial state
const initialState: NotificationContextState = {
  notifications: [],
  pausedNotifications: false,
};

// Generate a unique ID for notifications
const generateId = (): string => {
  return `notification-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
};

// Default notification duration
const DEFAULT_DURATION = 5000; // 5 seconds

// Notification reducer
const notificationReducer = (
  state: NotificationContextState,
  action: NotificationAction
): NotificationContextState => {
  switch (action.type) {
    case 'ADD_NOTIFICATION':
      return {
        ...state,
        notifications: [
          ...state.notifications,
          action.notification,
        ],
      };
    
    case 'REMOVE_NOTIFICATION':
      return {
        ...state,
        notifications: state.notifications.filter(
          (notification) => notification.id !== action.id
        ),
      };
    
    case 'CLEAR_NOTIFICATIONS':
      return {
        ...state,
        notifications: [],
      };
    
    case 'CLEAR_NOTIFICATIONS_BY_TYPE':
      return {
        ...state,
        notifications: state.notifications.filter(
          (notification) => notification.type !== action.notificationType
        ),
      };
    
    case 'PAUSE_NOTIFICATIONS':
      return {
        ...state,
        pausedNotifications: action.paused,
      };
    
    default:
      return state;
  }
};

// Context interface
interface NotificationContextProps {
  state: NotificationContextState;
  notify: (
    type: NotificationType,
    title: string,
    message: string,
    options?: {
      duration?: number;
      autoClose?: boolean;
      onAction?: () => void;
      actionLabel?: string;
    }
  ) => string;
  removeNotification: (id: string) => void;
  clearAllNotifications: () => void;
  clearNotificationsByType: (type: NotificationType) => void;
  pauseNotifications: (paused: boolean) => void;
}

// Create context
const NotificationContext = createContext<NotificationContextProps | undefined>(undefined);

// Notification Provider component
export const NotificationProvider: React.FC<{
  children: React.ReactNode;
  initialState?: Partial<NotificationContextState>;
}> = ({ children, initialState: initialStateOverrides }) => {
  // Merge default initial state with any overrides
  const mergedInitialState = {
    ...initialState,
    ...initialStateOverrides,
  };
  
  // Set up reducer
  const [state, dispatch] = useReducer(notificationReducer, mergedInitialState);
  
  // Create a notification
  const notify = useCallback(
    (
      type: NotificationType,
      title: string,
      message: string,
      options?: {
        duration?: number;
        autoClose?: boolean;
        onAction?: () => void;
        actionLabel?: string;
      }
    ): string => {
      const id = generateId();
      const duration = options?.duration ?? DEFAULT_DURATION;
      const autoClose = options?.autoClose ?? true;
      
      const notification: Notification = {
        id,
        type,
        title,
        message,
        createdAt: Date.now(),
        duration,
        autoClose,
        onAction: options?.onAction,
        actionLabel: options?.actionLabel,
      };
      
      dispatch({ type: 'ADD_NOTIFICATION', notification });
      
      // Auto-close the notification after the specified duration if enabled
      if (autoClose && duration > 0) {
        setTimeout(() => {
          dispatch({ type: 'REMOVE_NOTIFICATION', id });
        }, duration);
      }
      
      return id;
    },
    []
  );
  
  // Remove a notification
  const removeNotification = useCallback((id: string) => {
    dispatch({ type: 'REMOVE_NOTIFICATION', id });
  }, []);
  
  // Clear all notifications
  const clearAllNotifications = useCallback(() => {
    dispatch({ type: 'CLEAR_NOTIFICATIONS' });
  }, []);
  
  // Clear notifications by type
  const clearNotificationsByType = useCallback((type: NotificationType) => {
    dispatch({ type: 'CLEAR_NOTIFICATIONS_BY_TYPE', notificationType: type });
  }, []);
  
  // Pause notifications
  const pauseNotifications = useCallback((paused: boolean) => {
    dispatch({ type: 'PAUSE_NOTIFICATIONS', paused });
  }, []);
  
  // Combine state and actions to pass through the provider
  const contextValue: NotificationContextProps = {
    state,
    notify,
    removeNotification,
    clearAllNotifications,
    clearNotificationsByType,
    pauseNotifications,
  };
  
  return (
    <NotificationContext.Provider value={contextValue}>
      {children}
    </NotificationContext.Provider>
  );
};

// Custom hook for accessing the notification context
export const useNotification = (): NotificationContextProps => {
  const context = useContext(NotificationContext);
  
  if (context === undefined) {
    throw new Error('useNotification must be used within a NotificationProvider');
  }
  
  return context;
};

export default NotificationContext;