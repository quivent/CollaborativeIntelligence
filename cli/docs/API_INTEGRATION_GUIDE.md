# API Integration Guide

## Overview

This guide provides comprehensive documentation for integrating with the CI Coordination System APIs, including Python script integration, JSON data structures, configuration formats, and export file formats. Use this documentation to build custom tools, automation scripts, and third-party integrations.

## Integration Architecture

```
┌─────────────────┐    ┌──────────────────────┐    ┌─────────────────┐
│   CLI Commands  │    │  Python Coordinator  │    │  JSON Storage   │
│                 │    │                      │    │                 │
│  ci coordination│───▶│ session_coordinator  │───▶│ SESSION_        │
│  ci dashboard   │    │       .py            │    │ COORDINATION    │
│                 │    │                      │    │   .json         │
└─────────────────┘    └──────────────────────┘    └─────────────────┘
         │                       │                          │
         │                       │                          │
         ▼                       ▼                          ▼
┌─────────────────┐    ┌──────────────────────┐    ┌─────────────────┐
│   Web Dashboard │    │  Development Log     │    │  Report Files   │
│   HTTP Server   │    │                      │    │                 │
│                 │    │ DEVELOPMENT_LOG      │    │ .md, .html,     │
│ REST API        │    │      .jsonl          │    │ .csv, .json     │
│ Endpoints       │    │                      │    │                 │
└─────────────────┘    └──────────────────────┘    └─────────────────┘
```

## Python Script Integration

### Direct Python Integration

#### Basic Session Management
```python
import subprocess
import json
from pathlib import Path

class CoordinationClient:
    """Python client for CI coordination system"""
    
    def __init__(self, brain_path="/path/to/CollaborativeIntelligence/BRAIN"):
        self.script_path = Path(brain_path) / "coordination" / "session_coordinator.py"
        
    def _run_command(self, args):
        """Execute coordination script with error handling"""
        cmd = ["python3", str(self.script_path)] + args
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"Coordination command failed: {result.stderr}")
            
        return result.stdout.strip()
    
    def register_session(self, session_id=None, lane=None, claude_version=None):
        """Register a new coordination session"""
        args = ["register"]
        
        if session_id:
            args.extend(["--session", session_id])
        if lane:
            args.extend(["--lane", lane])
        if claude_version:
            args.extend(["--claude-version", claude_version])
            
        output = self._run_command(args)
        
        # Extract session ID from output
        for line in output.split('\n'):
            if 'Registered session:' in line:
                return line.split(': ')[1]
        
        return None
    
    def send_heartbeat(self, session_id, tasks_completed=None, tasks_reserved=None):
        """Send heartbeat for session"""
        args = ["heartbeat", "--session", session_id]
        
        if tasks_completed is not None:
            args.extend(["--tasks-completed", str(tasks_completed)])
        if tasks_reserved is not None:
            args.extend(["--tasks-reserved", str(tasks_reserved)])
            
        self._run_command(args)
        return True
    
    def get_session_status(self, session_id):
        """Get detailed session status"""
        args = ["status", "--session", session_id, "--format", "json"]
        output = self._run_command(args)
        return json.loads(output)
    
    def get_system_status(self):
        """Get system-wide status"""
        args = ["status", "--system", "--format", "json"]
        output = self._run_command(args)
        return json.loads(output)
    
    def get_all_sessions(self, status_filter=None):
        """Get all sessions with optional status filter"""
        args = ["status", "--format", "json"]
        
        if status_filter == "active":
            args.append("--active")
        else:
            args.append("--all")
            
        output = self._run_command(args)
        return json.loads(output)
    
    def end_session(self, session_id, reason="Python integration termination"):
        """End session gracefully"""
        args = ["end", "--session", session_id, "--reason", reason]
        self._run_command(args)
        return True

# Usage example
if __name__ == "__main__":
    client = CoordinationClient()
    
    # Start session
    session_id = client.register_session(
        lane="python-integration",
        claude_version="python-client-1.0"
    )
    print(f"Started session: {session_id}")
    
    # Send heartbeat with progress
    client.send_heartbeat(session_id, tasks_completed=3, tasks_reserved=2)
    
    # Get status
    status = client.get_session_status(session_id)
    print(f"Session status: {status['status']}")
    
    # End session
    client.end_session(session_id)
```

