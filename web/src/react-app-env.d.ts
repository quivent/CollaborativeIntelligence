/// <reference types="react-scripts" />

// NUCLEAR OPTION: Completely disable error overlay
declare global {
  interface Window {
    __REACT_ERROR_OVERLAY_GLOBAL_HOOK__?: {
      isFiberMounted: () => boolean;
    };
  }
}

// Override React error overlay
if (typeof window !== 'undefined') {
  window.__REACT_ERROR_OVERLAY_GLOBAL_HOOK__ = {
    isFiberMounted: () => false
  };
}
