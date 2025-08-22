import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { useAuth0 } from '@auth0/auth0-react';
import { Auth0ProviderWrapper } from './contexts/Auth0Provider';
import { AccessControl } from './components/AccessControl';
import { ThemeProvider } from './contexts/ThemeContext';
import { SearchProvider } from './contexts/SearchContext';
import { Navigation } from './components/Navigation';
import { LandingPage } from './components/LandingPage';
import { DirectoryView } from './components/DirectoryView';
import { GridView } from './components/GridView';
import { ConversationView } from './components/ConversationView';
import { ThemeSwitcher } from './components/ThemeSwitcher';

// Simple callback handler for Auth0
const CallbackPage: React.FC = () => {
  const { isLoading, isAuthenticated, error, user } = useAuth0();
  
  // Debug logging for callback
  React.useEffect(() => {
    console.log('CallbackPage state:', {
      isLoading,
      isAuthenticated,
      error: error?.message,
      userEmail: user?.email,
      currentURL: window.location.href
    });
    
    // After auth completes, redirect to main app
    if (!isLoading && isAuthenticated) {
      console.log('Auth complete, redirecting...');
      window.location.href = '/directory';
    }
    
    if (!isLoading && error) {
      console.error('Auth error:', error);
    }
  }, [isLoading, isAuthenticated, error, user]);

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <p className="text-red-600">Authentication Error: {error.message}</p>
          <button onClick={() => window.location.href = '/'} className="mt-4 px-4 py-2 bg-blue-600 text-white rounded">
            Try Again
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center">
      <div className="text-center">
        <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p className="text-gray-600">Processing authentication...</p>
        <p className="text-xs text-gray-400 mt-2">Loading: {isLoading ? 'Yes' : 'No'}, Authenticated: {isAuthenticated ? 'Yes' : 'No'}</p>
      </div>
    </div>
  );
};

function App() {
  // Check if this is a callback URL with auth code
  const isCallback = window.location.search.includes('code=') && window.location.search.includes('state=');
  
  if (isCallback) {
    return (
      <Auth0ProviderWrapper>
        <CallbackPage />
      </Auth0ProviderWrapper>
    );
  }

  return (
    <Auth0ProviderWrapper>
      <AccessControl>
        <ThemeProvider>
          <SearchProvider>
            <Router>
              <div className="min-h-screen bg-gray-50">
                <Navigation />
                <main className="h-[calc(100vh-4rem)]">
                  <Routes>
                    <Route path="/" element={<LandingPage />} />
                    <Route path="/directory" element={<DirectoryView />} />
                    <Route path="/grid" element={<GridView />} />
                    <Route path="/chat" element={<ConversationView />} />
                  </Routes>
                </main>
                <ThemeSwitcher />
              </div>
            </Router>
          </SearchProvider>
        </ThemeProvider>
      </AccessControl>
    </Auth0ProviderWrapper>
  );
}

export default App;