#### Advanced Integration with Context Manager
```python
from contextlib import contextmanager
import time
import threading

class CoordinationSession:
    """Context manager for coordination sessions with automatic heartbeat"""
    
    def __init__(self, client, lane=None, claude_version=None, context=None):
        self.client = client
        self.lane = lane
        self.claude_version = claude_version
        self.context = context
        self.session_id = None
        self.heartbeat_thread = None
        self.active = False
        
    def __enter__(self):
        """Start coordination session"""
        self.session_id = self.client.register_session(
            lane=self.lane,
            claude_version=self.claude_version
        )
        
        self.active = True
        self.heartbeat_thread = threading.Thread(target=self._heartbeat_loop)
        self.heartbeat_thread.daemon = True
        self.heartbeat_thread.start()
        
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """End coordination session"""
        self.active = False
        
        if self.heartbeat_thread:
            self.heartbeat_thread.join(timeout=1)
        
        if self.session_id:
            reason = "Normal completion"
            if exc_type:
                reason = f"Exception: {exc_type.__name__}"
            
            self.client.end_session(self.session_id, reason)
    
    def _heartbeat_loop(self):
        """Background heartbeat thread"""
        while self.active:
            try:
                if self.session_id:
                    self.client.send_heartbeat(self.session_id)
            except Exception as e:
                print(f"Heartbeat failed: {e}")
            
            time.sleep(300)  # 5-minute interval
    
    def update_progress(self, tasks_completed=None, tasks_reserved=None):
        """Update task progress"""
        if self.session_id:
            self.client.send_heartbeat(
                self.session_id,
                tasks_completed=tasks_completed,
                tasks_reserved=tasks_reserved
            )

# Usage with context manager
def automated_workflow():
    client = CoordinationClient()
    
    with CoordinationSession(client, lane="automation", claude_version="auto-1.0") as session:
        print(f"Working in session: {session.session_id}")
        
        # Simulate work with progress updates
        session.update_progress(tasks_reserved=5)
        
        for i in range(5):
            time.sleep(10)  # Simulate work
            session.update_progress(tasks_completed=i+1, tasks_reserved=5-i-1)
            print(f"Completed task {i+1}/5")
        
        print("All tasks completed!")
    
    print("Session ended automatically")

if __name__ == "__main__":
    automated_workflow()
```

## JSON Data Structures

### Session Coordination File Structure

#### SESSION_COORDINATION.json Format
```json
{
  "version": "1.0",
  "last_updated": "2025-09-01T15:30:45.123456",
  "sessions": {
    "claude-hostname-1693234567-abc12345": {
      "id": "claude-hostname-1693234567-abc12345",
      "status": "active",
      "registered_at": "2025-09-01T15:00:00.000000",
      "last_heartbeat": "2025-09-01T15:30:00.000000",
      "lane": "development",
      "claude_version": "claude-3.5-sonnet",
      "user_info": {
        "context": "Feature development",
        "custom_field": "value"
      },
      "host_info": {
        "hostname": "dev-machine.local",
        "platform": "darwin"
      },
      "activity": {
        "tasks_reserved": 2,
        "tasks_completed": 3,
        "last_activity": "2025-09-01T15:25:00.000000"
      },
      "heartbeat_count": 7,
      "missed_heartbeats": 0,
      "ended_at": null,
      "end_reason": null,
      "duration_seconds": null,
      "timeout_reason": null
    }
  },
  "coordination": {
    "active_sessions": 3,
    "total_sessions_registered": 15,
    "session_timeouts": 1,
    "last_cleanup": "2025-09-01T15:00:00.000000",
    "heartbeat_interval_seconds": 300
  },
  "metadata": {
    "max_concurrent_sessions": 10,
    "session_timeout_minutes": 15,
    "auto_cleanup_enabled": true,
    "last_update": "2025-09-01T15:30:45.123456"
  }
}
```

### Development Log Format

#### DEVELOPMENT_LOG.jsonl Format
Each line is a separate JSON object:
```json
{"timestamp": "2025-09-01T15:00:00.123456", "category": "session_activity", "action": "session_registered", "session": "claude-dev-001", "lane": "frontend", "claude_version": "claude-3.5-sonnet", "hostname": "dev-machine"}
{"timestamp": "2025-09-01T15:05:00.123456", "category": "session_activity", "action": "heartbeat", "session": "claude-dev-001", "heartbeat_count": 2, "activity_data": {"tasks_completed": 1}}
{"timestamp": "2025-09-01T15:10:00.123456", "category": "session_activity", "action": "task_reserved", "session": "claude-dev-001", "task_id": "feature-auth", "priority": "high"}
{"timestamp": "2025-09-01T15:30:00.123456", "category": "session_activity", "action": "session_ended", "session": "claude-dev-001", "reason": "Work completed", "duration_seconds": 1800, "tasks_completed": 3}
```

### API Response Formats

