#!/usr/bin/env python3
"""
Log Aggregation and Analysis System
BRAIN Multi-Agent Development Coordination

Provides comprehensive logging, analysis, and reporting for all coordination activities.
"""

import json
import argparse
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
import logging
from collections import defaultdict, Counter
import re

# Configuration
BRAIN_ROOT = Path(__file__).parent.parent
DEVELOPMENT_LOG_FILE = BRAIN_ROOT / "DEVELOPMENT_LOG.jsonl"
REPORTS_DIR = BRAIN_ROOT / "coordination" / "reports"

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class LogAggregator:
    """Aggregates and analyzes development coordination logs"""
    
    def __init__(self):
        self.log_file = DEVELOPMENT_LOG_FILE
        self.reports_dir = REPORTS_DIR
        self._ensure_directories()
        
    def _ensure_directories(self):
        """Ensure required directories exist"""
        self.reports_dir.mkdir(exist_ok=True)
        
        if not self.log_file.exists():
            self.log_file.touch()
    
    def _parse_log_entries(self, start_time: Optional[datetime] = None, 
                          end_time: Optional[datetime] = None,
                          categories: Optional[List[str]] = None,
                          actions: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        """Parse log entries with optional filtering"""
        entries = []
        
        try:
            with open(self.log_file, 'r') as f:
                for line_num, line in enumerate(f, 1):
                    line = line.strip()
                    if not line:
                        continue
                    
                    try:
                        entry = json.loads(line)
                        
                        # Parse timestamp
                        if 'timestamp' in entry:
                            entry_time = datetime.fromisoformat(entry['timestamp'].replace('Z', '+00:00'))
                            entry['_parsed_timestamp'] = entry_time
                            
                            # Apply time filters
                            if start_time and entry_time < start_time:
                                continue
                            if end_time and entry_time > end_time:
                                continue
                        
                        # Apply category filter
                        if categories and entry.get('category') not in categories:
                            continue
                            
                        # Apply action filter
                        if actions and entry.get('action') not in actions:
                            continue
                        
                        entry['_line_number'] = line_num
                        entries.append(entry)
                        
                    except json.JSONDecodeError as e:
                        logger.warning(f"Invalid JSON on line {line_num}: {e}")
                        continue
                        
        except FileNotFoundError:
            logger.warning(f"Log file {self.log_file} not found")
        
        return entries
    
    def get_activity_summary(self, hours_back: int = 24) -> Dict[str, Any]:
        """Get activity summary for the specified time period"""
        end_time = datetime.now()
        start_time = end_time - timedelta(hours=hours_back)
        
        entries = self._parse_log_entries(start_time=start_time, end_time=end_time)
        
        # Aggregate by category and action
        category_counts = Counter()
        action_counts = Counter()
        hourly_activity = defaultdict(int)
        session_activity = defaultdict(int)
        agent_activity = defaultdict(int)
        lane_activity = defaultdict(int)
        
        for entry in entries:
            category = entry.get('category', 'unknown')
            action = entry.get('action', 'unknown')
            timestamp = entry.get('_parsed_timestamp')
            
            category_counts[category] += 1
            action_counts[action] += 1
            
            if timestamp:
                hour_key = timestamp.strftime('%Y-%m-%d %H:00')
                hourly_activity[hour_key] += 1
            
            # Session-specific aggregation
            if 'session' in entry:
                session_activity[entry['session']] += 1
            
            # Agent-specific aggregation
            if 'agent' in entry:
                agent_activity[entry['agent']] += 1
            
            # Lane-specific aggregation  
            if 'lane' in entry:
                lane_activity[entry['lane']] += 1
        
        return {
            "time_period": {
                "start": start_time.isoformat(),
                "end": end_time.isoformat(),
                "hours": hours_back
            },
            "total_entries": len(entries),
            "categories": dict(category_counts.most_common()),
            "actions": dict(action_counts.most_common()),
            "hourly_activity": dict(sorted(hourly_activity.items())),
            "session_activity": dict(session_activity.most_common(10)),
            "agent_activity": dict(agent_activity.most_common(10)),
            "lane_activity": dict(lane_activity.most_common()),
            "generated_at": datetime.now().isoformat()
        }
    
    def get_error_analysis(self, hours_back: int = 24) -> Dict[str, Any]:
        """Analyze errors and issues from logs"""
        end_time = datetime.now()
        start_time = end_time - timedelta(hours=hours_back)
        
        entries = self._parse_log_entries(start_time=start_time, end_time=end_time)
        
        errors = []
        warnings = []
        failures = []
        timeouts = []
        
        error_patterns = [
            r'error|failed|failure|exception',
            r'timeout|expired|stale',
            r'not found|missing|invalid',
            r'denied|rejected|blocked'
        ]
        
        for entry in entries:
            action = entry.get('action', '').lower()
            category = entry.get('category', '').lower()
            
            # Look for explicit error indicators
            if 'error' in action or 'failed' in action or 'failure' in action:
                errors.append(entry)
            elif 'timeout' in action or 'expired' in action:
                timeouts.append(entry)
            elif 'warning' in action or entry.get('level') == 'WARNING':
                warnings.append(entry)
            
            # Pattern-based detection
            entry_text = json.dumps(entry).lower()
            for pattern in error_patterns:
                if re.search(pattern, entry_text):
                    if entry not in errors and entry not in warnings and entry not in timeouts:
                        failures.append(entry)
                    break
        
        # Analyze error patterns
        error_categories = Counter()
        error_agents = Counter()
        error_sessions = Counter()
        
        all_issues = errors + warnings + failures + timeouts
        for issue in all_issues:
            error_categories[issue.get('category', 'unknown')] += 1
            if 'agent' in issue:
                error_agents[issue['agent']] += 1
            if 'session' in issue:
                error_sessions[issue['session']] += 1
        
        return {
            "time_period": {
                "start": start_time.isoformat(),
                "end": end_time.isoformat(),
                "hours": hours_back
            },
            "summary": {
                "total_issues": len(all_issues),
                "errors": len(errors),
                "warnings": len(warnings),
                "failures": len(failures),
                "timeouts": len(timeouts)
            },
            "error_categories": dict(error_categories.most_common()),
            "affected_agents": dict(error_agents.most_common()),
            "affected_sessions": dict(error_sessions.most_common()),
            "recent_errors": [
                {
                    "timestamp": e.get('timestamp'),
                    "category": e.get('category'),
                    "action": e.get('action'),
                    "session": e.get('session'),
                    "agent": e.get('agent'),
                    "details": {k: v for k, v in e.items() if k not in ['timestamp', 'category', 'action', '_parsed_timestamp', '_line_number']}
                }
                for e in sorted(all_issues, key=lambda x: x.get('_parsed_timestamp', datetime.min), reverse=True)[:10]
            ],
            "generated_at": datetime.now().isoformat()
        }
    
    def get_performance_metrics(self, hours_back: int = 24) -> Dict[str, Any]:
        """Analyze performance metrics from logs"""
        end_time = datetime.now()
        start_time = end_time - timedelta(hours=hours_back)
        
        entries = self._parse_log_entries(start_time=start_time, end_time=end_time)
        
        # Task performance analysis
        task_reservations = []
        task_completions = []
        task_releases = []
        session_heartbeats = []
        
        task_durations = []
        session_durations = {}
        
        for entry in entries:
            action = entry.get('action', '')
            timestamp = entry.get('_parsed_timestamp')
            
            if action == 'task_reserved':
                task_reservations.append(entry)
            elif action == 'task_completed':
                task_completions.append(entry)
                
                # Try to calculate task duration
                task_id = entry.get('task')
                if task_id:
                    # Find corresponding reservation
                    for res_entry in reversed(task_reservations):
                        if res_entry.get('task') == task_id:
                            if timestamp and res_entry.get('_parsed_timestamp'):
                                duration = (timestamp - res_entry['_parsed_timestamp']).total_seconds()
                                task_durations.append({
                                    'task': task_id,
                                    'duration_seconds': duration,
                                    'agent': entry.get('agent'),
                                    'session': entry.get('session')
                                })
                            break
                            
            elif action == 'task_released':
                task_releases.append(entry)
            elif action == 'heartbeat':
                session_heartbeats.append(entry)
                
                # Track session duration
                session_id = entry.get('session')
                if session_id and timestamp:
                    if session_id not in session_durations:
                        session_durations[session_id] = {
                            'start': timestamp,
                            'last_heartbeat': timestamp,
                            'heartbeat_count': 1
                        }
                    else:
                        session_durations[session_id]['last_heartbeat'] = timestamp
                        session_durations[session_id]['heartbeat_count'] += 1
        
        # Calculate metrics
        avg_task_duration = 0
        if task_durations:
            total_duration = sum(t['duration_seconds'] for t in task_durations)
            avg_task_duration = total_duration / len(task_durations)
        
        task_success_rate = 0
        if task_reservations:
            task_success_rate = len(task_completions) / len(task_reservations)
        
        # Session health metrics
        active_sessions = len([s for s in session_durations.values() 
                             if (end_time - s['last_heartbeat']).total_seconds() < 900])  # Active within 15 min
        
        return {
            "time_period": {
                "start": start_time.isoformat(),
                "end": end_time.isoformat(),
                "hours": hours_back
            },
            "task_metrics": {
                "total_reservations": len(task_reservations),
                "total_completions": len(task_completions),
                "total_releases": len(task_releases),
                "success_rate": round(task_success_rate, 3),
                "average_duration_seconds": round(avg_task_duration, 2),
                "average_duration_minutes": round(avg_task_duration / 60, 2)
            },
            "session_metrics": {
                "total_sessions": len(session_durations),
                "active_sessions": active_sessions,
                "total_heartbeats": len(session_heartbeats),
                "average_heartbeats_per_session": round(len(session_heartbeats) / max(len(session_durations), 1), 2)
            },
            "task_durations": sorted(task_durations, key=lambda x: x['duration_seconds'], reverse=True)[:10],
            "generated_at": datetime.now().isoformat()
        }
    
    def get_agent_analysis(self, hours_back: int = 24) -> Dict[str, Any]:
        """Analyze agent activity and performance"""
        end_time = datetime.now()
        start_time = end_time - timedelta(hours=hours_back)
        
        entries = self._parse_log_entries(start_time=start_time, end_time=end_time)
        
        agent_metrics = defaultdict(lambda: {
            'total_activity': 0,
            'task_reservations': 0,
            'task_completions': 0,
            'task_releases': 0,
            'errors': 0,
            'first_seen': None,
            'last_seen': None,
            'sessions': set(),
            'lanes': set(),
            'actions': Counter()
        })
        
        for entry in entries:
            agent = entry.get('agent')
            if not agent:
                continue
            
            timestamp = entry.get('_parsed_timestamp')
            action = entry.get('action', '')
            
            metrics = agent_metrics[agent]
            metrics['total_activity'] += 1
            metrics['actions'][action] += 1
            
            if timestamp:
                if not metrics['first_seen'] or timestamp < metrics['first_seen']:
                    metrics['first_seen'] = timestamp
                if not metrics['last_seen'] or timestamp > metrics['last_seen']:
                    metrics['last_seen'] = timestamp
            
            if action == 'task_reserved':
                metrics['task_reservations'] += 1
            elif action == 'task_completed':
                metrics['task_completions'] += 1
            elif action == 'task_released':
                metrics['task_releases'] += 1
            elif 'error' in action or 'failed' in action:
                metrics['errors'] += 1
            
            if 'session' in entry:
                metrics['sessions'].add(entry['session'])
            if 'lane' in entry:
                metrics['lanes'].add(entry['lane'])
        
        # Convert to serializable format and calculate derived metrics
        agent_summary = {}
        for agent, metrics in agent_metrics.items():
            success_rate = 0
            if metrics['task_reservations'] > 0:
                success_rate = metrics['task_completions'] / metrics['task_reservations']
            
            agent_summary[agent] = {
                'total_activity': metrics['total_activity'],
                'task_reservations': metrics['task_reservations'],
                'task_completions': metrics['task_completions'],
                'task_releases': metrics['task_releases'],
                'task_success_rate': round(success_rate, 3),
                'errors': metrics['errors'],
                'sessions_count': len(metrics['sessions']),
                'lanes_count': len(metrics['lanes']),
                'first_seen': metrics['first_seen'].isoformat() if metrics['first_seen'] else None,
                'last_seen': metrics['last_seen'].isoformat() if metrics['last_seen'] else None,
                'top_actions': dict(metrics['actions'].most_common(5)),
                'sessions': list(metrics['sessions'])[:5],  # Limit for readability
                'lanes': list(metrics['lanes'])
            }
        
        # Sort by activity level
        sorted_agents = sorted(agent_summary.items(), key=lambda x: x[1]['total_activity'], reverse=True)
        
        return {
            "time_period": {
                "start": start_time.isoformat(),
                "end": end_time.isoformat(),
                "hours": hours_back
            },
            "summary": {
                "total_agents": len(agent_summary),
                "most_active_agent": sorted_agents[0][0] if sorted_agents else None,
                "total_agent_activities": sum(a['total_activity'] for a in agent_summary.values())
            },
            "agents": dict(sorted_agents),
            "generated_at": datetime.now().isoformat()
        }
    
    def generate_daily_report(self, date: Optional[str] = None) -> str:
        """Generate comprehensive daily activity report"""
        if date:
            target_date = datetime.fromisoformat(date)
        else:
            target_date = datetime.now().date()
            
        start_time = datetime.combine(target_date, datetime.min.time())
        end_time = start_time + timedelta(days=1)
        
        # Get all analyses for the day
        activity_summary = self.get_activity_summary(24)
        error_analysis = self.get_error_analysis(24)
        performance_metrics = self.get_performance_metrics(24)
        agent_analysis = self.get_agent_analysis(24)
        
        # Generate report
        report_lines = []
        report_lines.append(f"# BRAIN Development Daily Report")
        report_lines.append(f"**Date: {target_date}**")
        report_lines.append(f"**Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}**")
        report_lines.append("")
        
        # Executive Summary
        report_lines.append("## 📊 Executive Summary")
        report_lines.append("")
        report_lines.append(f"- **Total Activity**: {activity_summary['total_entries']} log entries")
        report_lines.append(f"- **Task Success Rate**: {performance_metrics['task_metrics']['success_rate']:.1%}")
        report_lines.append(f"- **Average Task Duration**: {performance_metrics['task_metrics']['average_duration_minutes']:.1f} minutes")
        report_lines.append(f"- **Active Sessions**: {performance_metrics['session_metrics']['active_sessions']}")
        report_lines.append(f"- **Total Issues**: {error_analysis['summary']['total_issues']}")
        report_lines.append("")
        
        # Activity Breakdown
        report_lines.append("## 🔄 Activity Breakdown")
        report_lines.append("")
        report_lines.append("### By Category")
        for category, count in activity_summary['categories'].items():
            report_lines.append(f"- **{category}**: {count} activities")
        report_lines.append("")
        
        report_lines.append("### By Lane")
        if activity_summary['lane_activity']:
            for lane, count in activity_summary['lane_activity'].items():
                report_lines.append(f"- **{lane}**: {count} activities")
        else:
            report_lines.append("- No lane-specific activity recorded")
        report_lines.append("")
        
        # Task Performance
        report_lines.append("## ⚡ Task Performance")
        report_lines.append("")
        task_metrics = performance_metrics['task_metrics']
        report_lines.append(f"- **Reservations**: {task_metrics['total_reservations']}")
        report_lines.append(f"- **Completions**: {task_metrics['total_completions']}")
        report_lines.append(f"- **Releases**: {task_metrics['total_releases']}")
        report_lines.append(f"- **Success Rate**: {task_metrics['success_rate']:.1%}")
        report_lines.append(f"- **Average Duration**: {task_metrics['average_duration_minutes']:.1f} minutes")
        report_lines.append("")
        
        # Agent Activity
        report_lines.append("## 👥 Agent Activity")
        report_lines.append("")
        agent_data = agent_analysis['agents']
        if agent_data:
            report_lines.append("### Top Agents by Activity")
            for agent, metrics in list(agent_data.items())[:5]:
                report_lines.append(f"- **{agent}**: {metrics['total_activity']} activities, "
                                  f"{metrics['task_completions']} tasks completed "
                                  f"({metrics['task_success_rate']:.1%} success rate)")
        else:
            report_lines.append("- No agent activity recorded")
        report_lines.append("")
        
        # Issues and Errors
        if error_analysis['summary']['total_issues'] > 0:
            report_lines.append("## ⚠️ Issues and Errors")
            report_lines.append("")
            summary = error_analysis['summary']
            report_lines.append(f"- **Total Issues**: {summary['total_issues']}")
            report_lines.append(f"- **Errors**: {summary['errors']}")
            report_lines.append(f"- **Warnings**: {summary['warnings']}")
            report_lines.append(f"- **Timeouts**: {summary['timeouts']}")
            
            if error_analysis['recent_errors']:
                report_lines.append("")
                report_lines.append("### Recent Issues")
                for error in error_analysis['recent_errors'][:3]:
                    report_lines.append(f"- **{error['timestamp']}**: {error['action']} "
                                      f"(Category: {error['category']})")
                    if error.get('session'):
                        report_lines.append(f"  - Session: {error['session']}")
                    if error.get('agent'):
                        report_lines.append(f"  - Agent: {error['agent']}")
            report_lines.append("")
        
        # Session Health
        report_lines.append("## 💓 Session Health")
        report_lines.append("")
        session_metrics = performance_metrics['session_metrics']
        report_lines.append(f"- **Total Sessions**: {session_metrics['total_sessions']}")
        report_lines.append(f"- **Currently Active**: {session_metrics['active_sessions']}")
        report_lines.append(f"- **Total Heartbeats**: {session_metrics['total_heartbeats']}")
        report_lines.append(f"- **Avg Heartbeats/Session**: {session_metrics['average_heartbeats_per_session']:.1f}")
        report_lines.append("")
        
        # Recommendations
        report_lines.append("## 💡 Recommendations")
        report_lines.append("")
        
        # Generate smart recommendations based on data
        recommendations = []
        
        if task_metrics['success_rate'] < 0.8:
            recommendations.append("Consider investigating task failure causes - success rate below 80%")
        
        if task_metrics['average_duration_minutes'] > 60:
            recommendations.append("Task durations are lengthy - consider breaking down complex tasks")
        
        if error_analysis['summary']['total_issues'] > 10:
            recommendations.append("High error count detected - review error patterns and implement fixes")
        
        if session_metrics['active_sessions'] < 2:
            recommendations.append("Low session activity - consider increasing development parallelization")
        
        if not recommendations:
            recommendations.append("System operating within normal parameters - maintain current practices")
        
        for rec in recommendations:
            report_lines.append(f"- {rec}")
        
        report_lines.append("")
        report_lines.append("---")
        report_lines.append("*Report generated by BRAIN Log Aggregator*")
        
        return "\n".join(report_lines)
    
    def save_report(self, report_content: str, filename: str) -> Path:
        """Save report to file"""
        report_path = self.reports_dir / filename
        
        with open(report_path, 'w') as f:
            f.write(report_content)
        
        logger.info(f"Saved report to {report_path}")
        return report_path
    
    def search_logs(self, query: str, hours_back: int = 24, max_results: int = 50) -> List[Dict[str, Any]]:
        """Search log entries for specific patterns"""
        end_time = datetime.now()
        start_time = end_time - timedelta(hours=hours_back)
        
        entries = self._parse_log_entries(start_time=start_time, end_time=end_time)
        
        results = []
        query_lower = query.lower()
        
        for entry in entries:
            entry_text = json.dumps(entry).lower()
            if query_lower in entry_text:
                results.append(entry)
                if len(results) >= max_results:
                    break
        
        return results

def main():
    """Command line interface for log aggregation"""
    parser = argparse.ArgumentParser(description="BRAIN Log Aggregation System")
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Activity summary
    activity_parser = subparsers.add_parser('activity', help='Get activity summary')
    activity_parser.add_argument('--hours', type=int, default=24, help='Hours back to analyze')
    activity_parser.add_argument('--format', choices=['json', 'text'], default='json', help='Output format')
    
    # Error analysis
    error_parser = subparsers.add_parser('errors', help='Analyze errors and issues')
    error_parser.add_argument('--hours', type=int, default=24, help='Hours back to analyze')
    error_parser.add_argument('--format', choices=['json', 'text'], default='json', help='Output format')
    
    # Performance metrics
    perf_parser = subparsers.add_parser('performance', help='Get performance metrics')
    perf_parser.add_argument('--hours', type=int, default=24, help='Hours back to analyze')
    perf_parser.add_argument('--format', choices=['json', 'text'], default='json', help='Output format')
    
    # Agent analysis
    agent_parser = subparsers.add_parser('agents', help='Analyze agent activity')
    agent_parser.add_argument('--hours', type=int, default=24, help='Hours back to analyze')
    agent_parser.add_argument('--format', choices=['json', 'text'], default='json', help='Output format')
    
    # Daily report
    report_parser = subparsers.add_parser('daily-report', help='Generate daily report')
    report_parser.add_argument('--date', help='Date (YYYY-MM-DD) - defaults to today')
    report_parser.add_argument('--save', action='store_true', help='Save report to file')
    
    # Search logs
    search_parser = subparsers.add_parser('search', help='Search log entries')
    search_parser.add_argument('query', help='Search query')
    search_parser.add_argument('--hours', type=int, default=24, help='Hours back to search')
    search_parser.add_argument('--max-results', type=int, default=50, help='Maximum results')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    aggregator = LogAggregator()
    
    try:
        if args.command == 'activity':
            summary = aggregator.get_activity_summary(args.hours)
            if args.format == 'json':
                print(json.dumps(summary, indent=2))
            else:
                print(f"Activity Summary ({args.hours} hours)")
                print(f"Total Entries: {summary['total_entries']}")
                print(f"Categories: {summary['categories']}")
                print(f"Top Actions: {dict(list(summary['actions'].items())[:5])}")
        
        elif args.command == 'errors':
            analysis = aggregator.get_error_analysis(args.hours)
            if args.format == 'json':
                print(json.dumps(analysis, indent=2))
            else:
                print(f"Error Analysis ({args.hours} hours)")
                print(f"Total Issues: {analysis['summary']['total_issues']}")
                print(f"Breakdown: {analysis['summary']}")
                if analysis['recent_errors']:
                    print("Recent Errors:")
                    for error in analysis['recent_errors'][:3]:
                        print(f"  - {error['timestamp']}: {error['action']}")
        
        elif args.command == 'performance':
            metrics = aggregator.get_performance_metrics(args.hours)
            if args.format == 'json':
                print(json.dumps(metrics, indent=2))
            else:
                print(f"Performance Metrics ({args.hours} hours)")
                task_metrics = metrics['task_metrics']
                print(f"Tasks: {task_metrics['total_completions']}/{task_metrics['total_reservations']} "
                      f"({task_metrics['success_rate']:.1%} success)")
                print(f"Avg Duration: {task_metrics['average_duration_minutes']:.1f} minutes")
                session_metrics = metrics['session_metrics']
                print(f"Sessions: {session_metrics['active_sessions']}/{session_metrics['total_sessions']} active")
        
        elif args.command == 'agents':
            analysis = aggregator.get_agent_analysis(args.hours)
            if args.format == 'json':
                print(json.dumps(analysis, indent=2))
            else:
                print(f"Agent Analysis ({args.hours} hours)")
                print(f"Total Agents: {analysis['summary']['total_agents']}")
                if analysis['summary']['most_active_agent']:
                    print(f"Most Active: {analysis['summary']['most_active_agent']}")
                print("Top Agents:")
                for agent, metrics in list(analysis['agents'].items())[:5]:
                    print(f"  {agent}: {metrics['total_activity']} activities, "
                          f"{metrics['task_completions']} completed")
        
        elif args.command == 'daily-report':
            report = aggregator.generate_daily_report(args.date)
            
            if args.save:
                date_str = args.date or datetime.now().strftime('%Y-%m-%d')
                filename = f"daily_report_{date_str}.md"
                report_path = aggregator.save_report(report, filename)
                print(f"✓ Saved daily report to {report_path}")
            else:
                print(report)
        
        elif args.command == 'search':
            results = aggregator.search_logs(args.query, args.hours, args.max_results)
            
            print(f"Found {len(results)} matching entries:")
            for i, entry in enumerate(results, 1):
                print(f"\n{i}. {entry.get('timestamp')} - {entry.get('category')}/{entry.get('action')}")
                if 'session' in entry:
                    print(f"   Session: {entry['session']}")
                if 'agent' in entry:
                    print(f"   Agent: {entry['agent']}")
                if 'task' in entry:
                    print(f"   Task: {entry['task']}")
    
    except Exception as e:
        logger.error(f"Command failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()