#!/bin/bash

# 🔐 Auth0 Automated Setup Script
# This script automates the entire Auth0 setup process for Agent Explorer

set -e  # Exit on any error

echo "🚀 Starting Auth0 automated setup..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required but not installed."
    echo "Please install Node.js from https://nodejs.org"
    exit 1
fi

# Check if Auth0 CLI is available
if ! command -v auth0 &> /dev/null; then
    echo "⚠️  Auth0 CLI not found. Installing..."
    
    # Try to install via Homebrew (macOS)
    if command -v brew &> /dev/null; then
        echo "📦 Installing Auth0 CLI via Homebrew..."
        brew install auth0/auth0-cli/auth0
    else
        echo "❌ Please install Auth0 CLI manually:"
        echo "   brew install auth0/auth0-cli/auth0"
        echo "   Or visit: https://github.com/auth0/auth0-cli#installation"
        exit 1
    fi
fi

# Check if Vercel CLI is available (optional)
if ! command -v vercel &> /dev/null; then
    echo "⚠️  Vercel CLI not found. Environment variables will need to be set manually."
    echo "   To install: npm install -g vercel"
fi

# Run the automated setup
echo ""
echo "🤖 Running automated Auth0 setup..."
node scripts/setup-auth0.js

echo ""
echo "✅ Automated setup complete!"
echo ""
echo "🔧 Manual steps remaining:"
echo "   1. Edit src/components/AccessControl.tsx"
echo "   2. Update the ALLOWED_EMAILS array with your email addresses"
echo "   3. Deploy: vercel --prod"
echo ""
echo "📖 For detailed instructions, see AUTH0_SETUP.md"