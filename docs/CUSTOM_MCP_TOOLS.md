# Custom MCP Tool Examples

This document outlines five practical custom MCP tools that could be built to extend Claude Code's capabilities for specific development workflows.

## 1. Database Schema Explorer

A tool that connects to your database, extracts schema information, and provides it to Claude for better context when working with database code.

### Functionality:
- Connect to various database types (PostgreSQL, MySQL, MongoDB, etc.)
- Extract table/collection schemas, relationships, and indexes
- Generate database diagrams on demand
- Run explain plans on queries for optimization
- Track schema changes over time

### Example implementation:
```javascript
// Basic PostgreSQL schema explorer MCP tool
const { Pool } = require('pg');
const express = require('express');
const app = express();
app.use(express.json());

const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

// Define the tool in MCP format
app.post('/tools/schema', async (req, res) => {
  try {
    const { table } = req.body.params;
    
    // Get table schema
    const result = await pool.query(`
      SELECT column_name, data_type, is_nullable
      FROM information_schema.columns
      WHERE table_name = $1
    `, [table]);
    
    res.json({ schema: result.rows });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000, () => {
  console.log('Database Schema Explorer MCP running on port 3000');
});
```

## 2. Test Case Generator

A tool that analyzes your code and generates comprehensive test cases based on function signatures, doc comments, and existing tests.

### Functionality:
- Parse function signatures and parameters
- Extract doc comments to understand expected behavior
- Identify edge cases based on parameter types
- Generate unit test code in the appropriate testing framework
- Track test coverage and suggest areas needing more tests

### Example implementation:
```python
# Basic Python test case generator MCP tool
import ast
import inspect
import json
from flask import Flask, request, jsonify

app = Flask(__name__)

def parse_function(code_path, function_name):
    with open(code_path, 'r') as file:
        code = file.read()
    
    tree = ast.parse(code)
    for node in ast.walk(tree):
        if isinstance(node, ast.FunctionDef) and node.name == function_name:
            # Extract parameters, docstring, and function body
            params = [arg.arg for arg in node.args.args]
            docstring = ast.get_docstring(node)
            return {
                "name": function_name,
                "params": params,
                "docstring": docstring
            }
    return None

@app.route('/tools/generate-test', methods=['POST'])
def generate_test():
    data = request.json
    path = data['params']['file_path']
    function = data['params']['function_name']
    
    function_info = parse_function(path, function)
    if not function_info:
        return jsonify({"error": "Function not found"})
    
    # Generate test template
    test_code = f"""
import unittest
from {path.replace('.py', '').replace('/', '.')} import {function}

class Test{function.capitalize()}(unittest.TestCase):
    def test_{function}_basic(self):
        # Test basic functionality
        result = {function}({', '.join(['None' for _ in function_info['params']])})
        self.assertEqual(result, expected_result)
        
    def test_{function}_edge_cases(self):
        # Test edge cases
        pass
        
    def test_{function}_error_handling(self):
        # Test error handling
        with self.assertRaises(Exception):
            {function}(invalid_input)
"""
    return jsonify({"test_code": test_code})

if __name__ == '__main__':
    app.run(port=3001)
```

## 3. Performance Profiler

A tool that runs performance measurements on your code and provides insights to Claude for optimization suggestions.

### Functionality:
- Run CPU and memory profiling on specified code segments
- Identify bottlenecks and hot spots
- Compare performance before and after changes
- Generate flame graphs and performance reports
- Track performance metrics over time

### Example implementation:
```javascript
// Basic Node.js performance profiler MCP tool
const express = require('express');
const { execSync } = require('child_process');
const fs = require('fs');
const app = express();
app.use(express.json());

// Define the profiler tool
app.post('/tools/profile', async (req, res) => {
  try {
    const { file_path, function_name } = req.body.params;
    
    // Create temporary profiling script
    const profileScript = `
      const { performance } = require('perf_hooks');
      const targetModule = require('${file_path}');
      
      // Run profiling
      const NUM_RUNS = 100;
      const start = performance.now();
      
      for (let i = 0; i < NUM_RUNS; i++) {
        targetModule.${function_name}();
      }
      
      const end = performance.now();
      console.log(JSON.stringify({
        function: '${function_name}',
        average_ms: (end - start) / NUM_RUNS,
        total_ms: (end - start)
      }));
    `;
    
    fs.writeFileSync('temp_profile.js', profileScript);
    
    // Run the profiling script
    const result = execSync('node --prof temp_profile.js');
    const metrics = JSON.parse(result.toString());
    
    // Process the v8 profiling data
    execSync('node --prof-process isolate-*.log > profile_analysis.txt');
    const analysis = fs.readFileSync('profile_analysis.txt', 'utf8');
    
    // Clean up
    fs.unlinkSync('temp_profile.js');
    fs.unlinkSync('profile_analysis.txt');
    
    res.json({
      metrics,
      analysis_summary: analysis.split('\n').slice(0, 20).join('\n')
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3002, () => {
  console.log('Performance Profiler MCP running on port 3002');
});
```

## 4. API Documentation Generator

A tool that analyzes your API endpoints and generates comprehensive documentation, enabling Claude to better understand and modify your API.

### Functionality:
- Scan code for API endpoints in various frameworks (Express, Flask, etc.)
- Extract parameters, response formats, and authentication requirements
- Generate OpenAPI/Swagger documentation
- Test endpoints and document example responses
- Track API changes for versioning

