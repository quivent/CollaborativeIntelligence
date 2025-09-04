# Dashboard System Guide

## Overview

The CI Dashboard System provides real-time monitoring and visualization of multi-agent coordination activities. It offers both terminal-based and web-based interfaces for comprehensive session management, system health monitoring, and analytics visualization.

## Dashboard Interfaces

### Terminal Dashboard

The terminal dashboard provides a console-based real-time view of coordination system status.

#### Basic Usage

```bash
# Launch terminal dashboard with default settings
ci dashboard

# Custom refresh interval and view
ci dashboard --refresh 10 --view sessions

# Available views: system, sessions, tasks, lanes
ci dashboard --view lanes --refresh 3
```

#### Dashboard Views

##### System View (Default)
- System health indicators
- Session statistics with visual progress bars
- Lane assignment overview
- Performance metrics and coordination stats

```bash
ci dashboard --view system
```

**Display includes:**
- 🏥 SYSTEM HEALTH: Status, active sessions, utilization bar, timeout rate
- 📊 SESSION STATISTICS: Active, ended, timed out sessions with color coding
- 🛤️ LANE ASSIGNMENTS: Visual bars showing session distribution
- ⚡ PERFORMANCE METRICS: Total registered, timeouts, heartbeat intervals

##### Sessions View
- Detailed session table with real-time updates
- Session status with color-coded indicators
- Duration, task progress, and heartbeat information

```bash
ci dashboard --view sessions
```

**Display includes:**
- Session ID (truncated for readability)
- Status with color coding (active=green, ended=blue, timed_out=red)
- Lane assignments
- Duration in minutes
- Task completion ratios
- Heartbeat counts

##### Tasks View
- Task coordination overview
- Most active sessions by task count
- Task distribution by lane

```bash
ci dashboard --view tasks
```

**Display includes:**
- Total completed and reserved tasks
- Active sessions processing tasks
- Task productivity metrics
- Most active sessions ranked by task activity
- Task breakdown by lane

##### Lanes View
- Lane utilization visualization
- Session distribution across lanes
- Lane activity metrics

```bash
ci dashboard --view lanes
```

**Display includes:**
- Visual progress bars for lane utilization
- Active session counts per lane
- Task completion statistics by lane
- Lane load balancing information

#### Terminal Dashboard Controls

```bash
# Navigation between views during runtime
# Press Ctrl+C to exit dashboard

# View modes and refresh rates
--refresh, -r <SECONDS>      # Refresh interval (default: 5 seconds)
--view, -v <VIEW>           # Initial view mode
```

### Web Dashboard

The web dashboard provides a modern, interactive interface with charts and real-time updates.

#### Launch Web Dashboard

```bash
# Start web dashboard on default port (8080)
ci dashboard --web

# Custom port
ci dashboard --web --port 3000

# The dashboard will be available at:
# http://localhost:8080 (or your specified port)
```

#### Web Dashboard Features

##### Real-time Updates
- Automatic refresh every 5 seconds
- Live connection status indicator
- Dynamic charts and metrics

##### Interactive Views
- Tabbed navigation (System, Sessions, Tasks, Lanes)
- Responsive design for mobile and desktop
- Dark theme optimized for development environments

##### API Endpoints
The web dashboard exposes REST API endpoints:

```bash
# System status
GET http://localhost:8080/api/system

# All sessions
GET http://localhost:8080/api/sessions

# Task metrics
GET http://localhost:8080/api/tasks

# Lane assignments
GET http://localhost:8080/api/lanes
```

##### Visual Components

**System Health Cards**
- Health status with color-coded indicators
- Utilization progress bars
- Key performance metrics

**Session Tables**
- Sortable and filterable session data
- Status badges with color coding
- Duration and activity metrics

**Task Activity Charts**
- Line charts showing task completion over time
- Activity trends and patterns
- Session productivity metrics

