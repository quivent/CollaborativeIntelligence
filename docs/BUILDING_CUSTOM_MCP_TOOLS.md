# Building and Integrating Custom MCP Tools

This guide provides step-by-step instructions for creating and integrating custom Model Context Protocol (MCP) tools with Claude Code.

## 1. Set Up Development Environment

```bash
# Create a project directory
mkdir custom-mcp-tools
cd custom-mcp-tools

# Initialize package.json for Node.js tools
npm init -y

# Install base dependencies
npm install express cors dotenv

# For Python tools, create a virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install flask requests
```

## 2. Implement a Single Tool First

Start with one tool (like the Database Schema Explorer):

```javascript
// schema-explorer.js
require('dotenv').config();
const express = require('express');
const cors = require('cors');
const { Pool } = require('pg');

const app = express();
app.use(express.json());
app.use(cors());

// Create database connection
const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

// Implement MCP tool endpoints
app.post('/tools/schema', async (req, res) => {
  try {
    const { table } = req.body.params || {};
    
    if (!table) {
      return res.json({
        tables: await listTables()
      });
    }
    
    const columns = await getTableSchema(table);
    return res.json({ schema: columns });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: error.message });
  }
});

async function listTables() {
  const result = await pool.query(`
    SELECT table_name 
    FROM information_schema.tables
    WHERE table_schema = 'public'
  `);
  return result.rows.map(row => row.table_name);
}

async function getTableSchema(table) {
  const result = await pool.query(`
    SELECT 
      column_name, 
      data_type, 
      is_nullable,
      column_default
    FROM information_schema.columns
    WHERE table_name = $1
  `, [table]);
  return result.rows;
}

// Start the server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`MCP Schema Explorer running on port ${PORT}`);
});
```

Create a `.env` file:
```
DATABASE_URL=postgresql://username:password@localhost:5432/your_database
```

## 3. Test Your Tool Independently

Before integrating with Claude, test your tool with curl or Postman:

```bash
curl -X POST http://localhost:3000/tools/schema \
  -H "Content-Type: application/json" \
  -d '{"params": {"table": "users"}}'
```

## 4. Create MCP Tool Manifest

Each MCP server needs a manifest that describes available tools:

```javascript
// Add this to schema-explorer.js before the app.listen line
app.get('/tools', (req, res) => {
  res.json({
    schema: {
      description: "Get database schema information",
      parameters: {
        type: "object",
        properties: {
          table: {
            type: "string",
            description: "Optional. The name of the table to get schema for. If omitted, returns a list of all tables."
          }
        }
      }
    }
  });
});
```

## 5. Register Your Tool with Claude Code

```bash
# Start your tool server
node schema-explorer.js

# In a new terminal, register the tool with Claude Code
claude mcp add schema-explorer http://localhost:3000
```

## 6. Test with Claude Code

```bash
claude
```

Inside Claude Code, try using your tool:
```
Can you look at my database schema and suggest optimizations for the users table?
```

## 7. Create a Comprehensive MCP Server

For efficiency, combine multiple tools into a single MCP server:

```javascript
// comprehensive-tools.js
const express = require('express');
const cors = require('cors');
const { Pool } = require('pg');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const app = express();
app.use(express.json());
app.use(cors());

// Database connection
const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

// Tool manifest
app.get('/tools', (req, res) => {
  res.json({
    schema: {
      description: "Get database schema information",
      parameters: {
        type: "object",
        properties: {
          table: { type: "string", description: "Table name" }
        }
      }
    },
    dependencies: {
      description: "Analyze project dependencies",
      parameters: {
        type: "object",
        properties: {
          project_path: { type: "string", description: "Path to project root" }
        },
        required: ["project_path"]
      }
    }
    // Add more tool definitions here
  });
});

// Implement schema tool
app.post('/tools/schema', async (req, res) => {
  // Implementation as before
});

// Implement dependency analyzer
app.post('/tools/dependencies', async (req, res) => {
  // Implementation from example
});

// Start server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Comprehensive MCP Tools running on port ${PORT}`);
});
```

## 8. Create an Installation Process

Package your tools for easier distribution:

1. Create a `package.json` with all dependencies
2. Add a README with setup instructions
3. Add an installation script:

```javascript
// install.js
const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