### Example implementation:
```python
# Basic Python API documentation generator MCP tool
import os
import json
import re
from flask import Flask, request, jsonify

app = Flask(__name__)

def extract_express_routes(directory):
    routes = []
    
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.js'):
                path = os.path.join(root, file)
                with open(path, 'r') as f:
                    content = f.read()
                    
                    # Simple regex to find Express route definitions
                    # This is a basic example - a real implementation would be more robust
                    route_patterns = re.finditer(
                        r'(app|router)\.(get|post|put|delete|patch)\([\'"]([^\'"]+)[\'"]',
                        content
                    )
                    
                    for match in route_patterns:
                        method = match.group(2).upper()
                        route_path = match.group(3)
                        
                        routes.append({
                            "file": path,
                            "method": method,
                            "path": route_path
                        })
    
    return routes

@app.route('/tools/document-api', methods=['POST'])
def document_api():
    data = request.json
    directory = data['params']['directory']
    
    routes = extract_express_routes(directory)
    
    # Generate OpenAPI format
    openapi_doc = {
        "openapi": "3.0.0",
        "info": {
            "title": "API Documentation",
            "version": "1.0.0"
        },
        "paths": {}
    }
    
    for route in routes:
        path = route["path"]
        method = route["method"].lower()
        
        if path not in openapi_doc["paths"]:
            openapi_doc["paths"][path] = {}
            
        openapi_doc["paths"][path][method] = {
            "summary": f"{method.upper()} {path}",
            "parameters": [],
            "responses": {
                "200": {
                    "description": "Successful response"
                }
            }
        }
    
    return jsonify({"documentation": openapi_doc})

if __name__ == '__main__':
    app.run(port=3003)
```

## 5. Dependency Analyzer

A tool that scans your project's dependencies, identifies security vulnerabilities, version conflicts, and unused packages.

### Functionality:
- Scan package.json, requirements.txt, and other dependency files
- Check for known vulnerabilities (CVEs)
- Identify outdated packages
- Find unused dependencies
- Suggest dependency updates and alternatives

### Example implementation:
```javascript
// Basic dependency analyzer MCP tool
const express = require('express');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const app = express();
app.use(express.json());

app.post('/tools/analyze-dependencies', async (req, res) => {
  try {
    const { project_path } = req.body.params;
    
    // Check if it's a Node.js project
    const packageJsonPath = path.join(project_path, 'package.json');
    if (fs.existsSync(packageJsonPath)) {
      // Run npm audit
      let auditResult = {};
      try {
        const auditOutput = execSync('npm audit --json', { 
          cwd: project_path,
          stdio: ['ignore', 'pipe', 'ignore']
        });
        auditResult = JSON.parse(auditOutput.toString());
      } catch (e) {
        // npm audit exits with non-zero code when vulnerabilities are found
        auditResult = JSON.parse(e.stdout.toString());
      }
      
      // Run npm outdated
      let outdatedResult = {};
      try {
        const outdatedOutput = execSync('npm outdated --json', { 
          cwd: project_path,
          stdio: ['ignore', 'pipe', 'ignore']
        });
        outdatedResult = JSON.parse(outdatedOutput.toString());
      } catch (e) {
        // npm outdated exits with non-zero code when outdated packages are found
        if (e.stdout) {
          outdatedResult = JSON.parse(e.stdout.toString());
        }
      }
      
      // Find potentially unused dependencies
      const depCheckOutput = execSync('npx depcheck --json', { 
        cwd: project_path,
        stdio: ['ignore', 'pipe', 'ignore']
      });
      const unusedResult = JSON.parse(depCheckOutput.toString());
      
      res.json({
        vulnerabilities: auditResult.vulnerabilities || {},
        outdated: outdatedResult,
        unused: unusedResult.dependencies || []
      });
    } 
    // Check if it's a Python project
    else if (fs.existsSync(path.join(project_path, 'requirements.txt'))) {
      const safetyOutput = execSync('pip-audit -r requirements.txt --format json', { 
        cwd: project_path,
        stdio: ['ignore', 'pipe', 'ignore']
      });
      
      res.json({
        vulnerabilities: JSON.parse(safetyOutput.toString())
      });
    } else {
      res.json({
        error: "Unsupported project type. Please provide a Node.js or Python project."
      });
    }
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3004, () => {
  console.log('Dependency Analyzer MCP running on port 3004');
});
```

## Setting Up Your Custom MCP Tool

To use any of these custom MCP tools with Claude Code:

1. Implement the tool as a small service (using Express, Flask, etc.)
2. Configure Claude Code to use your custom MCP server:

```bash
claude mcp add database-explorer http://localhost:3000
claude mcp add test-generator http://localhost:3001
claude mcp add profiler http://localhost:3002
claude mcp add api-documenter http://localhost:3003
claude mcp add dependency-analyzer http://localhost:3004
```

3. Update your `.claude.json` configuration:

```json
{
  "mcpServers": {
    "custom-tools": {
      "type": "http",
      "url": "http://localhost:3000",
      "tools": ["schema", "generate-test", "profile", "document-api", "analyze-dependencies"]
    }
  }
}
```

4. After restarting Claude Code, use the tools in your workflow:

```
I need to optimize the database queries in src/database/users.js. Can you analyze the schema?
```

Claude will now have access to these custom tools to enhance its capabilities and provide more powerful assistance in your specific workflow.