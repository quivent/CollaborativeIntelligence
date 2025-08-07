import React from 'react';
import { useAuth0 } from '@auth0/auth0-react';

// Email allowlist - add authorized email addresses here
const ALLOWED_EMAILS = [
  'jbenjaminkr@gmail.com',
  'claude.jbenjaminkr@gmail.com',
  'sudhirparam@gmail.com',
];

interface AccessControlProps {
  children: React.ReactNode;
}

export const AccessControl: React.FC<AccessControlProps> = ({ children }) => {
  const { isLoading, isAuthenticated, user, loginWithRedirect, logout } = useAuth0();
  
  // Debug logging
  console.log('AccessControl state:', {
    isLoading,
    isAuthenticated,
    userEmail: user?.email,
    userExists: !!user,
    allowedEmails: ALLOWED_EMAILS
  });

  // Show loading spinner while Auth0 is checking authentication
  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Checking access permissions...</p>
        </div>
      </div>
    );
  }

  // Not authenticated - show login screen
  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center px-4">
        <div className="max-w-md w-full bg-white rounded-xl shadow-lg p-8 text-center">
          <div className="mb-6">
            <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
              </svg>
            </div>
            <h1 className="text-2xl font-bold text-gray-800 mb-2">Agent Explorer</h1>
            <p className="text-gray-600">Secure access required</p>
          </div>
          
          <button
            onClick={() => loginWithRedirect()}
            className="w-full bg-blue-600 text-white py-3 px-6 rounded-lg hover:bg-blue-700 transition-colors duration-200 flex items-center justify-center gap-2"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
            </svg>
            Sign In to Continue
          </button>
          
          <p className="text-xs text-gray-500 mt-4">
            Access is restricted to authorized users only
          </p>
        </div>
      </div>
    );
  }

  // Authenticated but email not in allowlist
  if (user?.email && !ALLOWED_EMAILS.includes(user.email)) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-red-50 to-pink-100 flex items-center justify-center px-4">
        <div className="max-w-md w-full bg-white rounded-xl shadow-lg p-8 text-center">
          <div className="mb-6">
            <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
            </div>
            <h1 className="text-2xl font-bold text-gray-800 mb-2">Access Denied</h1>
            <p className="text-gray-600 mb-4">Your email address is not authorized</p>
            <p className="text-sm text-gray-500 mb-6">
              Signed in as: <strong>{user.email}</strong>
            </p>
          </div>
          
          <button
            onClick={() => logout({ logoutParams: { returnTo: window.location.origin } })}
            className="w-full bg-gray-600 text-white py-3 px-6 rounded-lg hover:bg-gray-700 transition-colors duration-200 flex items-center justify-center gap-2"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
            Sign Out
          </button>
          
          <p className="text-xs text-gray-500 mt-4">
            Please contact the administrator for access
          </p>
        </div>
      </div>
    );
  }

  // Authenticated and authorized - show the app
  return <>{children}</>;
};