// Install dependencies
console.log('Installing dependencies...');
execSync('npm install', { stdio: 'inherit' });

// Create .env file if it doesn't exist
const envPath = path.join(__dirname, '.env');
if (!fs.existsSync(envPath)) {
  console.log('Creating .env file...');
  fs.writeFileSync(envPath, 'DATABASE_URL=postgresql://username:password@localhost:5432/your_database\nPORT=3000\n');
  console.log('.env file created. Please update with your settings.');
}

// Register with Claude Code
console.log('Registering with Claude Code...');
try {
  execSync('claude mcp add custom-tools http://localhost:3000', { stdio: 'inherit' });
  console.log('Tools registered successfully!');
} catch (error) {
  console.log('Failed to register tools. Please run Claude Code and try manual registration.');
}
```

## 9. Deploy for Team Use

For team-wide tool access:

1. Deploy your MCP server to a shared internal server or cloud service
2. Update your `.mcp.json` to use the shared URL:

```json
{
  "mcpServers": {
    "team-tools": {
      "type": "http",
      "url": "https://internal-mcp-server.example.com",
      "tools": ["schema", "dependencies", "profile", "document-api", "generate-test"]
    }
  }
}
```

3. Commit this file to your repository so all team members can use these tools

## 10. Documentation and Ongoing Maintenance

Create documentation for your tools:
- Which tools are available
- What each tool does
- Example usage with Claude
- Configuration options

Set up a monitoring system to ensure your MCP tools stay operational and update them as needed when your internal systems change.

## Tool Permission Considerations

When building custom MCP tools, consider permissions carefully:

1. **Least Privilege**: Tools should only request the minimum permissions necessary
2. **Explicit Access Controls**: Add checks for sensitive operations
3. **Logging**: Track tool usage for auditing
4. **Sandboxing**: Restrict what each tool can access
5. **Rate Limiting**: Prevent abuse of resources

## Advanced Features

Once your basic tools are working, consider these enhancements:

### 1. Authentication

Add authentication to your MCP server:

```javascript
// Basic API key authentication
app.use((req, res, next) => {
  const apiKey = req.headers['x-api-key'];
  if (!apiKey || apiKey !== process.env.API_KEY) {
    return res.status(401).json({ error: 'Invalid API key' });
  }
  next();
});
```

### 2. Streaming Responses

For long-running operations, implement streaming:

```javascript
app.post('/tools/long-process', (req, res) => {
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Connection', 'keep-alive');
  
  const interval = setInterval(() => {
    res.write(`data: ${JSON.stringify({ progress: "25%" })}\n\n`);
  }, 1000);
  
  // When done:
  setTimeout(() => {
    clearInterval(interval);
    res.write(`data: ${JSON.stringify({ result: "Complete", data: {} })}\n\n`);
    res.end();
  }, 5000);
});
```

### 3. Tool Composability

Design tools that can work together:

```javascript
// First tool generates data
app.post('/tools/generate-data', (req, res) => {
  // Generate data
  const data = { /* ... */ };
  
  // Store data in a shared location
  SHARED_DATA[req.body.session_id] = data;
  
  res.json({ data_id: req.body.session_id });
});

// Second tool processes that data
app.post('/tools/process-data', (req, res) => {
  const data = SHARED_DATA[req.body.params.data_id];
  // Process data
  res.json({ result: "Processed!" });
});
```

By starting with simple tools and gradually adding more sophisticated features, you can build a powerful ecosystem of custom MCP tools that significantly enhance Claude's capabilities for your specific workflow.