import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { blockMetaMaskPopup } from './utils/blockMetaMask';

// Block MetaMask popup before app starts
blockMetaMaskPopup();

// NUCLEAR OPTION: Disable React error overlay completely
if (process.env.NODE_ENV === 'development') {
  const originalConsoleError = console.error;
  console.error = (...args) => {
    const message = args.join(' ');
    // Block ALL MetaMask related errors from reaching React error overlay
    if (message.includes('MetaMask') || 
        message.includes('Failed to connect') ||
        message.includes('chrome-extension') ||
        message.includes('inpage.js') ||
        message.includes('Uncaught')) {
      return; // SILENCE COMPLETELY
    }
    originalConsoleError(...args);
  };

  // Override window error handler to prevent React error overlay
  window.addEventListener('error', (event) => {
    if (event.message && (
        event.message.includes('MetaMask') ||
        event.message.includes('Failed to connect') ||
        event.message.includes('chrome-extension'))) {
      event.preventDefault();
      event.stopPropagation();
      return false;
    }
  });

  // Override unhandled rejection handler
  window.addEventListener('unhandledrejection', (event) => {
    if (event.reason?.message && (
        event.reason.message.includes('MetaMask') ||
        event.reason.message.includes('Failed to connect'))) {
      event.preventDefault();
      event.stopPropagation();
      return false;
    }
  });
}

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
