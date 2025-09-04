#!/usr/bin/env python3
"""
Task Check-in/Check-out System
BRAIN Multi-Agent Development Coordination

Manages task reservation and completion to prevent conflicts across multiple Claude sessions.
"""

import json
import time
import argparse
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any
import fcntl
import logging

# Configuration
BRAIN_ROOT = Path(__file__).parent.parent
TASK_REGISTRY_FILE = BRAIN_ROOT / "TASK_REGISTRY.json"
DEVELOPMENT_LOG_FILE = BRAIN_ROOT / "DEVELOPMENT_LOG.jsonl"
LOCK_FILE = BRAIN_ROOT / "coordination" / ".task_lock"
DEFAULT_TIMEOUT_HOURS = 4
HEARTBEAT_INTERVAL_MINUTES = 15

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class TaskManager:
    """Manages task check-in/check-out operations with file locking"""
    
    def __init__(self):
        self.task_registry_file = TASK_REGISTRY_FILE
        self.log_file = DEVELOPMENT_LOG_FILE
        self.lock_file = LOCK_FILE
        self._ensure_files_exist()
        
    def _ensure_files_exist(self):
        """Ensure required files exist with proper structure"""
        # Create task registry if it doesn't exist
        if not self.task_registry_file.exists():
            self._initialize_task_registry()
            
        # Create log file if it doesn't exist
        if not self.log_file.exists():
            self.log_file.touch()
            
        # Create lock file if it doesn't exist
        if not self.lock_file.exists():
            self.lock_file.touch()
    
    def _initialize_task_registry(self):
        """Initialize task registry with BRAIN development tasks"""
        initial_tasks = {
            "version": "1.0",
            "last_updated": datetime.now().isoformat(),
            "tasks": {
                # Lane 1: Foundation Infrastructure
                "runtime-engine-core": {
                    "id": "runtime-engine-core",
                    "title": "Runtime Execution Engine Implementation",
                    "description": "Core agent lifecycle management, task scheduling, session management",
                    "lane": "Lane1",
                    "phase": "Foundation",
                    "priority": "P0",
                    "estimated_hours": 160,
                    "status": "available",
                    "dependencies": [],
                    "assigned_agents": ["Manager", "Architect", "Developer"],
                    "subtasks": [
                        "agent-lifecycle-management",
                        "task-scheduling-engine", 
                        "session-management-system",
                        "integration-testing"
                    ],
                    "created_date": datetime.now().isoformat(),
                    "completion_criteria": [
                        "Single agent activation in <2 seconds",
                        "Task execution with coordination",
                        "Session persistence across restarts",
                        "90% unit test coverage"
                    ]
                },
                "agent-communication-infrastructure": {
                    "id": "agent-communication-infrastructure",
                    "title": "Agent Communication Infrastructure",
                    "description": "Message passing, event bus, collaboration protocols",
                    "lane": "Lane1", 
                    "phase": "Foundation",
                    "priority": "P0",
                    "estimated_hours": 160,
                    "status": "available",
                    "dependencies": ["runtime-engine-core"],
                    "assigned_agents": ["Architect", "Developer", "Analyst"],
                    "subtasks": [
                        "message-passing-system",
                        "event-bus-architecture",
                        "collaboration-protocols",
                        "testing-documentation"
                    ],
                    "created_date": datetime.now().isoformat(),
                    "completion_criteria": [
                        "Inter-agent message latency <50ms",
                        "Event throughput >1000 events/second", 
                        "Multi-agent collaboration working",
                        "Zero message loss under normal conditions"
                    ]
                },
                "database-persistence-layer": {
                    "id": "database-persistence-layer",
                    "title": "Database and Persistence Layer",
                    "description": "Enhanced schema, connection pooling, backup/recovery",
                    "lane": "Lane1",
                    "phase": "Foundation", 
                    "priority": "P1",
                    "estimated_hours": 160,
                    "status": "available",
                    "dependencies": [],
                    "assigned_agents": ["Developer", "Topologist", "Analyst"],
                    "subtasks": [
                        "enhanced-schema-design",
                        "connection-pool-management",
                        "backup-recovery-system", 
                        "performance-optimization"
                    ],
                    "created_date": datetime.now().isoformat(),
                    "completion_criteria": [
                        "Database queries <10ms average",
                        "Automated backups working",
                        "Migration system functional",
                        "10,000+ concurrent connections supported"
                    ]
                },
                "trinity-curation-system": {
                    "id": "trinity-curation-system",
                    "title": "Trinity Curation System Automation",
                    "description": "Automated Shannon/Architect/Memory analysis and knowledge integration",
                    "lane": "Lane2",
                    "phase": "Intelligence",
                    "priority": "P1", 
                    "estimated_hours": 120,
                    "status": "available",
                    "dependencies": ["runtime-engine-core", "agent-communication-infrastructure"],
                    "assigned_agents": ["Analyst", "Planner", "Developer"],
                    "subtasks": [
                        "shannon-perspective-implementation",
                        "architect-perspective-implementation", 
                        "memory-perspective-implementation",
                        "automated-integration-pipeline"
                    ],
                    "created_date": datetime.now().isoformat(),
                    "completion_criteria": [
                        "Automated Trinity evaluation system",
                        "Knowledge integration pipeline", 
                        "Quality validation framework",
                        "Processing throughput targets met"
                    ]
                },
                "security-access-control": {
                    "id": "security-access-control",
                    "title": "Security and Access Control Implementation",
                    "description": "Authentication, authorization, encryption, audit logging",
                    "lane": "Lane3",
                    "phase": "Production",
                    "priority": "P1",
                    "estimated_hours": 160,
                    "status": "available", 
                    "dependencies": ["runtime-engine-core"],
                    "assigned_agents": ["Enforcer", "Developer", "Analyst"],
                    "subtasks": [
                        "authentication-system",
                        "authorization-framework",
                        "data-protection",
                        "security-monitoring"
                    ],
                    "created_date": datetime.now().isoformat(),
                    "completion_criteria": [
                        "Complete security framework",
                        "Compliance certification",
                        "Security testing validation", 
                        "Penetration testing passed"
                    ]
                }
            },
            "metadata": {
                "total_tasks": 5,
                "available_tasks": 5,
                "reserved_tasks": 0,
                "completed_tasks": 0,
                "last_cleanup": datetime.now().isoformat()
            }
        }
        
        with open(self.task_registry_file, 'w') as f:
            json.dump(initial_tasks, f, indent=2)
        
        logger.info(f"Initialized task registry with {len(initial_tasks['tasks'])} tasks")
    
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
    
    def _load_registry(self) -> Dict[str, Any]:
        """Load task registry with locking"""
        if not self._acquire_lock():
            raise Exception("Could not acquire lock on task registry")
        
        try:
            with open(self.task_registry_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, FileNotFoundError) as e:
            logger.error(f"Could not load task registry: {e}")
            raise
    
    def _save_registry(self, registry: Dict[str, Any]):
        """Save task registry and release lock"""
        try:
            registry["last_updated"] = datetime.now().isoformat()
            with open(self.task_registry_file, 'w') as f:
                json.dump(registry, f, indent=2)
        finally:
            self._release_lock()
    
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
    
    def _cleanup_expired_tasks(self, registry: Dict[str, Any]) -> int:
        """Clean up expired task reservations"""
        cleaned_count = 0
        current_time = datetime.now()
        
        for task_id, task in registry["tasks"].items():
            if task["status"] == "reserved" and "reserved_until" in task:
                reserved_until = datetime.fromisoformat(task["reserved_until"])
                if current_time > reserved_until:
                    task["status"] = "available"
                    task.pop("reserved_by", None)
                    task.pop("reserved_session", None) 
                    task.pop("reserved_until", None)
                    task.pop("last_heartbeat", None)
                    
                    self._log_activity("task_activity", "task_expired", {
                        "task": task_id,
                        "expired_at": current_time.isoformat()
                    })
                    
                    cleaned_count += 1
                    logger.info(f"Cleaned up expired task: {task_id}")
        
        if cleaned_count > 0:
            registry["metadata"]["last_cleanup"] = current_time.isoformat()
        
        return cleaned_count
    
    def list_available_tasks(self, lane: Optional[str] = None, agent: Optional[str] = None) -> List[Dict[str, Any]]:
        """List available tasks, optionally filtered by lane and agent"""
        registry = self._load_registry()
        
        try:
            # Clean up expired tasks first
            self._cleanup_expired_tasks(registry)
            
            available_tasks = []
            for task_id, task in registry["tasks"].items():
                if task["status"] == "available":
                    # Filter by lane if specified
                    if lane and task.get("lane") != lane:
                        continue
                    
                    # Filter by agent if specified
                    if agent and agent not in task.get("assigned_agents", []):
                        continue
                    
                    # Check if dependencies are met
                    dependencies_met = True
                    for dep_id in task.get("dependencies", []):
                        if dep_id in registry["tasks"]:
                            dep_task = registry["tasks"][dep_id]
                            if dep_task["status"] != "completed":
                                dependencies_met = False
                                break
                    
                    if dependencies_met:
                        available_tasks.append({
                            "id": task_id,
                            "title": task["title"],
                            "lane": task.get("lane"),
                            "priority": task.get("priority"),
                            "estimated_hours": task.get("estimated_hours"),
                            "assigned_agents": task.get("assigned_agents", []),
                            "dependencies": task.get("dependencies", [])
                        })
            
            # Sort by priority (P0 first, then P1, etc.)
            priority_order = {"P0": 0, "P1": 1, "P2": 2, "P3": 3, "P4": 4}
            available_tasks.sort(key=lambda x: priority_order.get(x.get("priority", "P4"), 4))
            
            return available_tasks
            
        finally:
            self._save_registry(registry)
    
    def reserve_task(self, task_id: str, session_id: str, agent: str, timeout_hours: int = DEFAULT_TIMEOUT_HOURS) -> bool:
        """Reserve a task for a specific session and agent"""
        registry = self._load_registry()
        
        try:
            # Clean up expired tasks first
            self._cleanup_expired_tasks(registry)
            
            if task_id not in registry["tasks"]:
                logger.error(f"Task {task_id} not found")
                return False
            
            task = registry["tasks"][task_id]
            
            # Check if task is available
            if task["status"] != "available":
                logger.error(f"Task {task_id} is not available (status: {task['status']})")
                return False
            
            # Check if agent is assigned to this task
            if agent not in task.get("assigned_agents", []):
                logger.warning(f"Agent {agent} not assigned to task {task_id}, but allowing reservation")
            
            # Check dependencies
            for dep_id in task.get("dependencies", []):
                if dep_id in registry["tasks"]:
                    dep_task = registry["tasks"][dep_id]
                    if dep_task["status"] != "completed":
                        logger.error(f"Task {task_id} depends on uncompleted task {dep_id}")
                        return False
            
            # Reserve the task
            reserved_until = datetime.now() + timedelta(hours=timeout_hours)
            task["status"] = "reserved"
            task["reserved_by"] = agent
            task["reserved_session"] = session_id
            task["reserved_until"] = reserved_until.isoformat()
            task["last_heartbeat"] = datetime.now().isoformat()
            
            # Update metadata
            registry["metadata"]["available_tasks"] -= 1
            registry["metadata"]["reserved_tasks"] += 1
            
            self._log_activity("task_activity", "task_reserved", {
                "task": task_id,
                "session": session_id,
                "agent": agent,
                "reserved_until": reserved_until.isoformat(),
                "priority": task.get("priority"),
                "estimated_hours": task.get("estimated_hours")
            })
            
            logger.info(f"Reserved task {task_id} for agent {agent} (session {session_id}) until {reserved_until}")
            return True
            
        finally:
            self._save_registry(registry)
    
    def heartbeat_task(self, task_id: str, session_id: str) -> bool:
        """Send heartbeat for a reserved task"""
        registry = self._load_registry()
        
        try:
            if task_id not in registry["tasks"]:
                logger.error(f"Task {task_id} not found")
                return False
            
            task = registry["tasks"][task_id]
            
            # Verify task is reserved by this session
            if task["status"] != "reserved" or task.get("reserved_session") != session_id:
                logger.error(f"Task {task_id} not reserved by session {session_id}")
                return False
            
            # Update heartbeat
            task["last_heartbeat"] = datetime.now().isoformat()
            
            self._log_activity("task_activity", "task_heartbeat", {
                "task": task_id,
                "session": session_id,
                "heartbeat_time": task["last_heartbeat"]
            })
            
            logger.info(f"Heartbeat updated for task {task_id} from session {session_id}")
            return True
            
        finally:
            self._save_registry(registry)
    
    def update_progress(self, task_id: str, session_id: str, progress_percent: int, notes: str = "") -> bool:
        """Update task progress"""
        registry = self._load_registry()
        
        try:
            if task_id not in registry["tasks"]:
                logger.error(f"Task {task_id} not found")
                return False
            
            task = registry["tasks"][task_id]
            
            # Verify task is reserved by this session
            if task["status"] != "reserved" or task.get("reserved_session") != session_id:
                logger.error(f"Task {task_id} not reserved by session {session_id}")
                return False
            
            # Update progress
            task["progress_percent"] = min(100, max(0, progress_percent))
            task["last_progress_update"] = datetime.now().isoformat()
            if notes:
                if "progress_notes" not in task:
                    task["progress_notes"] = []
                task["progress_notes"].append({
                    "timestamp": datetime.now().isoformat(),
                    "notes": notes,
                    "progress": progress_percent
                })
            
            # Update status if in progress
            if task["status"] == "reserved" and progress_percent > 0:
                task["status"] = "in_progress"
            
            self._log_activity("task_activity", "task_progress_updated", {
                "task": task_id,
                "session": session_id,
                "progress_percent": progress_percent,
                "notes": notes
            })
            
            logger.info(f"Updated progress for task {task_id}: {progress_percent}%")
            return True
            
        finally:
            self._save_registry(registry)
    
    def complete_task(self, task_id: str, session_id: str, deliverables: str, notes: str = "") -> bool:
        """Mark task as completed with deliverables"""
        registry = self._load_registry()
        
        try:
            if task_id not in registry["tasks"]:
                logger.error(f"Task {task_id} not found")
                return False
            
            task = registry["tasks"][task_id]
            
            # Verify task is reserved by this session  
            if task.get("reserved_session") != session_id:
                logger.error(f"Task {task_id} not reserved by session {session_id}")
                return False
            
            # Mark as completed
            task["status"] = "completed"
            task["completed_date"] = datetime.now().isoformat()
            task["deliverables"] = deliverables
            task["completion_notes"] = notes
            task["progress_percent"] = 100
            
            # Clean up reservation fields
            task.pop("reserved_by", None)
            task.pop("reserved_session", None)
            task.pop("reserved_until", None)
            task.pop("last_heartbeat", None)
            
            # Update metadata
            if task["status"] == "reserved":
                registry["metadata"]["reserved_tasks"] -= 1
            registry["metadata"]["completed_tasks"] += 1
            
            self._log_activity("task_activity", "task_completed", {
                "task": task_id,
                "session": session_id,
                "completed_date": task["completed_date"],
                "deliverables": deliverables,
                "notes": notes
            })
            
            logger.info(f"Completed task {task_id} with deliverables: {deliverables}")
            return True
            
        finally:
            self._save_registry(registry)
    
    def release_task(self, task_id: str, session_id: str, reason: str = "") -> bool:
        """Release a reserved task back to available status"""
        registry = self._load_registry()
        
        try:
            if task_id not in registry["tasks"]:
                logger.error(f"Task {task_id} not found")
                return False
            
            task = registry["tasks"][task_id]
            
            # Verify task is reserved by this session
            if task.get("reserved_session") != session_id:
                logger.error(f"Task {task_id} not reserved by session {session_id}")
                return False
            
            # Release the task
            task["status"] = "available"
            task.pop("reserved_by", None)
            task.pop("reserved_session", None)
            task.pop("reserved_until", None) 
            task.pop("last_heartbeat", None)
            task.pop("progress_percent", None)
            task.pop("progress_notes", None)
            
            # Update metadata
            registry["metadata"]["available_tasks"] += 1
            registry["metadata"]["reserved_tasks"] -= 1
            
            self._log_activity("task_activity", "task_released", {
                "task": task_id,
                "session": session_id,
                "reason": reason
            })
            
            logger.info(f"Released task {task_id} from session {session_id}: {reason}")
            return True
            
        finally:
            self._save_registry(registry)
    
    def get_task_status(self, task_id: str) -> Optional[Dict[str, Any]]:
        """Get detailed status of a specific task"""
        registry = self._load_registry()
        
        try:
            if task_id not in registry["tasks"]:
                return None
            
            task = registry["tasks"][task_id]
            
            # Check if reserved task has expired
            if task["status"] == "reserved" and "reserved_until" in task:
                reserved_until = datetime.fromisoformat(task["reserved_until"])
                if datetime.now() > reserved_until:
                    # Mark as expired but don't modify registry here
                    task = task.copy()
                    task["_expired"] = True
            
            return task
            
        finally:
            self._release_lock()
    
    def get_session_tasks(self, session_id: str) -> List[Dict[str, Any]]:
        """Get all tasks assigned to a specific session"""
        registry = self._load_registry()
        
        try:
            session_tasks = []
            for task_id, task in registry["tasks"].items():
                if task.get("reserved_session") == session_id:
                    session_tasks.append({
                        "id": task_id,
                        "title": task["title"],
                        "status": task["status"],
                        "progress_percent": task.get("progress_percent", 0),
                        "reserved_until": task.get("reserved_until"),
                        "last_heartbeat": task.get("last_heartbeat")
                    })
            
            return session_tasks
            
        finally:
            self._release_lock()
    
    def get_system_status(self) -> Dict[str, Any]:
        """Get overall system status and statistics"""
        registry = self._load_registry()
        
        try:
            # Count tasks by status
            status_counts = {"available": 0, "reserved": 0, "in_progress": 0, "completed": 0, "blocked": 0}
            lane_counts = {}
            active_sessions = set()
            
            for task in registry["tasks"].values():
                status_counts[task["status"]] = status_counts.get(task["status"], 0) + 1
                
                lane = task.get("lane", "Unknown")
                lane_counts[lane] = lane_counts.get(lane, 0) + 1
                
                if task.get("reserved_session"):
                    active_sessions.add(task["reserved_session"])
            
            return {
                "version": registry.get("version"),
                "last_updated": registry.get("last_updated"),
                "total_tasks": len(registry["tasks"]),
                "status_counts": status_counts,
                "lane_counts": lane_counts,
                "active_sessions": len(active_sessions),
                "session_list": list(active_sessions),
                "metadata": registry.get("metadata", {})
            }
            
        finally:
            self._release_lock()