**Lane Visualization**
- Horizontal bar charts for lane utilization
- Load distribution analysis
- Capacity planning insights

## Data Export

### Dashboard Data Export

Export current dashboard data to files:

```bash
# Export system status
ci dashboard --export system-status.json --view system

# Export session data
ci dashboard --export sessions.json --view sessions

# Export task metrics
ci dashboard --export tasks.json --view tasks

# Export lane data
ci dashboard --export lanes.json --view lanes
```

### Export Formats

#### System Status Export
```json
{
  "system_health": {
    "status": "healthy",
    "active_sessions": 3,
    "max_concurrent": 10,
    "utilization_percent": 30,
    "timeout_rate": 0.05
  },
  "session_counts": {
    "active": 3,
    "ended": 12,
    "timed_out": 1
  },
  "lane_assignments": {
    "development": 2,
    "testing": 1
  },
  "coordination": {
    "active_sessions": 3,
    "total_sessions_registered": 16,
    "session_timeouts": 1,
    "heartbeat_interval_seconds": 300
  }
}
```

#### Sessions Export
```json
[
  {
    "id": "claude-hostname-1693234567-abc12345",
    "status": "active",
    "lane": "development",
    "registered_at": "2025-09-01T10:00:00Z",
    "last_heartbeat": "2025-09-01T10:25:00Z",
    "hostname": "dev-machine",
    "tasks_completed": 5,
    "tasks_reserved": 2,
    "heartbeat_count": 6,
    "duration_seconds": 1500
  }
]
```

## Monitoring and Alerts

### Health Status Indicators

The dashboard uses color-coded health indicators:

- 🟢 **Healthy**: System operating within normal parameters
- 🟡 **Warning**: Approaching capacity limits or elevated timeout rates
- 🔴 **Overloaded**: Maximum capacity reached or high failure rates

### Key Metrics

#### System Health
- **Active Sessions**: Current number of active coordination sessions
- **Utilization**: Percentage of maximum concurrent sessions in use
- **Timeout Rate**: Percentage of sessions that have timed out
- **Heartbeat Interval**: Expected time between session heartbeats

#### Session Metrics
- **Duration**: Time since session registration
- **Task Activity**: Completed vs reserved task counts
- **Heartbeat Count**: Number of heartbeats sent by session
- **Last Activity**: Time since last session update

#### Lane Metrics
- **Distribution**: Number of sessions assigned to each lane
- **Activity**: Task completion rates by lane
- **Load Balance**: Even distribution of work across lanes

### Performance Indicators

#### Dashboard Performance
- Connection status (connected/error)
- Last update timestamp
- Refresh interval timing
- Data loading states

#### System Performance
- Session registration rate
- Cleanup efficiency
- Lock contention (minimal expected)
- File I/O performance

## Configuration

### Dashboard Configuration

```bash
# Environment variables for dashboard customization
export CI_DASHBOARD_REFRESH_RATE=5          # Terminal refresh rate
export CI_DASHBOARD_WEB_PORT=8080           # Web dashboard port
export CI_DASHBOARD_THEME=dark              # Theme selection
export CI_DASHBOARD_MAX_SESSIONS_DISPLAY=20 # Max sessions in table view
```

### Web Dashboard Configuration

The web dashboard supports runtime configuration through URL parameters:

```
http://localhost:8080?refresh=10&theme=light&view=sessions
```

### Custom Styling

Web dashboard CSS can be customized by modifying the generated CSS files in the temporary dashboard directory.

## Integration Examples

### Automated Monitoring

```bash
#!/bin/bash
# monitoring-script.sh

# Check system health
HEALTH=$(ci dashboard --export /tmp/health.json --view system && jq -r '.system_health.status' /tmp/health.json)

if [ "$HEALTH" != "healthy" ]; then
    echo "ALERT: System health is $HEALTH"
    # Send notification to team
    ci coordination reports summary --format markdown | mail -s "System Alert" team@company.com
fi
```

