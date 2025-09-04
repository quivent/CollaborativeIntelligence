#!/usr/bin/env python3
"""
Session Coordination System
BRAIN Multi-Agent Development Coordination

Manages multiple Claude Code sessions, heartbeats, and coordination.
"""

import json
import argparse
import os
import sys
import socket
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any
import fcntl
import logging
import uuid

# Configuration
BRAIN_ROOT = Path(__file__).parent.parent
SESSION_COORDINATION_FILE = BRAIN_ROOT / "SESSION_COORDINATION.json"
DEVELOPMENT_LOG_FILE = BRAIN_ROOT / "DEVELOPMENT_LOG.jsonl"
LOCK_FILE = BRAIN_ROOT / "coordination" / ".session_lock"
DEFAULT_HEARTBEAT_INTERVAL = 300  # 5 minutes
HEARTBEAT_TIMEOUT_MULTIPLIER = 3  # 3 missed heartbeats = timeout

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class SessionCoordinator:
    """Manages multiple Claude Code session coordination"""
    
    def __init__(self):
        self.session_file = SESSION_COORDINATION_FILE
        self.log_file = DEVELOPMENT_LOG_FILE
        self.lock_file = LOCK_FILE
        self._ensure_files_exist()
        
    def _ensure_files_exist(self):
        """Ensure required files exist with proper structure"""
        if not self.session_file.exists():
            self._initialize_session_coordination()
            
        if not self.log_file.exists():
            self.log_file.touch()
            
        if not self.lock_file.exists():
            self.lock_file.touch()
    
    def _initialize_session_coordination(self):
        """Initialize session coordination with empty structure"""
        initial_data = {
            "version": "1.0",
            "last_updated": datetime.now().isoformat(),
            "sessions": {},
            "coordination": {
                "active_sessions": 0,
                "total_sessions_registered": 0,
                "session_timeouts": 0,
                "last_cleanup": datetime.now().isoformat(),
                "heartbeat_interval_seconds": DEFAULT_HEARTBEAT_INTERVAL
            },
            "metadata": {
                "max_concurrent_sessions": 10,
                "session_timeout_minutes": 15,
                "auto_cleanup_enabled": True,
                "last_update": datetime.now().isoformat()
            }
        }
        
        with open(self.session_file, 'w') as f:
            json.dump(initial_data, f, indent=2)
        
        logger.info("Initialized session coordination system")
    
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
    
    def _load_sessions(self) -> Dict[str, Any]:
        """Load session coordination data with locking"""
        if not self._acquire_lock():
            raise Exception("Could not acquire lock on session coordination")
        
        try:
            with open(self.session_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, FileNotFoundError) as e:
            logger.error(f"Could not load session coordination: {e}")
            raise
    
    def _save_sessions(self, sessions: Dict[str, Any]):
        """Save session coordination data and release lock"""
        try:
            sessions["last_updated"] = datetime.now().isoformat()
            with open(self.session_file, 'w') as f:
                json.dump(sessions, f, indent=2)
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
    
    def _generate_session_id(self, prefix: str = None) -> str:
        """Generate a unique session ID"""
        if prefix:
            return f"{prefix}-{int(datetime.now().timestamp())}-{uuid.uuid4().hex[:8]}"
        else:
            hostname = socket.gethostname().replace('.', '-')
            return f"claude-{hostname}-{int(datetime.now().timestamp())}-{uuid.uuid4().hex[:8]}"
    
    def _cleanup_stale_sessions(self, sessions: Dict[str, Any]) -> int:
        """Clean up stale sessions that have missed heartbeats"""
        cleaned_count = 0
        current_time = datetime.now()
        timeout_threshold = timedelta(minutes=sessions["metadata"]["session_timeout_minutes"])
        
        sessions_to_remove = []
        
        for session_id, session in sessions["sessions"].items():
            if session["status"] == "active":
                last_heartbeat = datetime.fromisoformat(session["last_heartbeat"])
                if current_time - last_heartbeat > timeout_threshold:
                    sessions_to_remove.append(session_id)
        
        for session_id in sessions_to_remove:
            session = sessions["sessions"][session_id]
            session["status"] = "timed_out"
            session["ended_at"] = current_time.isoformat()
            session["timeout_reason"] = "Missed heartbeat timeout"
            
            self._log_activity("session_activity", "session_timeout", {
                "session": session_id,
                "last_heartbeat": session["last_heartbeat"],
                "timeout_at": current_time.isoformat()
            })
            
            cleaned_count += 1
            logger.info(f"Session {session_id} timed out due to missed heartbeats")
        
        if cleaned_count > 0:
            sessions["coordination"]["active_sessions"] = len([
                s for s in sessions["sessions"].values() if s["status"] == "active"
            ])
            sessions["coordination"]["session_timeouts"] += cleaned_count
            sessions["coordination"]["last_cleanup"] = current_time.isoformat()
        
        return cleaned_count
    
    def register_session(self, session_id: str = None, lane: str = None, claude_version: str = None, 
                        user_info: Dict[str, Any] = None) -> str:
        """Register a new Claude Code session"""
        sessions = self._load_sessions()
        
        try:
            # Auto-cleanup stale sessions
            self._cleanup_stale_sessions(sessions)
            
            # Generate session ID if not provided
            if not session_id:
                session_id = self._generate_session_id()
            
            # Check if session already exists
            if session_id in sessions["sessions"]:
                logger.warning(f"Session {session_id} already exists, updating registration")
            
            # Check max concurrent sessions
            active_count = len([s for s in sessions["sessions"].values() if s["status"] == "active"])
            max_concurrent = sessions["metadata"]["max_concurrent_sessions"]
            
            if active_count >= max_concurrent and session_id not in sessions["sessions"]:
                logger.error(f"Maximum concurrent sessions ({max_concurrent}) reached")
                return None
            
            # Create session record
            session_data = {
                "id": session_id,
                "status": "active",
                "registered_at": datetime.now().isoformat(),
                "last_heartbeat": datetime.now().isoformat(),
                "lane": lane,
                "claude_version": claude_version or "Unknown",
                "user_info": user_info or {},
                "host_info": {
                    "hostname": socket.gethostname(),
                    "platform": sys.platform
                },
                "activity": {
                    "tasks_reserved": 0,
                    "tasks_completed": 0,
                    "last_activity": datetime.now().isoformat()
                },
                "heartbeat_count": 1,
                "missed_heartbeats": 0
            }
            
            sessions["sessions"][session_id] = session_data
            
            # Update coordination metadata
            if session_id not in [s["id"] for s in sessions["sessions"].values() if s["status"] == "active"]:
                sessions["coordination"]["active_sessions"] += 1
            sessions["coordination"]["total_sessions_registered"] += 1
            
            self._log_activity("session_activity", "session_registered", {
                "session": session_id,
                "lane": lane,
                "claude_version": claude_version,
                "hostname": socket.gethostname()
            })
            
            logger.info(f"Registered session {session_id} for lane {lane or 'unassigned'}")
            return session_id
            
        finally:
            self._save_sessions(sessions)
    
    def send_heartbeat(self, session_id: str, activity_data: Dict[str, Any] = None) -> bool:
        """Send heartbeat for a session"""
        sessions = self._load_sessions()
        
        try:
            if session_id not in sessions["sessions"]:
                logger.error(f"Session {session_id} not found")
                return False
            
            session = sessions["sessions"][session_id]
            
            if session["status"] != "active":
                logger.error(f"Session {session_id} is not active (status: {session['status']})")
                return False
            
            # Update heartbeat
            current_time = datetime.now()
            session["last_heartbeat"] = current_time.isoformat()
            session["heartbeat_count"] += 1
            session["missed_heartbeats"] = 0  # Reset missed heartbeat counter
            
            # Update activity data if provided
            if activity_data:
                session["activity"].update(activity_data)
                session["activity"]["last_activity"] = current_time.isoformat()
            
            self._log_activity("session_activity", "heartbeat", {
                "session": session_id,
                "heartbeat_count": session["heartbeat_count"],
                "activity_data": activity_data or {}
            })
            
            return True
            
        finally:
            self._save_sessions(sessions)
    
    def end_session(self, session_id: str, reason: str = "Normal termination") -> bool:
        """End a session gracefully"""
        sessions = self._load_sessions()
        
        try:
            if session_id not in sessions["sessions"]:
                logger.error(f"Session {session_id} not found")
                return False
            
            session = sessions["sessions"][session_id]
            
            # Update session status
            session["status"] = "ended"
            session["ended_at"] = datetime.now().isoformat()
            session["end_reason"] = reason
            
            # Calculate session duration
            start_time = datetime.fromisoformat(session["registered_at"])
            end_time = datetime.fromisoformat(session["ended_at"])
            session["duration_seconds"] = int((end_time - start_time).total_seconds())
            
            # Update coordination metadata
            sessions["coordination"]["active_sessions"] -= 1
            
            self._log_activity("session_activity", "session_ended", {
                "session": session_id,
                "reason": reason,
                "duration_seconds": session["duration_seconds"],
                "tasks_completed": session["activity"]["tasks_completed"]
            })
            
            logger.info(f"Ended session {session_id}: {reason}")
            return True
            
        finally:
            self._save_sessions(sessions)
    
    def get_session_status(self, session_id: str) -> Optional[Dict[str, Any]]:
        """Get detailed status of a specific session"""
        sessions = self._load_sessions()
        
        try:
            if session_id not in sessions["sessions"]:
                return None
            
            session = sessions["sessions"][session_id].copy()
            
            # Calculate time since last heartbeat
            if session.get("last_heartbeat"):
                last_heartbeat = datetime.fromisoformat(session["last_heartbeat"])
                time_since_heartbeat = (datetime.now() - last_heartbeat).total_seconds()
                session["time_since_last_heartbeat_seconds"] = int(time_since_heartbeat)
                
                # Determine if session is likely stale
                timeout_threshold = sessions["metadata"]["session_timeout_minutes"] * 60
                session["likely_stale"] = time_since_heartbeat > timeout_threshold
            
            # Calculate session duration
            if session.get("registered_at"):
                start_time = datetime.fromisoformat(session["registered_at"])
                if session["status"] == "active":
                    duration = (datetime.now() - start_time).total_seconds()
                elif session.get("ended_at"):
                    end_time = datetime.fromisoformat(session["ended_at"])
                    duration = (end_time - start_time).total_seconds()
                else:
                    duration = 0
                
                session["current_duration_seconds"] = int(duration)
            
            return session
            
        finally:
            self._release_lock()
    
    def get_all_sessions(self, status_filter: str = None) -> List[Dict[str, Any]]:
        """Get list of all sessions, optionally filtered by status"""
        sessions = self._load_sessions()
        
        try:
            # Clean up stale sessions first
            self._cleanup_stale_sessions(sessions)
            
            session_list = []
            
            for session_id, session in sessions["sessions"].items():
                if status_filter and session["status"] != status_filter:
                    continue
                
                session_summary = {
                    "id": session_id,
                    "status": session["status"],
                    "registered_at": session["registered_at"],
                    "last_heartbeat": session["last_heartbeat"],
                    "lane": session.get("lane"),
                    "claude_version": session.get("claude_version"),
                    "hostname": session.get("host_info", {}).get("hostname"),
                    "tasks_reserved": session["activity"]["tasks_reserved"],
                    "tasks_completed": session["activity"]["tasks_completed"],
                    "heartbeat_count": session["heartbeat_count"]
                }
                
                # Add duration information
                if session.get("registered_at"):
                    start_time = datetime.fromisoformat(session["registered_at"])
                    if session["status"] == "active":
                        duration = (datetime.now() - start_time).total_seconds()
                    elif session.get("ended_at"):
                        end_time = datetime.fromisoformat(session["ended_at"])
                        duration = (end_time - start_time).total_seconds()
                    else:
                        duration = 0
                    
                    session_summary["duration_seconds"] = int(duration)
                
                session_list.append(session_summary)
            
            # Sort by registration time (newest first)
            session_list.sort(key=lambda x: x["registered_at"], reverse=True)
            
            return session_list
            
        finally:
            self._save_sessions(sessions)
    
    def get_system_status(self) -> Dict[str, Any]:
        """Get overall session coordination system status"""
        sessions = self._load_sessions()
        
        try:
            # Clean up stale sessions first
            self._cleanup_stale_sessions(sessions)
            
            # Count sessions by status
            status_counts = {"active": 0, "ended": 0, "timed_out": 0}
            lane_assignments = {}
            
            for session in sessions["sessions"].values():
                status_counts[session["status"]] = status_counts.get(session["status"], 0) + 1
                
                if session.get("lane"):
                    lane_assignments[session["lane"]] = lane_assignments.get(session["lane"], 0) + 1
            
            # Calculate system health metrics
            total_sessions = len(sessions["sessions"])
            active_sessions = status_counts.get("active", 0)
            timeout_rate = sessions["coordination"]["session_timeouts"] / max(total_sessions, 1)
            
            return {
                "system_health": {
                    "status": "healthy" if active_sessions <= sessions["metadata"]["max_concurrent_sessions"] else "overloaded",
                    "active_sessions": active_sessions,
                    "max_concurrent": sessions["metadata"]["max_concurrent_sessions"],
                    "utilization_percent": int((active_sessions / sessions["metadata"]["max_concurrent_sessions"]) * 100),
                    "timeout_rate": round(timeout_rate, 3)
                },
                "session_counts": status_counts,
                "lane_assignments": lane_assignments,
                "coordination": sessions["coordination"],
                "metadata": sessions["metadata"],
                "last_updated": sessions["last_updated"]
            }
            
        finally:
            self._save_sessions(sessions)
    
    def update_session_activity(self, session_id: str, activity_update: Dict[str, Any]) -> bool:
        """Update session activity metrics"""
        sessions = self._load_sessions()
        
        try:
            if session_id not in sessions["sessions"]:
                logger.error(f"Session {session_id} not found")
                return False
            
            session = sessions["sessions"][session_id]
            
            if session["status"] != "active":
                logger.warning(f"Updating activity for non-active session {session_id}")
            
            # Update activity data
            session["activity"].update(activity_update)
            session["activity"]["last_activity"] = datetime.now().isoformat()
            
            self._log_activity("session_activity", "activity_updated", {
                "session": session_id,
                "activity_update": activity_update
            })
            
            return True
            
        finally:
            self._save_sessions(sessions)
    
    def assign_session_to_lane(self, session_id: str, lane: str) -> bool:
        """Assign a session to a specific lane"""
        sessions = self._load_sessions()
        
        try:
            if session_id not in sessions["sessions"]:
                logger.error(f"Session {session_id} not found")
                return False
            
            session = sessions["sessions"][session_id]
            old_lane = session.get("lane")
            session["lane"] = lane
            
            self._log_activity("session_activity", "lane_assigned", {
                "session": session_id,
                "old_lane": old_lane,
                "new_lane": lane
            })
            
            logger.info(f"Assigned session {session_id} to lane {lane}")
            return True
            
        finally:
            self._save_sessions(sessions)

