#!/usr/bin/env node

/**
 * Auth0 Automated Setup Script
 * 
 * This script automates the Auth0 setup process for the Agent Explorer.
 * It creates the Auth0 application, configures settings, and sets up Vercel environment variables.
 */

const { execSync, spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

class Auth0Setup {
  constructor() {
    this.appName = 'Agent Explorer';
    this.productionUrl = 'https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app';
    this.localUrl = 'http://localhost:3000';
    this.appConfig = null;
  }

  // Colors for console output
  colors = {
    red: '\x1b[31m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    magenta: '\x1b[35m',
    cyan: '\x1b[36m',
    reset: '\x1b[0m',
    bright: '\x1b[1m'
  };

  log(message, color = 'reset') {
    console.log(`${this.colors[color]}${message}${this.colors.reset}`);
  }

  async runCommand(command, description) {
    this.log(`\n🔄 ${description}...`, 'blue');
    try {
      const result = execSync(command, { encoding: 'utf8' });
      this.log(`✅ ${description} completed`, 'green');
      return result;
    } catch (error) {
      this.log(`❌ ${description} failed: ${error.message}`, 'red');
      throw error;
    }
  }

  async checkAuth0Login() {
    try {
      const result = execSync('auth0 auth whoami', { encoding: 'utf8' });
      this.log('✅ Already logged in to Auth0', 'green');
      return true;
    } catch (error) {
      this.log('⚠️  Not logged in to Auth0', 'yellow');
      return false;
    }
  }

  async loginToAuth0() {
    this.log('\n🔐 Logging in to Auth0...', 'cyan');
    this.log('A browser window will open for authentication', 'yellow');
    
    try {
      execSync('auth0 auth login', { stdio: 'inherit' });
      this.log('✅ Successfully logged in to Auth0', 'green');
    } catch (error) {
      this.log('❌ Failed to login to Auth0', 'red');
      throw error;
    }
  }

  async createApplication() {
    this.log('\n🏗️  Creating Auth0 Application...', 'cyan');
    
    const configFile = path.join(__dirname, 'temp-app-config.json');
    const appConfig = {
      name: this.appName,
      type: 'spa', // Single Page Application
      description: 'Secure AI agent management interface with directory and grid views',
      callbacks: [this.productionUrl, this.localUrl],
      web_origins: [this.productionUrl, this.localUrl],
      logout_urls: [this.productionUrl, this.localUrl],
      grant_types: ['authorization_code', 'refresh_token', 'implicit'],
      token_endpoint_auth_method: 'none'
    };

    // Write config to temporary file
    fs.writeFileSync(configFile, JSON.stringify(appConfig, null, 2));

    try {
      const result = await this.runCommand(
        `auth0 apps create --file ${configFile} --json`,
        'Creating Auth0 application'
      );
      
      this.appConfig = JSON.parse(result);
      this.log(`✅ Application created with ID: ${this.appConfig.client_id}`, 'green');
      
      // Clean up temp file
      fs.unlinkSync(configFile);
      
      return this.appConfig;
    } catch (error) {
      // Clean up temp file on error
      if (fs.existsSync(configFile)) {
        fs.unlinkSync(configFile);
      }
      throw error;
    }
  }

  async updateEnvironmentFile() {
    this.log('\n📝 Updating .env file...', 'cyan');
    
    const envPath = path.join(__dirname, '../.env');
    let envContent = '';
    
    if (fs.existsSync(envPath)) {
      envContent = fs.readFileSync(envPath, 'utf8');
    }
    
    // Update or add Auth0 environment variables
    const domain = this.appConfig.domain;
    const clientId = this.appConfig.client_id;
    
    // Replace existing values or add new ones
    if (envContent.includes('REACT_APP_AUTH0_DOMAIN=')) {
      envContent = envContent.replace(
        /REACT_APP_AUTH0_DOMAIN=.*/g,
        `REACT_APP_AUTH0_DOMAIN=${domain}`
      );
    } else {
      envContent = `REACT_APP_AUTH0_DOMAIN=${domain}\n` + envContent;
    }
    
    if (envContent.includes('REACT_APP_AUTH0_CLIENT_ID=')) {
      envContent = envContent.replace(
        /REACT_APP_AUTH0_CLIENT_ID=.*/g,
        `REACT_APP_AUTH0_CLIENT_ID=${clientId}`
      );
    } else {
      envContent = `REACT_APP_AUTH0_CLIENT_ID=${clientId}\n` + envContent;
    }
    
    fs.writeFileSync(envPath, envContent);
    this.log('✅ Environment file updated', 'green');
  }

  async setVercelEnvironmentVariables() {
    this.log('\n🚀 Setting Vercel environment variables...', 'cyan');
    
    try {
      // Check if vercel is installed and user is logged in
      execSync('vercel --version', { encoding: 'utf8' });
      execSync('vercel auth whoami', { encoding: 'utf8' });
    } catch (error) {
      this.log('⚠️  Vercel CLI not available or not logged in', 'yellow');
      this.log('Please set environment variables manually in Vercel dashboard', 'yellow');
      return;
    }

    const domain = this.appConfig.domain;
    const clientId = this.appConfig.client_id;

    try {
      // Set environment variables for production
      await this.runCommand(
        `echo "${domain}" | vercel env add REACT_APP_AUTH0_DOMAIN production`,
        'Setting Auth0 domain in Vercel'
      );

      await this.runCommand(
        `echo "${clientId}" | vercel env add REACT_APP_AUTH0_CLIENT_ID production`,
        'Setting Auth0 client ID in Vercel'
      );

      this.log('✅ Vercel environment variables set', 'green');
    } catch (error) {
      this.log('⚠️  Failed to set Vercel environment variables automatically', 'yellow');
      this.log('Please set them manually in the Vercel dashboard:', 'yellow');
      this.log(`  REACT_APP_AUTH0_DOMAIN = ${domain}`, 'cyan');
      this.log(`  REACT_APP_AUTH0_CLIENT_ID = ${clientId}`, 'cyan');
    }
  }

  async enableSocialConnections() {
    this.log('\n🔗 Enabling social connections...', 'cyan');
    
    try {
      // Enable Google connection (most common)
      await this.runCommand(
        'auth0 connections create --type google-oauth2 --name google --enabled',
        'Enabling Google OAuth connection'
      );
    } catch (error) {
      this.log('ℹ️  Google connection may already exist or require additional setup', 'blue');
    }
  }

  displaySummary() {
    this.log('\n🎉 Auth0 Setup Complete!', 'bright');
    this.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'cyan');
    
    this.log('\n📊 Application Details:', 'yellow');
    this.log(`   Name: ${this.appConfig.name}`, 'cyan');
    this.log(`   Domain: ${this.appConfig.domain}`, 'cyan');
    this.log(`   Client ID: ${this.appConfig.client_id}`, 'cyan');
    this.log(`   Type: Single Page Application`, 'cyan');
    
    this.log('\n🔧 Environment Variables Set:', 'yellow');
    this.log(`   REACT_APP_AUTH0_DOMAIN=${this.appConfig.domain}`, 'cyan');
    this.log(`   REACT_APP_AUTH0_CLIENT_ID=${this.appConfig.client_id}`, 'cyan');
    
    this.log('\n📧 Next Steps:', 'yellow');
    this.log('   1. Update allowed emails in src/components/AccessControl.tsx', 'cyan');
    this.log('   2. Deploy your application: vercel --prod', 'cyan');
    this.log('   3. Test authentication with allowed email addresses', 'cyan');
    
    this.log('\n🌐 URLs Configured:', 'yellow');
    this.log(`   Production: ${this.productionUrl}`, 'cyan');
    this.log(`   Development: ${this.localUrl}`, 'cyan');
    
    this.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'cyan');
    this.log('Your Agent Explorer is now secured with Auth0! 🔐✨', 'green');
  }

  async run() {
    try {
      this.log('🚀 Auth0 Automated Setup for Agent Explorer', 'bright');
      this.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'cyan');
      
      // Check if already logged in
      const isLoggedIn = await this.checkAuth0Login();
      if (!isLoggedIn) {
        await this.loginToAuth0();
      }
      
      // Create Auth0 application
      await this.createApplication();
      
      // Update local environment file
      await this.updateEnvironmentFile();
      
      // Set Vercel environment variables
      await this.setVercelEnvironmentVariables();
      
      // Enable social connections
      await this.enableSocialConnections();
      
      // Display summary
      this.displaySummary();
      
    } catch (error) {
      this.log(`\n❌ Setup failed: ${error.message}`, 'red');
      this.log('\nPlease check the error above and try again, or follow the manual setup guide in AUTH0_SETUP.md', 'yellow');
      process.exit(1);
    }
  }
}

// Run the setup if called directly
if (require.main === module) {
  const setup = new Auth0Setup();
  setup.run();
}

module.exports = Auth0Setup;