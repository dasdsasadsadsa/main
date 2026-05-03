"""
SourceClass V6 - Session Store
Manages analysis sessions and history.
"""

import json
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional


class SessionStore:
    """
    Manages session data for projects.
    
    Session schema:
    {
        "project_path": "",
        "history": [],
        "focus_files": [],
        "user_intent": "",
        "last_analysis": {},
        "created_at": "",
        "updated_at": ""
    }
    """
    
    def __init__(self, session_dir: str | None = None):
        self.session_dir = Path(session_dir) if session_dir else Path(".sourceclass-v6") / "sessions"
        self.session_dir.mkdir(parents=True, exist_ok=True)
    
    def _get_session_path(self, project_path: str) -> Path:
        """Get session file path for a project."""
        # Use hash of path as filename
        import hashlib
        path_hash = hashlib.sha256(project_path.encode()).hexdigest()[:16]
        return self.session_dir / f"{path_hash}.json"
    
    def load_session(self, project_path: str) -> Optional[Dict]:
        """Load existing session for a project."""
        session_path = self._get_session_path(project_path)
        
        if not session_path.exists():
            return None
        
        try:
            with open(session_path, "r") as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError):
            return None
    
    def save_session(self, project_path: str, session: Dict) -> None:
        """Save session for a project."""
        session_path = self._get_session_path(project_path)
        
        session["updated_at"] = datetime.now().isoformat()
        
        with open(session_path, "w") as f:
            json.dump(session, f, indent=2)
    
    def create_session(self, project_path: str) -> Dict:
        """Create a new session for a project."""
        session = {
            "project_path": os.path.abspath(project_path),
            "history": [],
            "focus_files": [],
            "user_intent": "",
            "last_analysis": {},
            "created_at": datetime.now().isoformat(),
            "updated_at": datetime.now().isoformat(),
        }
        
        self.save_session(project_path, session)
        return session
    
    def add_to_history(self, project_path: str, entry: Dict) -> None:
        """Add an entry to session history."""
        session = self.load_session(project_path)
        
        if not session:
            session = self.create_session(project_path)
        
        entry["timestamp"] = datetime.now().isoformat()
        session["history"].append(entry)
        
        # Keep last 50 entries
        session["history"] = session["history"][-50:]
        
        self.save_session(project_path, session)
    
    def set_focus_files(self, project_path: str, files: List[str]) -> None:
        """Set focus files for a project."""
        session = self.load_session(project_path)
        
        if not session:
            session = self.create_session(project_path)
        
        session["focus_files"] = files
        self.save_session(project_path, session)
    
    def set_user_intent(self, project_path: str, intent: str) -> None:
        """Set user's modification intent."""
        session = self.load_session(project_path)
        
        if not session:
            session = self.create_session(project_path)
        
        session["user_intent"] = intent
        self.save_session(project_path, session)
    
    def update_last_analysis(self, project_path: str, analysis: Dict) -> None:
        """Update the last analysis result."""
        session = self.load_session(project_path)
        
        if not session:
            session = self.create_session(project_path)
        
        session["last_analysis"] = analysis
        self.save_session(project_path, session)