### Performance Tracking

```bash
#!/bin/bash
# performance-tracker.sh

while true; do
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    UTILIZATION=$(ci dashboard --export /tmp/metrics.json --view system && jq -r '.system_health.utilization_percent' /tmp/metrics.json)
    
    echo "$TIMESTAMP,$UTILIZATION" >> performance-log.csv
    sleep 300  # Track every 5 minutes
done
```

### Team Dashboard Setup

```bash
#!/bin/bash
# team-dashboard-setup.sh

# Start web dashboard for team monitoring
ci dashboard --web --port 8080 &
WEB_PID=$!

echo "Team dashboard available at http://$(hostname):8080"
echo "Dashboard PID: $WEB_PID"

# Keep dashboard running and restart if needed
while true; do
    if ! kill -0 $WEB_PID 2>/dev/null; then
        echo "Dashboard crashed, restarting..."
        ci dashboard --web --port 8080 &
        WEB_PID=$!
    fi
    sleep 30
done
```

## Troubleshooting

### Common Dashboard Issues

#### Terminal Dashboard Issues

1. **Dashboard Not Refreshing**
   ```bash
   # Check if coordination system is responsive
   ci coordination status --system
   
   # Restart with different refresh rate
   ci dashboard --refresh 1
   ```

2. **Display Formatting Issues**
   ```bash
   # Check terminal size
   echo "Terminal size: $(tput cols)x$(tput lines)"
   
   # Resize terminal if needed (minimum 80x25 recommended)
   ```

3. **Permission Errors**
   ```bash
   # Check BRAIN directory permissions
   ls -la /path/to/CollaborativeIntelligence/BRAIN/
   
   # Fix permissions if needed
   chmod 755 /path/to/CollaborativeIntelligence/BRAIN/
   ```

#### Web Dashboard Issues

1. **Port Already in Use**
   ```bash
   # Check what's using the port
   netstat -tulpn | grep :8080
   
   # Use different port
   ci dashboard --web --port 8081
   ```

2. **Dashboard Not Loading**
   ```bash
   # Check if server started successfully
   ps aux | grep dashboard
   
   # Check server logs
   tail -f /tmp/dashboard-server.log
   ```

3. **API Endpoints Not Working**
   ```bash
   # Test API directly
   curl http://localhost:8080/api/system
   
   # Check coordination script
   python3 /path/to/BRAIN/coordination/session_coordinator.py status --system
   ```

### Performance Optimization

#### Terminal Dashboard
- Use longer refresh intervals for large numbers of sessions
- Filter views to specific lanes or statuses when needed
- Export data for offline analysis instead of continuous monitoring

#### Web Dashboard
- Close unused browser tabs to reduce resource usage
- Use appropriate refresh intervals based on activity level
- Monitor browser memory usage for long-running sessions

## Best Practices

### Dashboard Usage
1. **Choose Appropriate Views**: Use system view for overview, sessions for detailed monitoring
2. **Set Reasonable Refresh Rates**: Balance real-time needs with system performance
3. **Use Export Features**: Save snapshots for analysis and reporting
4. **Monitor Resource Usage**: Dashboard should not impact system performance

### Team Coordination
1. **Shared Dashboard Access**: Use web dashboard for team visibility
2. **Role-Based Views**: Different team members may need different dashboard views
3. **Integration with Tools**: Export data to existing monitoring and alerting systems
4. **Documentation**: Keep dashboard usage patterns documented for team onboarding

### System Administration
1. **Regular Health Checks**: Monitor system health indicators daily
2. **Capacity Planning**: Use utilization metrics to plan for growth
3. **Performance Baselines**: Establish normal operating ranges for key metrics
4. **Automated Alerting**: Set up scripts to monitor critical thresholds

---

**Last Updated**: 2025-09-01  
**Version**: 1.0  
**Compatibility**: CI CLI v1.0+, Web browsers with JavaScript enabled