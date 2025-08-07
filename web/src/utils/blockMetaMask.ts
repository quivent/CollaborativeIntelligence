// Block MetaMask popup and other crypto wallet detection
export const blockMetaMaskPopup = () => {
  // Override ethereum object to prevent MetaMask detection
  if (typeof window !== 'undefined') {
    // Skip ethereum and web3 - already handled in HTML
    // Only block other wallet providers that aren't already blocked
    const walletProviders = [
      'solana',
      'phantom',
      'coin98', 
      'trustwallet',
      'binance',
      'okxwallet'
    ];

    walletProviders.forEach(provider => {
      try {
        if (!(provider in window)) {
          Object.defineProperty(window, provider, {
            value: undefined,
            writable: false,
            configurable: false,
          });
        }
      } catch (e) {
        // Silently ignore if property can't be overridden
      }
    });

    // Block common crypto detection patterns
    const originalWindowAddEventListener = window.addEventListener;
    window.addEventListener = function(type: string, listener: any, options?: any) {
      // Block ethereum-related events
      if (type === 'ethereum#initialized' || 
          type === 'eip6963:announceProvider' ||
          type.includes('wallet') ||
          type.includes('ethereum')) {
        return;
      }
      return originalWindowAddEventListener.call(this, type, listener, options);
    };

    // Override fetch to block wallet-related requests
    const originalFetch = window.fetch;
    window.fetch = function(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
      const url = typeof input === 'string' ? input : input.toString();
      
      // Block common wallet detection endpoints
      if (url.includes('metamask') || 
          url.includes('wallet') || 
          url.includes('ethereum') ||
          url.includes('web3')) {
        return Promise.reject(new Error('Blocked wallet request'));
      }
      
      return originalFetch.call(this, input, init);
    };

    // COMPLETE SILENCE: Block every possible wallet console output
    const isWalletMessage = (args: any[]): boolean => {
      const message = args.join(' ').toLowerCase();
      return message.includes('metamask') || 
             message.includes('ethereum') ||
             message.includes('chrome-extension') ||
             message.includes('failed to connect') ||
             message.includes('extension not found') ||
             message.includes('inpage.js') ||
             message.includes('defineproperty patch') ||
             message.includes('eth {value') ||
             message.includes('wallet') ||
             message.includes('web3') ||
             message.includes('crypto wallet detection blocked');
    };

    // Override EVERY console method completely
    const originalConsole = {
      log: console.log,
      error: console.error,
      warn: console.warn,
      info: console.info,
      debug: console.debug,
      trace: console.trace,
      group: console.group,
      groupEnd: console.groupEnd,
      groupCollapsed: console.groupCollapsed
    };
    
    Object.keys(originalConsole).forEach((method) => {
      (console as any)[method] = function(...args: any[]) {
        if (isWalletMessage(args)) return; // SILENCE ALL WALLET MESSAGES
        return (originalConsole as any)[method].apply(console, args);
      };
    });

    // Block ALL promise rejections from wallets
    window.addEventListener('unhandledrejection', function(event: PromiseRejectionEvent) {
      if (event.reason) {
        const message = String(event.reason).toLowerCase() + 
                       String(event.reason?.message || '').toLowerCase();
        if (message.includes('metamask') ||
            message.includes('failed to connect') ||
            message.includes('extension not found') ||
            message.includes('ethereum') ||
            message.includes('wallet')) {
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          return false;
        }
      }
    });

    // MUTATION OBSERVER: Kill any wallet elements that appear in DOM
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeType === Node.ELEMENT_NODE) {
            const element = node as Element;
            
            // Check if element or its children contain wallet-related content
            const walletSelectors = [
              '[class*="metamask"]',
              '[id*="metamask"]', 
              '[class*="wallet"]',
              '[id*="wallet"]',
              '[data-testid*="wallet"]',
              '[data-testid*="metamask"]',
              'iframe[src*="chrome-extension"]',
              '.web3modal',
              '.wallet-connect-modal'
            ];
            
            walletSelectors.forEach((selector) => {
              // Check the element itself
              if (element.matches && element.matches(selector)) {
                element.remove();
                return;
              }
              
              // Check children
              const walletElements = element.querySelectorAll(selector);
              walletElements.forEach((el) => el.remove());
            });
          }
        });
      });
    });

    // Start observing
    observer.observe(document.body, {
      childList: true,
      subtree: true
    });

    // FINAL NUCLEAR OPTION: Block all event listeners that might trigger popups
    const originalEventTargetAddEventListener = EventTarget.prototype.addEventListener;
    EventTarget.prototype.addEventListener = function(type: string, listener: any, options?: any) {
      // Block wallet-related events
      const blockedEvents = [
        'ethereum#initialized',
        'eip6963:announceProvider', 
        'walletconnect',
        'metamask',
        'web3'
      ];
      
      if (blockedEvents.some(blocked => type.includes(blocked))) {
        return; // Don't add the listener
      }
      
      return originalEventTargetAddEventListener.call(this, type, listener, options);
    };

    // Use original console to log our success (bypassing our own filter)
    originalConsole.log('🚫 NUCLEAR WALLET BLOCKING ACTIVE - POPUPS TERMINATED');
  }
};