#### System Status Response
```json
{
  "system_health": {
    "status": "healthy",
    "active_sessions": 3,
    "max_concurrent": 10,
    "utilization_percent": 30,
    "timeout_rate": 0.067
  },
  "session_counts": {
    "active": 3,
    "ended": 12,
    "timed_out": 1
  },
  "lane_assignments": {
    "frontend": 2,
    "backend": 1,
    "testing": 0
  },
  "coordination": {
    "active_sessions": 3,
    "total_sessions_registered": 16,
    "session_timeouts": 1,
    "last_cleanup": "2025-09-01T15:00:00.000000",
    "heartbeat_interval_seconds": 300
  },
  "metadata": {
    "max_concurrent_sessions": 10,
    "session_timeout_minutes": 15,
    "auto_cleanup_enabled": true,
    "last_update": "2025-09-01T15:30:45.123456"
  },
  "last_updated": "2025-09-01T15:30:45.123456"
}
```

#### Session Detail Response
```json
{
  "id": "claude-hostname-1693234567-abc12345",
  "status": "active",
  "registered_at": "2025-09-01T15:00:00.000000",
  "last_heartbeat": "2025-09-01T15:30:00.000000",
  "lane": "development",
  "claude_version": "claude-3.5-sonnet",
  "user_info": {
    "context": "Feature development"
  },
  "host_info": {
    "hostname": "dev-machine.local",
    "platform": "darwin"
  },
  "activity": {
    "tasks_reserved": 2,
    "tasks_completed": 3,
    "last_activity": "2025-09-01T15:25:00.000000"
  },
  "heartbeat_count": 7,
  "missed_heartbeats": 0,
  "time_since_last_heartbeat_seconds": 45,
  "current_duration_seconds": 1800,
  "likely_stale": false
}
```

#### Session List Response
```json
[
  {
    "id": "claude-hostname-1693234567-abc12345",
    "status": "active",
    "registered_at": "2025-09-01T15:00:00.000000",
    "last_heartbeat": "2025-09-01T15:30:00.000000",
    "lane": "development",
    "claude_version": "claude-3.5-sonnet",
    "hostname": "dev-machine.local",
    "tasks_reserved": 2,
    "tasks_completed": 3,
    "heartbeat_count": 7,
    "duration_seconds": 1800
  },
  {
    "id": "claude-hostname-1693234123-def67890",
    "status": "ended",
    "registered_at": "2025-09-01T14:30:00.000000",
    "last_heartbeat": "2025-09-01T14:55:00.000000",
    "lane": "testing",
    "claude_version": "claude-3.5-sonnet",
    "hostname": "test-machine.local",
    "tasks_reserved": 0,
    "tasks_completed": 5,
    "heartbeat_count": 6,
    "duration_seconds": 1500
  }
]
```

## REST API Endpoints

### Web Dashboard API

When the web dashboard is running, it exposes the following REST endpoints:

#### GET /api/system
Returns system-wide status and health metrics.

**Response**: System Status JSON (see above)

**Example**:
```bash
curl http://localhost:8080/api/system | jq .
```

#### GET /api/sessions
Returns list of all sessions with summary information.

**Response**: Session List JSON (see above)

**Example**:
```bash
curl http://localhost:8080/api/sessions | jq .
```

#### GET /api/tasks
Returns aggregated task metrics across all sessions.

**Response**:
```json
{
  "total_completed": 25,
  "total_reserved": 8,
  "active_sessions": 3,
  "sessions": [
    {
      "id": "session-id",
      "tasks_completed": 5,
      "tasks_reserved": 2,
      "status": "active"
    }
  ]
}
```

#### GET /api/lanes
Returns lane assignment distribution.

**Response**:
```json
{
  "frontend": 2,
  "backend": 1,
  "testing": 0,
  "documentation": 1
}
```

### Custom API Integration

