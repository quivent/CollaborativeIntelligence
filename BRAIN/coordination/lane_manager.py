#!/usr/bin/env python3
"""
Lane Management System
BRAIN Multi-Agent Development Coordination

Manages development lane assignments, progress tracking, and coordination.
"""

import json
import argparse
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any, Set
import fcntl
import logging
from collections import defaultdict

# Configuration
BRAIN_ROOT = Path(__file__).parent.parent
LANE_MANAGEMENT_FILE = BRAIN_ROOT / "LANE_MANAGEMENT.json"
TASK_REGISTRY_FILE = BRAIN_ROOT / "TASK_REGISTRY.json"
DEVELOPMENT_LOG_FILE = BRAIN_ROOT / "DEVELOPMENT_LOG.jsonl"
LOCK_FILE = BRAIN_ROOT / "coordination" / ".lane_lock"

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class LaneManager:
    """Manages development lanes, agent assignments, and progress coordination"""
    
    def __init__(self):
        self.lane_file = LANE_MANAGEMENT_FILE
        self.task_file = TASK_REGISTRY_FILE
        self.log_file = DEVELOPMENT_LOG_FILE
        self.lock_file = LOCK_FILE
        self._ensure_files_exist()
        
    def _ensure_files_exist(self):
        """Ensure required files exist with proper structure"""
        if not self.lane_file.exists():
            self._initialize_lane_management()
            
        if not self.log_file.exists():
            self.log_file.touch()
            
        if not self.lock_file.exists():
            self.lock_file.touch()
    
    def _initialize_lane_management(self):
        """Initialize lane management with BRAIN development structure"""
        initial_lanes = {
            "version": "1.0",
            "last_updated": datetime.now().isoformat(),
            "lanes": {
                "Lane1": {
                    "id": "Lane1",
                    "name": "Foundation Infrastructure",
                    "description": "Core runtime engine, agent communication, database layer",
                    "phase": "Foundation",
                    "duration_months": 3,
                    "start_month": 1,
                    "end_month": 3,
                    "status": "active",
                    "priority": 1,
                    "lead_agents": ["Manager", "Architect", "Developer"],
                    "contributor_agents": ["Analyst", "Enforcer"],
                    "assigned_sessions": [],
                    "tasks": [
                        "runtime-engine-core",
                        "agent-communication-infrastructure", 
                        "database-persistence-layer"
                    ],
                    "dependencies": [],
                    "blockers": [],
                    "milestones": [
                        {
                            "name": "Runtime Engine Complete",
                            "target_date": "2025-09-30",
                            "status": "pending",
                            "criteria": [
                                "Agent activation <2 seconds",
                                "Task execution with coordination",
                                "Session persistence working",
                                "90% unit test coverage"
                            ]
                        },
                        {
                            "name": "Communication Infrastructure Complete", 
                            "target_date": "2025-10-31",
                            "status": "pending",
                            "criteria": [
                                "Inter-agent messaging <50ms",
                                "Event throughput >1000/sec",
                                "Multi-agent collaboration working",
                                "Zero message loss"
                            ]
                        },
                        {
                            "name": "Database Layer Complete",
                            "target_date": "2025-11-30", 
                            "status": "pending",
                            "criteria": [
                                "Database queries <10ms avg",
                                "Automated backups working",
                                "Migration system functional",
                                "10K+ concurrent connections"
                            ]
                        }
                    ],
                    "progress": {
                        "overall_percent": 0,
                        "tasks_completed": 0,
                        "tasks_total": 3,
                        "last_update": datetime.now().isoformat()
                    }
                },
                "Lane2": {
                    "id": "Lane2",
                    "name": "Intelligence Systems",
                    "description": "Knowledge processing, agent capabilities, learning systems",
                    "phase": "Intelligence",
                    "duration_months": 3,
                    "start_month": 4,
                    "end_month": 6,
                    "status": "planned",
                    "priority": 2,
                    "lead_agents": ["Analyst", "Planner", "Developer"],
                    "contributor_agents": ["Architect", "Manager"],
                    "assigned_sessions": [],
                    "tasks": [
                        "trinity-curation-system",
                        "advanced-agent-capabilities",
                        "continuous-learning-system"
                    ],
                    "dependencies": ["Lane1"],
                    "blockers": [],
                    "milestones": [
                        {
                            "name": "Trinity Curation Automated",
                            "target_date": "2026-01-31",
                            "status": "pending",
                            "criteria": [
                                "Automated Trinity evaluation",
                                "Knowledge integration pipeline",
                                "Quality validation framework",
                                "Processing throughput targets met"
                            ]
                        }
                    ],
                    "progress": {
                        "overall_percent": 0,
                        "tasks_completed": 0,
                        "tasks_total": 3,
                        "last_update": datetime.now().isoformat()
                    }
                },
                "Lane3": {
                    "id": "Lane3",
                    "name": "Production Readiness",
                    "description": "Security, monitoring, deployment, operations",
                    "phase": "Production",
                    "duration_months": 3,
                    "start_month": 7,
                    "end_month": 9,
                    "status": "planned",
                    "priority": 3,
                    "lead_agents": ["Enforcer", "BusinessOwner", "Topologist"],
                    "contributor_agents": ["Developer", "Analyst"],
                    "assigned_sessions": [],
                    "tasks": [
                        "security-access-control",
                        "api-gateway-web-interface",
                        "monitoring-observability-system"
                    ],
                    "dependencies": ["Lane1", "Lane2"],
                    "blockers": [],
                    "milestones": [
                        {
                            "name": "Security Framework Complete",
                            "target_date": "2026-04-30", 
                            "status": "pending",
                            "criteria": [
                                "Complete security framework",
                                "Compliance certification",
                                "Security testing validation",
                                "Penetration testing passed"
                            ]
                        }
                    ],
                    "progress": {
                        "overall_percent": 0,
                        "tasks_completed": 0,
                        "tasks_total": 3,
                        "last_update": datetime.now().isoformat()
                    }
                },
                "Lane4": {
                    "id": "Lane4",
                    "name": "Advanced Features",
                    "description": "Advanced analytics, optimization, scaling",
                    "phase": "Enhancement",
                    "duration_months": 3,
                    "start_month": 10,
                    "end_month": 12,
                    "status": "planned",
                    "priority": 4,
                    "lead_agents": ["Architect", "Analyst", "Manager"],
                    "contributor_agents": ["Developer", "BusinessOwner"],
                    "assigned_sessions": [],
                    "tasks": [
                        "advanced-analytics-ml",
                        "deployment-devops-automation",
                        "performance-optimization-scaling"
                    ],
                    "dependencies": ["Lane1", "Lane2", "Lane3"],
                    "blockers": [],
                    "milestones": [
                        {
                            "name": "Advanced Analytics Complete",
                            "target_date": "2026-07-31",
                            "status": "pending", 
                            "criteria": [
                                "Predictive analytics operational",
                                "ML integration complete",
                                "Advanced visualization available",
                                "Analytics APIs functional"
                            ]
                        }
                    ],
                    "progress": {
                        "overall_percent": 0,
                        "tasks_completed": 0,
                        "tasks_total": 3,
                        "last_update": datetime.now().isoformat()
                    }
                }
            },
            "coordination": {
                "active_lanes": ["Lane1"],
                "total_agents_assigned": 0,
                "total_sessions_active": 0,
                "cross_lane_dependencies": [
                    {"from": "Lane2", "to": "Lane1", "type": "sequential"},
                    {"from": "Lane3", "to": "Lane1", "type": "sequential"},
                    {"from": "Lane3", "to": "Lane2", "type": "sequential"},
                    {"from": "Lane4", "to": "Lane1", "type": "sequential"},
                    {"from": "Lane4", "to": "Lane2", "type": "sequential"},
                    {"from": "Lane4", "to": "Lane3", "type": "sequential"}
                ],
                "last_coordination_update": datetime.now().isoformat()
            },
            "metadata": {
                "total_lanes": 4,
                "active_lanes": 1,
                "completed_lanes": 0,
                "last_update": datetime.now().isoformat()
            }
        }
        
        with open(self.lane_file, 'w') as f:
            json.dump(initial_lanes, f, indent=2)
        
        logger.info(f"Initialized lane management with {len(initial_lanes['lanes'])} lanes")
    
    def _acquire_lock(self):
        """Acquire exclusive file lock"""
        try:
            self.lock_fd = open(self.lock_file, 'w')
            fcntl.flock(self.lock_fd.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            return True
        except (IOError, OSError) as e:
            logger.warning(f"Could not acquire lock: {e}")
            return False
    
    def _release_lock(self):
        """Release file lock"""
        try:
            if hasattr(self, 'lock_fd'):
                fcntl.flock(self.lock_fd.fileno(), fcntl.LOCK_UN)
                self.lock_fd.close()
        except (IOError, OSError) as e:
            logger.warning(f"Could not release lock: {e}")
    
    def _load_lanes(self) -> Dict[str, Any]:
        """Load lane management data with locking"""
        if not self._acquire_lock():
            raise Exception("Could not acquire lock on lane management")
        
        try:
            with open(self.lane_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, FileNotFoundError) as e:
            logger.error(f"Could not load lane management: {e}")
            raise
    
    def _save_lanes(self, lanes: Dict[str, Any]):
        """Save lane management data and release lock"""
        try:
            lanes["last_updated"] = datetime.now().isoformat()
            with open(self.lane_file, 'w') as f:
                json.dump(lanes, f, indent=2)
        finally:
            self._release_lock()
    
    def _load_tasks(self) -> Dict[str, Any]:
        """Load task registry for progress calculation"""
        try:
            with open(self.task_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, FileNotFoundError):
            return {"tasks": {}}
    
    def _log_activity(self, category: str, action: str, details: Dict[str, Any]):
        """Log activity to development log"""
        log_entry = {
            "timestamp": datetime.now().isoformat(),
            "category": category,
            "action": action,
            **details
        }
        
        with open(self.log_file, 'a') as f:
            f.write(json.dumps(log_entry) + '\n')
    
    def _update_lane_progress(self, lanes: Dict[str, Any], lane_id: str):
        """Update progress metrics for a lane based on task completion"""
        if lane_id not in lanes["lanes"]:
            return
        
        lane = lanes["lanes"][lane_id]
        task_registry = self._load_tasks()
        
        total_tasks = len(lane["tasks"])
        completed_tasks = 0
        in_progress_tasks = 0
        
        for task_id in lane["tasks"]:
            if task_id in task_registry.get("tasks", {}):
                task_status = task_registry["tasks"][task_id]["status"]
                if task_status == "completed":
                    completed_tasks += 1
                elif task_status in ["in_progress", "reserved"]:
                    in_progress_tasks += 1
        
        # Update progress
        if total_tasks > 0:
            overall_percent = int((completed_tasks / total_tasks) * 100)
        else:
            overall_percent = 0
        
        lane["progress"].update({
            "overall_percent": overall_percent,
            "tasks_completed": completed_tasks,
            "tasks_in_progress": in_progress_tasks,
            "tasks_total": total_tasks,
            "last_update": datetime.now().isoformat()
        })
        
        # Update lane status based on progress
        if overall_percent == 100:
            lane["status"] = "completed"
        elif overall_percent > 0 or in_progress_tasks > 0:
            lane["status"] = "active"
    
    def assign_agent_to_lane(self, lane_id: str, agent: str, role: str = "contributor") -> bool:
        """Assign an agent to a lane with a specific role"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                logger.error(f"Lane {lane_id} not found")
                return False
            
            lane = lanes["lanes"][lane_id]
            
            if role == "lead":
                if agent not in lane["lead_agents"]:
                    lane["lead_agents"].append(agent)
                # Remove from contributors if promoting to lead
                if agent in lane.get("contributor_agents", []):
                    lane["contributor_agents"].remove(agent)
            else:  # contributor role
                if agent not in lane.get("contributor_agents", []):
                    if "contributor_agents" not in lane:
                        lane["contributor_agents"] = []
                    lane["contributor_agents"].append(agent)
                # Don't remove from leads if demoting, keep both roles
            
            self._log_activity("lane_activity", "agent_assigned", {
                "lane": lane_id,
                "agent": agent,
                "role": role
            })
            
            logger.info(f"Assigned agent {agent} to lane {lane_id} as {role}")
            return True
            
        finally:
            self._save_lanes(lanes)
    
    def assign_session_to_lane(self, lane_id: str, session_id: str) -> bool:
        """Assign a session to work on a specific lane"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                logger.error(f"Lane {lane_id} not found")
                return False
            
            lane = lanes["lanes"][lane_id]
            
            if session_id not in lane.get("assigned_sessions", []):
                if "assigned_sessions" not in lane:
                    lane["assigned_sessions"] = []
                lane["assigned_sessions"].append(session_id)
            
            # Update coordination metadata
            lanes["coordination"]["total_sessions_active"] = sum(
                len(l.get("assigned_sessions", [])) for l in lanes["lanes"].values()
            )
            
            self._log_activity("lane_activity", "session_assigned", {
                "lane": lane_id,
                "session": session_id
            })
            
            logger.info(f"Assigned session {session_id} to lane {lane_id}")
            return True
            
        finally:
            self._save_lanes(lanes)
    
    def remove_session_from_lane(self, lane_id: str, session_id: str) -> bool:
        """Remove a session from a lane"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                logger.error(f"Lane {lane_id} not found")
                return False
            
            lane = lanes["lanes"][lane_id]
            
            if session_id in lane.get("assigned_sessions", []):
                lane["assigned_sessions"].remove(session_id)
            
            # Update coordination metadata
            lanes["coordination"]["total_sessions_active"] = sum(
                len(l.get("assigned_sessions", [])) for l in lanes["lanes"].values()
            )
            
            self._log_activity("lane_activity", "session_removed", {
                "lane": lane_id,
                "session": session_id
            })
            
            logger.info(f"Removed session {session_id} from lane {lane_id}")
            return True
            
        finally:
            self._save_lanes(lanes)
    
    def add_blocker(self, lane_id: str, blocker_description: str, severity: str = "medium") -> bool:
        """Add a blocker to a lane"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                logger.error(f"Lane {lane_id} not found")
                return False
            
            lane = lanes["lanes"][lane_id]
            
            blocker = {
                "id": f"blocker_{int(datetime.now().timestamp())}",
                "description": blocker_description,
                "severity": severity,
                "created_date": datetime.now().isoformat(),
                "status": "active"
            }
            
            if "blockers" not in lane:
                lane["blockers"] = []
            lane["blockers"].append(blocker)
            
            self._log_activity("lane_activity", "blocker_added", {
                "lane": lane_id,
                "blocker": blocker
            })
            
            logger.info(f"Added blocker to lane {lane_id}: {blocker_description}")
            return True
            
        finally:
            self._save_lanes(lanes)
    
    def resolve_blocker(self, lane_id: str, blocker_id: str, resolution: str = "") -> bool:
        """Resolve a blocker in a lane"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                logger.error(f"Lane {lane_id} not found")
                return False
            
            lane = lanes["lanes"][lane_id]
            
            for blocker in lane.get("blockers", []):
                if blocker["id"] == blocker_id:
                    blocker["status"] = "resolved"
                    blocker["resolved_date"] = datetime.now().isoformat()
                    blocker["resolution"] = resolution
                    
                    self._log_activity("lane_activity", "blocker_resolved", {
                        "lane": lane_id,
                        "blocker_id": blocker_id,
                        "resolution": resolution
                    })
                    
                    logger.info(f"Resolved blocker {blocker_id} in lane {lane_id}")
                    return True
            
            logger.error(f"Blocker {blocker_id} not found in lane {lane_id}")
            return False
            
        finally:
            self._save_lanes(lanes)
    
    def get_lane_status(self, lane_id: str) -> Optional[Dict[str, Any]]:
        """Get detailed status of a specific lane"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                return None
            
            lane = lanes["lanes"][lane_id].copy()
            
            # Update progress before returning
            self._update_lane_progress(lanes, lane_id)
            lane["progress"] = lanes["lanes"][lane_id]["progress"]
            
            # Add task details
            task_registry = self._load_tasks()
            task_details = []
            
            for task_id in lane.get("tasks", []):
                if task_id in task_registry.get("tasks", {}):
                    task = task_registry["tasks"][task_id]
                    task_details.append({
                        "id": task_id,
                        "title": task.get("title", "Unknown"),
                        "status": task.get("status", "unknown"),
                        "progress_percent": task.get("progress_percent", 0),
                        "assigned_agents": task.get("assigned_agents", [])
                    })
            
            lane["task_details"] = task_details
            
            return lane
            
        finally:
            # Save updates made during progress calculation
            self._save_lanes(lanes)
    
    def get_all_lanes_status(self) -> Dict[str, Any]:
        """Get status of all lanes with summary information"""
        lanes = self._load_lanes()
        
        try:
            # Update progress for all lanes
            for lane_id in lanes["lanes"]:
                self._update_lane_progress(lanes, lane_id)
            
            # Calculate overall system metrics
            total_progress = 0
            active_count = 0
            completed_count = 0
            
            for lane in lanes["lanes"].values():
                total_progress += lane["progress"]["overall_percent"]
                if lane["status"] == "active":
                    active_count += 1
                elif lane["status"] == "completed":
                    completed_count += 1
            
            overall_progress = int(total_progress / len(lanes["lanes"])) if lanes["lanes"] else 0
            
            return {
                "overall_progress": overall_progress,
                "active_lanes": active_count,
                "completed_lanes": completed_count,
                "total_lanes": len(lanes["lanes"]),
                "lanes": {
                    lane_id: {
                        "name": lane["name"],
                        "status": lane["status"],
                        "progress_percent": lane["progress"]["overall_percent"],
                        "tasks_completed": lane["progress"]["tasks_completed"],
                        "tasks_total": lane["progress"]["tasks_total"],
                        "assigned_sessions": len(lane.get("assigned_sessions", [])),
                        "active_blockers": len([b for b in lane.get("blockers", []) if b.get("status") == "active"])
                    }
                    for lane_id, lane in lanes["lanes"].items()
                },
                "coordination": lanes.get("coordination", {}),
                "last_updated": lanes.get("last_updated")
            }
            
        finally:
            self._save_lanes(lanes)
    
    def get_session_lanes(self, session_id: str) -> List[Dict[str, Any]]:
        """Get all lanes assigned to a specific session"""
        lanes = self._load_lanes()
        
        try:
            session_lanes = []
            
            for lane_id, lane in lanes["lanes"].items():
                if session_id in lane.get("assigned_sessions", []):
                    session_lanes.append({
                        "id": lane_id,
                        "name": lane["name"],
                        "status": lane["status"],
                        "progress_percent": lane["progress"]["overall_percent"],
                        "role": "contributor",  # Could be enhanced to track specific roles
                        "assigned_tasks": lane.get("tasks", [])
                    })
            
            return session_lanes
            
        finally:
            self._release_lock()
    
    def check_dependencies(self, lane_id: str) -> Dict[str, Any]:
        """Check if lane dependencies are satisfied"""
        lanes = self._load_lanes()
        
        try:
            if lane_id not in lanes["lanes"]:
                return {"satisfied": False, "error": f"Lane {lane_id} not found"}
            
            lane = lanes["lanes"][lane_id]
            dependencies = lane.get("dependencies", [])
            
            satisfied = True
            dependency_status = []
            
            for dep_lane_id in dependencies:
                if dep_lane_id not in lanes["lanes"]:
                    dependency_status.append({
                        "lane": dep_lane_id,
                        "satisfied": False,
                        "reason": "Lane not found"
                    })
                    satisfied = False
                else:
                    dep_lane = lanes["lanes"][dep_lane_id]
                    dep_satisfied = dep_lane["status"] == "completed"
                    dependency_status.append({
                        "lane": dep_lane_id,
                        "satisfied": dep_satisfied,
                        "status": dep_lane["status"],
                        "progress": dep_lane["progress"]["overall_percent"]
                    })
                    if not dep_satisfied:
                        satisfied = False
            
            return {
                "satisfied": satisfied,
                "dependencies": dependency_status,
                "can_start": satisfied or lane["status"] in ["active", "completed"]
            }
            
        finally:
            self._release_lock()
    
    def generate_lane_report(self, lane_id: str, format: str = "text") -> str:
        """Generate a comprehensive report for a specific lane"""
        lane_status = self.get_lane_status(lane_id)
        
        if not lane_status:
            return f"Lane {lane_id} not found"
        
        if format == "json":
            return json.dumps(lane_status, indent=2)
        
        # Text format report
        report = []
        report.append(f"=== Lane Report: {lane_status['name']} ===")
        report.append(f"ID: {lane_status['id']}")
        report.append(f"Status: {lane_status['status'].upper()}")
        report.append(f"Phase: {lane_status['phase']}")
        report.append(f"Progress: {lane_status['progress']['overall_percent']}%")
        report.append(f"Duration: Month {lane_status['start_month']}-{lane_status['end_month']}")
        report.append("")
        
        report.append("Description:")
        report.append(f"  {lane_status['description']}")
        report.append("")
        
        report.append("Team Assignment:")
        report.append(f"  Lead Agents: {', '.join(lane_status['lead_agents'])}")
        if lane_status.get('contributor_agents'):
            report.append(f"  Contributors: {', '.join(lane_status['contributor_agents'])}")
        report.append(f"  Active Sessions: {len(lane_status.get('assigned_sessions', []))}")
        report.append("")
        
        report.append("Task Progress:")
        for task in lane_status.get('task_details', []):
            status_symbol = "✓" if task['status'] == "completed" else "○" if task['status'] == "available" else "●"
            report.append(f"  {status_symbol} {task['title']} ({task['status']}) - {task.get('progress_percent', 0)}%")
        report.append("")
        
        if lane_status.get('blockers'):
            active_blockers = [b for b in lane_status['blockers'] if b.get('status') == 'active']
            if active_blockers:
                report.append("Active Blockers:")
                for blocker in active_blockers:
                    report.append(f"  ⚠ {blocker['description']} (Severity: {blocker.get('severity', 'unknown')})")
                report.append("")
        
        if lane_status.get('milestones'):
            report.append("Milestones:")
            for milestone in lane_status['milestones']:
                status_symbol = "✓" if milestone['status'] == "completed" else "○"
                report.append(f"  {status_symbol} {milestone['name']} (Target: {milestone['target_date']})")
            report.append("")
        
        if lane_status.get('dependencies'):
            report.append(f"Dependencies: {', '.join(lane_status['dependencies'])}")
            report.append("")
        
        report.append(f"Last Updated: {lane_status['progress']['last_update']}")
        
        return "\n".join(report)

def main():
    """Command line interface for lane management"""
    parser = argparse.ArgumentParser(description="BRAIN Lane Management System")
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Lane status
    status_parser = subparsers.add_parser('status', help='Get lane status')
    status_parser.add_argument('--lane', help='Specific lane ID')
    status_parser.add_argument('--session', help='Session ID to filter lanes')
    status_parser.add_argument('--all', action='store_true', help='Show all lanes status')
    status_parser.add_argument('--format', choices=['table', 'json'], default='table', help='Output format')
    
    # Assign agent
    assign_parser = subparsers.add_parser('assign', help='Assign agent to lane')
    assign_parser.add_argument('--agent', required=True, help='Agent name')
    assign_parser.add_argument('--lane', required=True, help='Lane ID')
    assign_parser.add_argument('--role', choices=['lead', 'contributor'], default='contributor', help='Agent role')
    
    # Assign session
    session_parser = subparsers.add_parser('assign-session', help='Assign session to lane')
    session_parser.add_argument('--session', required=True, help='Session ID')
    session_parser.add_argument('--lane', required=True, help='Lane ID')
    
    # Remove session
    remove_parser = subparsers.add_parser('remove-session', help='Remove session from lane')
    remove_parser.add_argument('--session', required=True, help='Session ID')
    remove_parser.add_argument('--lane', required=True, help='Lane ID')
    
    # Add blocker
    block_parser = subparsers.add_parser('add-blocker', help='Add blocker to lane')
    block_parser.add_argument('--lane', required=True, help='Lane ID')
    block_parser.add_argument('--description', required=True, help='Blocker description')
    block_parser.add_argument('--severity', choices=['low', 'medium', 'high', 'critical'], default='medium', help='Blocker severity')
    
    # Resolve blocker
    resolve_parser = subparsers.add_parser('resolve-blocker', help='Resolve lane blocker')
    resolve_parser.add_argument('--lane', required=True, help='Lane ID')
    resolve_parser.add_argument('--blocker', required=True, help='Blocker ID')
    resolve_parser.add_argument('--resolution', help='Resolution description')
    
    # Check dependencies
    deps_parser = subparsers.add_parser('check-deps', help='Check lane dependencies')
    deps_parser.add_argument('--lane', required=True, help='Lane ID')
    
    # Generate report
    report_parser = subparsers.add_parser('report', help='Generate lane report')
    report_parser.add_argument('--lane', required=True, help='Lane ID')
    report_parser.add_argument('--format', choices=['text', 'json'], default='text', help='Output format')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    lane_manager = LaneManager()
    
    try:
        if args.command == 'status':
            if args.all:
                status = lane_manager.get_all_lanes_status()
                if args.format == 'json':
                    print(json.dumps(status, indent=2))
                else:
                    print(f"\n=== BRAIN Development Lanes Status ===")
                    print(f"Overall Progress: {status['overall_progress']}%")
                    print(f"Active Lanes: {status['active_lanes']}/{status['total_lanes']}")
                    print(f"Completed Lanes: {status['completed_lanes']}/{status['total_lanes']}")
                    print("\nLane Details:")
                    print(f"{'ID':<8} {'Name':<25} {'Status':<12} {'Progress':<10} {'Tasks':<10} {'Sessions':<10} {'Blockers':<10}")
                    print("-" * 100)
                    for lane_id, lane in status['lanes'].items():
                        print(f"{lane_id:<8} {lane['name'][:23]:<25} {lane['status']:<12} {lane['progress_percent']:>3}%{'':<7} {lane['tasks_completed']}/{lane['tasks_total']:<8} {lane['assigned_sessions']:<10} {lane['active_blockers']:<10}")
            
            elif args.lane:
                lane_status = lane_manager.get_lane_status(args.lane)
                if lane_status:
                    if args.format == 'json':
                        print(json.dumps(lane_status, indent=2))
                    else:
                        print(lane_manager.generate_lane_report(args.lane))
                else:
                    print(f"Lane {args.lane} not found")
                    sys.exit(1)
            
            elif args.session:
                session_lanes = lane_manager.get_session_lanes(args.session)
                if args.format == 'json':
                    print(json.dumps(session_lanes, indent=2))
                else:
                    print(f"\nLanes assigned to session {args.session}:")
                    if session_lanes:
                        for lane in session_lanes:
                            print(f"  {lane['id']}: {lane['name']} ({lane['status']}, {lane['progress_percent']}% complete)")
                    else:
                        print("  No lanes assigned to this session")
            else:
                print("Specify --lane, --session, or --all for status")
                sys.exit(1)
        
        elif args.command == 'assign':
            success = lane_manager.assign_agent_to_lane(args.lane, args.agent, args.role)
            if success:
                print(f"✓ Assigned agent {args.agent} to lane {args.lane} as {args.role}")
            else:
                print(f"✗ Failed to assign agent {args.agent} to lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'assign-session':
            success = lane_manager.assign_session_to_lane(args.lane, args.session)
            if success:
                print(f"✓ Assigned session {args.session} to lane {args.lane}")
            else:
                print(f"✗ Failed to assign session {args.session} to lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'remove-session':
            success = lane_manager.remove_session_from_lane(args.lane, args.session)
            if success:
                print(f"✓ Removed session {args.session} from lane {args.lane}")
            else:
                print(f"✗ Failed to remove session {args.session} from lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'add-blocker':
            success = lane_manager.add_blocker(args.lane, args.description, args.severity)
            if success:
                print(f"✓ Added {args.severity} severity blocker to lane {args.lane}")
            else:
                print(f"✗ Failed to add blocker to lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'resolve-blocker':
            success = lane_manager.resolve_blocker(args.lane, args.blocker, args.resolution or "")
            if success:
                print(f"✓ Resolved blocker {args.blocker} in lane {args.lane}")
            else:
                print(f"✗ Failed to resolve blocker {args.blocker} in lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'check-deps':
            deps = lane_manager.check_dependencies(args.lane)
            print(json.dumps(deps, indent=2))
        
        elif args.command == 'report':
            report = lane_manager.generate_lane_report(args.lane, args.format)
            print(report)
    
    except Exception as e:
        logger.error(f"Command failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()