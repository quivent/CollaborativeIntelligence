import React from 'react';
import { Auth0Provider } from '@auth0/auth0-react';

interface Auth0ProviderWrapperProps {
  children: React.ReactNode;
}

export const Auth0ProviderWrapper: React.FC<Auth0ProviderWrapperProps> = ({ children }) => {
  // Check if Auth0 is disabled for local development
  const isAuthDisabled = process.env.REACT_APP_DISABLE_AUTH === 'true';
  
  // If Auth0 is disabled, return children directly without wrapping
  if (isAuthDisabled) {
    return <>{children}</>;
  }
  
  const domain = process.env.REACT_APP_AUTH0_DOMAIN!;
  const clientId = process.env.REACT_APP_AUTH0_CLIENT_ID!;
  
  return (
    <Auth0Provider
      domain={domain}
      clientId={clientId}
      authorizationParams={{
        redirect_uri: window.location.origin
      }}
      useRefreshTokens={true}
      cacheLocation="localstorage"
      skipRedirectCallback={false}
    >
      {children}
    </Auth0Provider>
  );
};