#### Node.js Integration Example
```javascript
const axios = require('axios');

class CoordinationAPI {
    constructor(dashboardUrl = 'http://localhost:8080') {
        this.baseUrl = dashboardUrl;
    }
    
    async getSystemStatus() {
        try {
            const response = await axios.get(`${this.baseUrl}/api/system`);
            return response.data;
        } catch (error) {
            throw new Error(`Failed to get system status: ${error.message}`);
        }
    }
    
    async getAllSessions() {
        try {
            const response = await axios.get(`${this.baseUrl}/api/sessions`);
            return response.data;
        } catch (error) {
            throw new Error(`Failed to get sessions: ${error.message}`);
        }
    }
    
    async getTaskMetrics() {
        try {
            const response = await axios.get(`${this.baseUrl}/api/tasks`);
            return response.data;
        } catch (error) {
            throw new Error(`Failed to get task metrics: ${error.message}`);
        }
    }
    
    async monitorSystem(callback, interval = 5000) {
        const monitor = async () => {
            try {
                const status = await this.getSystemStatus();
                callback(null, status);
            } catch (error) {
                callback(error, null);
            }
        };
        
        // Initial call
        await monitor();
        
        // Set up interval monitoring
        return setInterval(monitor, interval);
    }
}

// Usage example
const api = new CoordinationAPI();

api.monitorSystem((error, status) => {
    if (error) {
        console.error('Monitoring error:', error);
        return;
    }
    
    console.log(`System Health: ${status.system_health.status}`);
    console.log(`Active Sessions: ${status.system_health.active_sessions}`);
    console.log(`Utilization: ${status.system_health.utilization_percent}%`);
}, 10000); // Monitor every 10 seconds
```

## Configuration File Formats

### CLI Configuration

The CLI uses environment variables and command-line arguments for configuration. No separate configuration file is required.

#### Environment Variables
```bash
# Coordination system configuration
export CI_COORDINATION_HEARTBEAT_INTERVAL=300  # Heartbeat interval in seconds
export CI_COORDINATION_SESSION_TIMEOUT=900     # Session timeout in seconds
export CI_COORDINATION_MAX_SESSIONS=10         # Maximum concurrent sessions
export CI_COORDINATION_AUTO_CLEANUP=true       # Enable automatic cleanup

# Dashboard configuration
export CI_DASHBOARD_REFRESH_RATE=5             # Terminal dashboard refresh rate
export CI_DASHBOARD_WEB_PORT=8080              # Web dashboard port
export CI_DASHBOARD_THEME=dark                 # Dashboard theme
export CI_DASHBOARD_MAX_DISPLAY=20             # Maximum sessions in display

# Path configuration
export CI_BRAIN_ROOT="/path/to/CollaborativeIntelligence/BRAIN"
```

### Runtime Configuration

#### Dynamic Configuration via JSON
```json
{
  "coordination": {
    "heartbeat_interval_seconds": 300,
    "session_timeout_minutes": 15,
    "max_concurrent_sessions": 10,
    "auto_cleanup_enabled": true
  },
  "dashboard": {
    "refresh_rate_seconds": 5,
    "web_port": 8080,
    "theme": "dark",
    "max_sessions_display": 20
  },
  "monitoring": {
    "enable_performance_tracking": true,
    "log_level": "INFO",
    "export_formats": ["json", "csv", "markdown"]
  }
}
```

## Export File Formats

### Report Formats

#### Markdown Report Format
```markdown
# Coordination Summary Report

**Generated**: 2025-09-01T15:30:45Z
**Timeframe**: 24h

## System Health

- **Status**: HEALTHY
- **Active Sessions**: 3/10
- **Utilization**: 30%
- **Timeout Rate**: 6.7%

## Session Statistics

- **Active**: 3
- **Ended**: 12
- **Timed Out**: 1

## Lane Assignments

- **frontend**: 2 sessions
- **backend**: 1 sessions
- **testing**: 0 sessions

## Coordination Metrics

- **Total Sessions Registered**: 16
- **Session Timeouts**: 1
- **Heartbeat Interval**: 300s
- **Last Cleanup**: 2025-09-01T15:00:00Z

---
*Report generated by CI Coordination System*
```

#### CSV Activity Report Format
```csv
Session ID,Status,Lane,Duration (minutes),Tasks Completed,Tasks Reserved,Heartbeats,Registered At
claude-hostname-1693234567-abc12345,active,development,30,3,2,7,2025-09-01T15:00:00Z
claude-hostname-1693234123-def67890,ended,testing,25,5,0,6,2025-09-01T14:30:00Z
claude-hostname-1693233999-ghi34561,timed_out,frontend,45,2,1,4,2025-09-01T14:15:00Z
```

#### JSON Export Format
```json
{
  "export_metadata": {
    "generated_at": "2025-09-01T15:30:45.123456Z",
    "export_type": "system_status",
    "format_version": "1.0"
  },
  "data": {
    "system_health": {
      "status": "healthy",
      "active_sessions": 3,
      "max_concurrent": 10,
      "utilization_percent": 30,
      "timeout_rate": 0.067
    },
    "sessions": [...],
    "coordination": {...},
    "metadata": {...}
  }
}
```

### Custom Export Tools

