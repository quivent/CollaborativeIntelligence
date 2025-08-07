// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';

// NUCLEAR ERROR SUPPRESSION - Block React error overlay for MetaMask
const originalConsoleError = console.error;
console.error = (...args) => {
  const message = args.join(' ');
  if (message.includes('MetaMask') || 
      message.includes('Failed to connect') ||
      message.includes('chrome-extension')) {
    return; // Completely ignore MetaMask errors
  }
  originalConsoleError(...args);
};