def main():
    """Command line interface for session coordination"""
    parser = argparse.ArgumentParser(description="BRAIN Session Coordination System")
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Register session
    register_parser = subparsers.add_parser('register', help='Register a new session')
    register_parser.add_argument('--session', help='Session ID (auto-generated if not provided)')
    register_parser.add_argument('--lane', help='Lane assignment')
    register_parser.add_argument('--claude-version', help='Claude version')
    
    # Send heartbeat
    heartbeat_parser = subparsers.add_parser('heartbeat', help='Send session heartbeat')
    heartbeat_parser.add_argument('--session', required=True, help='Session ID')
    heartbeat_parser.add_argument('--tasks-reserved', type=int, help='Number of tasks reserved')
    heartbeat_parser.add_argument('--tasks-completed', type=int, help='Number of tasks completed')
    
    # End session
    end_parser = subparsers.add_parser('end', help='End a session')
    end_parser.add_argument('--session', required=True, help='Session ID')
    end_parser.add_argument('--reason', default='Normal termination', help='End reason')
    
    # Session status
    status_parser = subparsers.add_parser('status', help='Get session status')
    status_parser.add_argument('--session', help='Specific session ID')
    status_parser.add_argument('--all', action='store_true', help='Show all sessions')
    status_parser.add_argument('--active', action='store_true', help='Show only active sessions')
    status_parser.add_argument('--system', action='store_true', help='Show system status')
    status_parser.add_argument('--format', choices=['table', 'json'], default='table', help='Output format')
    
    # Assign to lane
    assign_parser = subparsers.add_parser('assign-lane', help='Assign session to lane')
    assign_parser.add_argument('--session', required=True, help='Session ID')
    assign_parser.add_argument('--lane', required=True, help='Lane ID')
    
    # Update activity
    activity_parser = subparsers.add_parser('update-activity', help='Update session activity')
    activity_parser.add_argument('--session', required=True, help='Session ID')
    activity_parser.add_argument('--tasks-reserved', type=int, help='Number of tasks reserved')
    activity_parser.add_argument('--tasks-completed', type=int, help='Number of tasks completed')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    coordinator = SessionCoordinator()
    
    try:
        if args.command == 'register':
            session_id = coordinator.register_session(
                session_id=args.session,
                lane=args.lane,
                claude_version=args.claude_version
            )
            
            if session_id:
                print(f"✓ Registered session: {session_id}")
                if args.lane:
                    print(f"  Assigned to lane: {args.lane}")
            else:
                print("✗ Failed to register session")
                sys.exit(1)
        
        elif args.command == 'heartbeat':
            activity_data = {}
            if args.tasks_reserved is not None:
                activity_data["tasks_reserved"] = args.tasks_reserved
            if args.tasks_completed is not None:
                activity_data["tasks_completed"] = args.tasks_completed
            
            success = coordinator.send_heartbeat(args.session, activity_data if activity_data else None)
            if success:
                print(f"✓ Heartbeat sent for session {args.session}")
            else:
                print(f"✗ Failed to send heartbeat for session {args.session}")
                sys.exit(1)
        
        elif args.command == 'end':
            success = coordinator.end_session(args.session, args.reason)
            if success:
                print(f"✓ Ended session {args.session}")
            else:
                print(f"✗ Failed to end session {args.session}")
                sys.exit(1)
        
        elif args.command == 'status':
            if args.system:
                status = coordinator.get_system_status()
                if args.format == 'json':
                    print(json.dumps(status, indent=2))
                else:
                    health = status['system_health']
                    print(f"\n=== Session Coordination System Status ===")
                    print(f"System Health: {health['status'].upper()}")
                    print(f"Active Sessions: {health['active_sessions']}/{health['max_concurrent']} ({health['utilization_percent']}%)")
                    print(f"Timeout Rate: {health['timeout_rate']:.1%}")
                    print(f"\nSession Counts: {status['session_counts']}")
                    if status['lane_assignments']:
                        print(f"Lane Assignments: {status['lane_assignments']}")
            
            elif args.session:
                session_status = coordinator.get_session_status(args.session)
                if session_status:
                    if args.format == 'json':
                        print(json.dumps(session_status, indent=2))
                    else:
                        print(f"\n=== Session {args.session} ===")
                        print(f"Status: {session_status['status'].upper()}")
                        print(f"Lane: {session_status.get('lane', 'Unassigned')}")
                        print(f"Registered: {session_status['registered_at']}")
                        print(f"Last Heartbeat: {session_status['last_heartbeat']}")
                        if session_status.get('time_since_last_heartbeat_seconds'):
                            minutes = session_status['time_since_last_heartbeat_seconds'] // 60
                            print(f"Time Since Heartbeat: {minutes}m {session_status['time_since_last_heartbeat_seconds'] % 60}s")
                        print(f"Duration: {session_status.get('current_duration_seconds', 0) // 60}m")
                        print(f"Tasks: {session_status['activity']['tasks_completed']} completed, {session_status['activity']['tasks_reserved']} reserved")
                        print(f"Heartbeats: {session_status['heartbeat_count']}")
                else:
                    print(f"Session {args.session} not found")
                    sys.exit(1)
            
            elif args.all or args.active:
                status_filter = "active" if args.active else None
                sessions = coordinator.get_all_sessions(status_filter)
                
                if args.format == 'json':
                    print(json.dumps(sessions, indent=2))
                else:
                    print(f"\n{'Session ID':<50} {'Status':<10} {'Lane':<8} {'Host':<15} {'Duration':<10} {'Tasks':<10}")
                    print("-" * 110)
                    
                    for session in sessions:
                        duration_min = session.get('duration_seconds', 0) // 60
                        tasks_info = f"{session['tasks_completed']}/{session['tasks_reserved']}"
                        hostname = session.get('hostname', 'Unknown')[:14]
                        
                        print(f"{session['id']:<50} {session['status']:<10} {session.get('lane', 'None'):<8} {hostname:<15} {duration_min}m{'':<7} {tasks_info:<10}")
                    
                    if not sessions:
                        print("No sessions found.")
            else:
                print("Specify --session, --all, --active, or --system for status")
                sys.exit(1)
        
        elif args.command == 'assign-lane':
            success = coordinator.assign_session_to_lane(args.session, args.lane)
            if success:
                print(f"✓ Assigned session {args.session} to lane {args.lane}")
            else:
                print(f"✗ Failed to assign session {args.session} to lane {args.lane}")
                sys.exit(1)
        
        elif args.command == 'update-activity':
            activity_data = {}
            if args.tasks_reserved is not None:
                activity_data["tasks_reserved"] = args.tasks_reserved
            if args.tasks_completed is not None:
                activity_data["tasks_completed"] = args.tasks_completed
            
            success = coordinator.update_session_activity(args.session, activity_data)
            if success:
                print(f"✓ Updated activity for session {args.session}")
            else:
                print(f"✗ Failed to update activity for session {args.session}")
                sys.exit(1)
    
    except Exception as e:
        logger.error(f"Command failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()