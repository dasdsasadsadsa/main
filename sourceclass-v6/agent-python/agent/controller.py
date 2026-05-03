"""
SourceClass V6 - Agent Controller
Main orchestration component for the Python Agent.
"""

import os
import sys
import json
from typing import Dict, List, Optional
from datetime import datetime

# Add agent-python to path for imports
agent_python_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if agent_python_dir not in sys.path:
    sys.path.insert(0, agent_python_dir)

from providers.base import LLMProvider
from providers.openai_provider import OpenAIProvider
from prompt.builder import PromptBuilder
from report.markdown import MarkdownReporter
from session.store import SessionStore


class AgentController:
    """
    Main agent controller that orchestrates:
    - Rust core execution
    - Project IR parsing
    - LLM prompt building
    - Provider selection
    - Report generation
    """
    
    def __init__(self):
        self.provider = self._initialize_provider()
        self.prompt_builder = PromptBuilder()
        self.reporter = MarkdownReporter()
        self.session_store = SessionStore()
    
    def _initialize_provider(self) -> LLMProvider:
        """Initialize LLM provider based on environment."""
        provider_type = os.environ.get("SOURCECLASS_PROVIDER", "openai")
        
        if provider_type == "openai":
            return OpenAIProvider()
        # Add more providers as needed
        
        return OpenAIProvider()
    
    def generate_analysis_report(self, project_ir: Dict) -> str:
        """Generate a full analysis report using LLM."""
        # Build analysis prompt
        prompt = self.prompt_builder.build_analysis_prompt(project_ir)
        
        # Call LLM (if available)
        try:
            response = self.provider.complete(prompt)
            return response
        except Exception as e:
            # Fallback to basic report without LLM
            return self.reporter.generate_basic_report(project_ir)
    
    def generate_modify_plan(self, project_ir: Dict, intent: str) -> Dict:
        """Generate a modification plan based on user intent."""
        # Build modification prompt
        prompt = self.prompt_builder.build_modify_prompt(project_ir, intent)
        
        # Call LLM (if available)
        try:
            response = self.provider.complete(prompt)
            # Parse response into structured plan
            return self._parse_modify_response(response, project_ir, intent)
        except Exception as e:
            # Return basic plan without LLM
            return self._generate_basic_modify_plan(project_ir, intent)
    
    def _parse_modify_response(self, response: str, project_ir: Dict, intent: str) -> Dict:
        """Parse LLM response into structured modification plan."""
        relevant_files = self._extract_relevant_files(project_ir, intent)
        
        return {
            "intent": intent,
            "relevant_files": relevant_files[:5],
            "edit_order": relevant_files[:3],
            "warnings": ["Review existing tests before modifying"],
            "safe_first_step": "Read the identified files to understand current implementation",
            "do_not_touch": self._get_dangerous_files(project_ir),
        }
    
    def _generate_basic_modify_plan(self, project_ir: Dict, intent: str) -> Dict:
        """Generate a basic modification plan without LLM."""
        relevant_files = self._extract_relevant_files(project_ir, intent)
        
        return {
            "intent": intent,
            "relevant_files": relevant_files[:5],
            "edit_order": relevant_files[:3],
            "warnings": ["No AI analysis available - review code manually"],
            "safe_first_step": "Read existing implementation before making changes",
            "do_not_touch": self._get_dangerous_files(project_ir),
        }
    
    def _extract_relevant_files(self, project_ir: Dict, intent: str) -> List[str]:
        """Extract relevant files based on intent keywords."""
        intent_lower = intent.lower()
        file_roles = project_ir.get("file_roles", [])
        
        # Simple keyword matching
        role_keywords = {
            "auth": ["Connector", "Interface"],
            "login": ["Connector", "Interface"],
            "database": ["Connector", "Brain"],
            "db": ["Connector", "Brain"],
            "api": ["Interface", "Connector"],
            "route": ["Interface"],
            "ui": ["Interface", "Output"],
            "config": ["Config"],
        }
        
        relevant_roles = set()
        for keyword, roles in role_keywords.items():
            if keyword in intent_lower:
                relevant_roles.update(roles)
        
        if not relevant_roles:
            relevant_roles = {"Brain", "Interface", "Entry"}
        
        # Get files with relevant roles
        relevant_files = [
            fr["file"] for fr in file_roles
            if fr.get("role") in relevant_roles
        ]
        
        # Add entrypoints
        entrypoints = project_ir.get("entrypoints", [])
        relevant_files.extend([ep["file"] for ep in entrypoints])
        
        return list(dict.fromkeys(relevant_files))  # Remove duplicates
    
    def _get_dangerous_files(self, project_ir: Dict) -> List[str]:
        """Get list of dangerous files that should not be modified."""
        risk_flags = project_ir.get("risk_flags", [])
        return [
            rf["file"] for rf in risk_flags
            if rf.get("severity") in ["Critical", "High"]
        ]