def main():
    """Command line interface for task management"""
    parser = argparse.ArgumentParser(description="BRAIN Task Check-in/Check-out System")
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # List available tasks
    list_parser = subparsers.add_parser('available', help='List available tasks')
    list_parser.add_argument('--lane', help='Filter by lane')
    list_parser.add_argument('--agent', help='Filter by agent')
    list_parser.add_argument('--format', choices=['table', 'json'], default='table', help='Output format')
    
    # Reserve task
    reserve_parser = subparsers.add_parser('reserve', help='Reserve a task')
    reserve_parser.add_argument('--task', required=True, help='Task ID to reserve')
    reserve_parser.add_argument('--session', required=True, help='Session ID')
    reserve_parser.add_argument('--agent', required=True, help='Agent name')
    reserve_parser.add_argument('--timeout', type=int, default=DEFAULT_TIMEOUT_HOURS, help='Timeout in hours')
    
    # Heartbeat
    heartbeat_parser = subparsers.add_parser('heartbeat', help='Send heartbeat for reserved task')
    heartbeat_parser.add_argument('--task', required=True, help='Task ID')
    heartbeat_parser.add_argument('--session', required=True, help='Session ID')
    
    # Update progress
    progress_parser = subparsers.add_parser('progress', help='Update task progress')
    progress_parser.add_argument('--task', required=True, help='Task ID')
    progress_parser.add_argument('--session', required=True, help='Session ID')
    progress_parser.add_argument('--percent', type=int, required=True, help='Progress percentage')
    progress_parser.add_argument('--notes', help='Progress notes')
    
    # Complete task
    complete_parser = subparsers.add_parser('complete', help='Complete a task')
    complete_parser.add_argument('--task', required=True, help='Task ID')
    complete_parser.add_argument('--session', required=True, help='Session ID')
    complete_parser.add_argument('--deliverables', required=True, help='Deliverables description')
    complete_parser.add_argument('--notes', help='Completion notes')
    
    # Release task
    release_parser = subparsers.add_parser('release', help='Release a reserved task')
    release_parser.add_argument('--task', required=True, help='Task ID')
    release_parser.add_argument('--session', required=True, help='Session ID')
    release_parser.add_argument('--reason', help='Release reason')
    
    # Task status
    status_parser = subparsers.add_parser('status', help='Get task status')
    status_parser.add_argument('--task', help='Specific task ID')
    status_parser.add_argument('--session', help='Session ID to filter tasks')
    status_parser.add_argument('--system', action='store_true', help='Show system status')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    task_manager = TaskManager()
    
    try:
        if args.command == 'available':
            tasks = task_manager.list_available_tasks(args.lane, args.agent)
            
            if args.format == 'json':
                print(json.dumps(tasks, indent=2))
            else:
                print(f"\n{'ID':<30} {'Title':<40} {'Lane':<8} {'Priority':<8} {'Hours':<6} {'Agents'}")
                print("-" * 120)
                for task in tasks:
                    agents = ", ".join(task.get('assigned_agents', [])[:2])
                    if len(task.get('assigned_agents', [])) > 2:
                        agents += "..."
                    print(f"{task['id']:<30} {task['title'][:38]:<40} {task.get('lane', 'N/A'):<8} {task.get('priority', 'N/A'):<8} {task.get('estimated_hours', 0):<6} {agents}")
                
                if not tasks:
                    print("No available tasks found.")
        
        elif args.command == 'reserve':
            success = task_manager.reserve_task(args.task, args.session, args.agent, args.timeout)
            if success:
                print(f"✓ Successfully reserved task {args.task}")
            else:
                print(f"✗ Failed to reserve task {args.task}")
                sys.exit(1)
        
        elif args.command == 'heartbeat':
            success = task_manager.heartbeat_task(args.task, args.session)
            if success:
                print(f"✓ Heartbeat sent for task {args.task}")
            else:
                print(f"✗ Failed to send heartbeat for task {args.task}")
                sys.exit(1)
        
        elif args.command == 'progress':
            success = task_manager.update_progress(args.task, args.session, args.percent, args.notes or "")
            if success:
                print(f"✓ Updated progress for task {args.task}: {args.percent}%")
            else:
                print(f"✗ Failed to update progress for task {args.task}")
                sys.exit(1)
        
        elif args.command == 'complete':
            success = task_manager.complete_task(args.task, args.session, args.deliverables, args.notes or "")
            if success:
                print(f"✓ Successfully completed task {args.task}")
            else:
                print(f"✗ Failed to complete task {args.task}")
                sys.exit(1)
        
        elif args.command == 'release':
            success = task_manager.release_task(args.task, args.session, args.reason or "")
            if success:
                print(f"✓ Successfully released task {args.task}")
            else:
                print(f"✗ Failed to release task {args.task}")
                sys.exit(1)
        
        elif args.command == 'status':
            if args.system:
                status = task_manager.get_system_status()
                print(json.dumps(status, indent=2))
            elif args.task:
                task_status = task_manager.get_task_status(args.task)
                if task_status:
                    print(json.dumps(task_status, indent=2))
                else:
                    print(f"Task {args.task} not found")
                    sys.exit(1)
            elif args.session:
                session_tasks = task_manager.get_session_tasks(args.session)
                print(json.dumps(session_tasks, indent=2))
            else:
                print("Specify --task, --session, or --system for status")
                sys.exit(1)
    
    except Exception as e:
        logger.error(f"Command failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()