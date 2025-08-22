import React, { useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { UserProfile } from './UserProfile';
import { useSearch } from '../contexts/SearchContext';

export const Navigation: React.FC = () => {
  const location = useLocation();
  const { searchQuery, setSearchQuery } = useSearch();

  // Clear search when route changes
  useEffect(() => {
    setSearchQuery('');
  }, [location.pathname, setSearchQuery]);

  const isActive = (path: string) => location.pathname === path;

  const handleSearchSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Search is handled by live filtering, no additional action needed
  };

  return (
    <nav className="bg-white shadow-sm border-b border-gray-200 h-16">
      <div className="px-4">
        <div className="flex items-center justify-between h-16">
          {/* Left side - Logo/Title */}
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <h1 className="text-xl font-bold text-gray-800">
                Agent Explorer
              </h1>
            </div>
            <div className="ml-4 text-sm text-gray-600">
              AI Agent Management System
            </div>
          </div>

          {/* Center - Navigation + Search */}
          <div className="flex items-center space-x-8">
            {/* Navigation Links */}
            <div className="flex items-center space-x-1">
              <Link
                to="/directory"
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 border ${
                  isActive('/directory')
                    ? 'pastel-blue border-transparent shadow-sm'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50 border-transparent'
                }`}
              >
                <div className="flex items-center space-x-2">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 5a2 2 0 012-2h4a2 2 0 012 2v.01" />
                  </svg>
                  <span>Directory</span>
                </div>
              </Link>

              <Link
                to="/grid"
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 border ${
                  isActive('/grid')
                    ? 'pastel-purple border-transparent shadow-sm'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50 border-transparent'
                }`}
              >
                <div className="flex items-center space-x-2">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                  </svg>
                  <span>Grid</span>
                </div>
              </Link>

              <Link
                to="/chat"
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 border ${
                  isActive('/chat')
                    ? 'pastel-green border-transparent shadow-sm'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50 border-transparent'
                }`}
              >
                <div className="flex items-center space-x-2">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                  </svg>
                  <span>Chat</span>
                </div>
              </Link>
            </div>

            {/* Search Bar */}
            <form onSubmit={handleSearchSubmit} className="relative">
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg className="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                  </svg>
                </div>
                <input
                  type="text"
                  placeholder="Search agents..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-64 pl-10 pr-4 py-2 text-sm border border-gray-200 rounded-full bg-gray-50 focus:bg-white focus:border-blue-300 focus:ring-2 focus:ring-blue-100 focus:outline-none transition-all duration-200 placeholder-gray-400"
                />
              </div>
            </form>
          </div>

          {/* Right side - User Profile */}
          <div className="flex items-center">
            <UserProfile />
          </div>
        </div>
      </div>
    </nav>
  );
};