#### Python Export Utility
```python
import json
import csv
from datetime import datetime
from pathlib import Path

class CoordinationExporter:
    """Export coordination data in various formats"""
    
    def __init__(self, client):
        self.client = client
    
    def export_to_json(self, output_path, data_type="system"):
        """Export data to JSON format"""
        if data_type == "system":
            data = self.client.get_system_status()
        elif data_type == "sessions":
            data = self.client.get_all_sessions()
        else:
            raise ValueError(f"Unknown data type: {data_type}")
        
        export_data = {
            "export_metadata": {
                "generated_at": datetime.now().isoformat() + "Z",
                "export_type": data_type,
                "format_version": "1.0"
            },
            "data": data
        }
        
        with open(output_path, 'w') as f:
            json.dump(export_data, f, indent=2)
        
        print(f"Exported {data_type} data to {output_path}")
    
    def export_sessions_to_csv(self, output_path):
        """Export session data to CSV format"""
        sessions = self.client.get_all_sessions()
        
        with open(output_path, 'w', newline='') as f:
            writer = csv.writer(f)
            
            # Header
            writer.writerow([
                'Session ID', 'Status', 'Lane', 'Duration (minutes)',
                'Tasks Completed', 'Tasks Reserved', 'Heartbeats', 'Registered At'
            ])
            
            # Data rows
            for session in sessions:
                writer.writerow([
                    session['id'],
                    session['status'],
                    session.get('lane', 'None'),
                    session.get('duration_seconds', 0) // 60,
                    session['tasks_completed'],
                    session['tasks_reserved'],
                    session['heartbeat_count'],
                    session['registered_at']
                ])
        
        print(f"Exported {len(sessions)} sessions to {output_path}")
    
    def generate_markdown_report(self, output_path, timeframe="24h"):
        """Generate markdown summary report"""
        system_data = self.client.get_system_status()
        
        report = f"""# Coordination Summary Report

**Generated**: {datetime.now().isoformat()}Z
**Timeframe**: {timeframe}

## System Health

- **Status**: {system_data['system_health']['status'].upper()}
- **Active Sessions**: {system_data['system_health']['active_sessions']}/{system_data['system_health']['max_concurrent']}
- **Utilization**: {system_data['system_health']['utilization_percent']}%
- **Timeout Rate**: {system_data['system_health']['timeout_rate']:.1%}

## Session Statistics

"""
        
        for status, count in system_data['session_counts'].items():
            report += f"- **{status.title()}**: {count}\n"
        
        report += "\n## Lane Assignments\n\n"
        
        for lane, count in system_data['lane_assignments'].items():
            report += f"- **{lane}**: {count} sessions\n"
        
        report += f"""
## Coordination Metrics

- **Total Sessions Registered**: {system_data['coordination']['total_sessions_registered']}
- **Session Timeouts**: {system_data['coordination']['session_timeouts']}
- **Heartbeat Interval**: {system_data['coordination']['heartbeat_interval_seconds']}s
- **Last Cleanup**: {system_data['coordination']['last_cleanup']}

---
*Report generated by CI Coordination System*
"""
        
        with open(output_path, 'w') as f:
            f.write(report)
        
        print(f"Generated markdown report: {output_path}")

# Usage example
if __name__ == "__main__":
    client = CoordinationClient()
    exporter = CoordinationExporter(client)
    
    # Export in different formats
    exporter.export_to_json("system_status.json", "system")
    exporter.export_to_json("sessions.json", "sessions")
    exporter.export_sessions_to_csv("sessions.csv")
    exporter.generate_markdown_report("coordination_report.md")
```

## Integration Best Practices

### Error Handling
1. **Graceful Degradation**: Handle coordination system unavailability
2. **Retry Logic**: Implement exponential backoff for failed operations
3. **Timeout Management**: Set appropriate timeouts for API calls
4. **Logging**: Comprehensive logging for debugging integration issues

### Performance Optimization
1. **Caching**: Cache frequently accessed data to reduce API calls
2. **Batching**: Group multiple operations when possible
3. **Async Operations**: Use asynchronous patterns for non-blocking operations
4. **Resource Management**: Properly manage connections and file handles

### Security Considerations
1. **Path Validation**: Validate file paths to prevent directory traversal
2. **Input Sanitization**: Sanitize all input data before processing
3. **Permission Checks**: Ensure appropriate file and directory permissions
4. **Error Information**: Avoid exposing sensitive information in error messages

### Monitoring and Observability
1. **Health Checks**: Regular verification of system availability
2. **Metrics Collection**: Track key performance indicators
3. **Alerting**: Set up alerts for system anomalies
4. **Audit Logging**: Log all significant operations for audit trails

---

**Last Updated**: 2025-09-01  
**Version**: 1.0  
**Compatibility**: CI CLI v1.0+, Python 3.7+, Node.js 14+