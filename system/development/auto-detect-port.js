// auto-detect-port.js
// This script finds an available port and updates the Vite and Tauri configuration
// Add this to your project and call it from your package.json scripts

const fs = require('fs');
const path = require('path');
const net = require('net');
const { execSync } = require('child_process');

// Configuration
const DEFAULT_PORT = 28973;
const PORT_RANGE = 10; // Try up to 10 ports after the default
const TAURI_CONFIG_PATH = path.join(__dirname, 'src-tauri', 'tauri.conf.json');

// Function to check if a port is available
function isPortAvailable(port) {
  return new Promise((resolve) => {
    const server = net.createServer();
    
    server.once('error', () => {
      resolve(false); // Port is in use
    });
    
    server.once('listening', () => {
      server.close();
      resolve(true); // Port is available
    });
    
    server.listen(port);
  });
}

// Function to find an available port
async function findAvailablePort(startPort, range) {
  // First check if the default port is available
  if (await isPortAvailable(startPort)) {
    return startPort;
  }
  
  // Try subsequent ports in the range
  for (let i = 1; i <= range; i++) {
    const port = startPort + i;
    if (await isPortAvailable(port)) {
      return port;
    }
  }
  
  throw new Error(`No available ports found in range ${startPort}-${startPort + range}`);
}

// Function to update Tauri config with new port
function updateTauriConfig(port) {
  if (!fs.existsSync(TAURI_CONFIG_PATH)) {
    console.error(`Tauri config not found at ${TAURI_CONFIG_PATH}`);
    return false;
  }
  
  try {
    const config = JSON.parse(fs.readFileSync(TAURI_CONFIG_PATH, 'utf8'));
    
    // Update the devPath with the new port
    if (config.build && config.build.devPath) {
      config.build.devPath = `http://localhost:${port}`;
      fs.writeFileSync(TAURI_CONFIG_PATH, JSON.stringify(config, null, 2));
      console.log(`Updated Tauri config to use port ${port}`);
      return true;
    } else {
      console.error('Could not find build.devPath in Tauri config');
      return false;
    }
  } catch (error) {
    console.error('Error updating Tauri config:', error);
    return false;
  }
}

// Main function
async function main() {
  try {
    console.log(`Checking if default port ${DEFAULT_PORT} is available...`);
    
    // Find an available port
    const port = await findAvailablePort(DEFAULT_PORT, PORT_RANGE);
    
    if (port === DEFAULT_PORT) {
      console.log(`Using default port ${DEFAULT_PORT}`);
    } else {
      console.log(`Default port ${DEFAULT_PORT} is in use, using port ${port} instead`);
      
      // Update Tauri config
      if (!updateTauriConfig(port)) {
        throw new Error('Failed to update Tauri configuration');
      }
      
      // Set environment variable for Vite to use
      process.env.VITE_PORT = port;
    }
    
    // Return the selected port
    console.log(`PORT=${port}`);
    return port;
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Execute main function if this script is run directly
if (require.main === module) {
  main();
} else {
  // Export for use in other scripts
  module.exports = { findAvailablePort, updateTauriConfig };
}