"""
SourceClass V6 - Markdown Report Generator
Generates GitBook-style Markdown documentation.
"""

from typing import Dict, List
from datetime import datetime


class MarkdownReporter:
    """
    Generates Markdown reports from Project IR.
    
    Output sections:
    1. Project Identity
    2. Execution Spine
    3. File Role Classification
    4. Project Anatomy Map
    5. Safe Edit Map
    6. Learning Path
    7. Risk and Weakness Analysis
    8. Modification Hotspots
    9. AI Context Plan
    10. Final Recommendation
    """
    
    def generate_basic_report(self, project_ir: Dict) -> str:
        """Generate a basic report without LLM enhancement."""
        lines = []
        
        # Header
        lines.append("# SourceClass V6 Analysis\n")
        lines.append(f"*Generated: {datetime.now().isoformat()}*\n")
        
        # Project Identity
        project = project_ir.get("project", {})
        lines.append("## 1. Project Identity\n")
        lines.append(f"- **Name**: {project.get('name', 'Unknown')}")
        lines.append(f"- **Path**: {project.get('path', 'Unknown')}")
        lines.append(f"- **Dominant Language**: {project.get('dominant_language', 'Unknown')}")
        lines.append(f"- **Languages**: {', '.join(project.get('languages', []))}")
        lines.append(f"- **Frameworks**: {', '.join(project.get('detected_frameworks', [])) or 'None detected'}")
        lines.append("")
        
        # Summary Stats
        stats = project_ir.get("summary_stats", {})
        lines.append("## Summary Statistics\n")
        lines.append(f"- **Total Files**: {stats.get('total_files', 0)}")
        lines.append(f"- **Analyzed Files**: {stats.get('analyzed_files', 0)}")
        lines.append(f"- **Risk Count**: {stats.get('risk_count', 0)}")
        lines.append(f"- **Estimated Tokens**: {project_ir.get('token_estimate', 0):,}")
        lines.append("")
        
        # Execution Spine
        entrypoints = project_ir.get("entrypoints", [])
        lines.append("## 2. Execution Spine\n")
        if entrypoints:
            for i, ep in enumerate(entrypoints, 1):
                lines.append(f"{i}. **{ep.get('file')}** ({ep.get('kind')})")
                lines.append(f"   - Confidence: {ep.get('confidence', 0):.0%}")
                lines.append(f"   - Reason: {ep.get('reason')}")
        else:
            lines.append("*No clear entrypoints detected.*")
        lines.append("")
        
        # File Role Classification
        file_roles = project_ir.get("file_roles", [])
        lines.append("## 3. File Role Classification\n")
        
        roles_by_type = {}
        for fr in file_roles:
            role = fr.get("role", "Unknown")
            if role not in roles_by_type:
                roles_by_type[role] = []
            roles_by_type[role].append(fr)
        
        for role, files in sorted(roles_by_type.items()):
            lines.append(f"### {role} ({len(files)} files)\n")
            for f in files[:10]:  # Limit to 10 per category
                lines.append(f"- `{f.get('file')}`")
                if len(files) > 10:
                    lines.append(f"- *...and {len(files) - 10} more*")
            lines.append("")
        
        # Risk Analysis
        risk_flags = project_ir.get("risk_flags", [])
        lines.append("## 4. Risk Analysis\n")
        
        if risk_flags:
            severity_order = {"Critical": 0, "High": 1, "Medium": 2, "Low": 3}
            sorted_risks = sorted(risk_flags, key=lambda r: severity_order.get(r.get("severity", "Low"), 4))
            
            for risk in sorted_risks[:10]:
                lines.append(f"### {risk.get('severity')} - {risk.get('risk_type')}\n")
                lines.append(f"- **File**: `{risk.get('file')}`")
                lines.append(f"- **Reason**: {risk.get('reason')}")
                lines.append(f"- **Action**: {risk.get('safe_action')}")
                lines.append("")
        else:
            lines.append("*No significant risks detected.*\n")
        
        # Context Plan
        context_plan = project_ir.get("context_plan", [])
        lines.append("## 5. AI Context Plan\n")
        lines.append("Recommended file reading order for LLM analysis:\n")
        
        for item in context_plan[:15]:
            priority_emoji = {
                "MustSend": "🔴",
                "ShouldSend": "🟡",
                "SendIfNeeded": "🟢",
                "IgnoreFirst": "⚪",
                "NeverSend": "🚫",
            }.get(item.get("priority"), "⚪")
            
            lines.append(f"{priority_emoji} {item.get('rank')}. `{item.get('file')}` ({item.get('estimated_tokens', 0)} tokens)")
            lines.append(f"   - Priority: {item.get('priority')}")
            lines.append(f"   - Reason: {item.get('reason')}")
        
        lines.append("")
        
        # Final Recommendation
        lines.append("## 6. Final Recommendation\n")
        lines.append(self._generate_recommendation(project_ir))
        
        return "\n".join(lines)
    
    def _generate_recommendation(self, project_ir: Dict) -> str:
        """Generate final recommendation based on analysis."""
        stats = project_ir.get("summary_stats", {})
        risks = project_ir.get("risk_flags", [])
        entrypoints = project_ir.get("entrypoints", [])
        
        recommendations = []
        
        # Based on risks
        high_risks = [r for r in risks if r.get("severity") in ["Critical", "High"]]
        if high_risks:
            recommendations.append("⚠️ **Address security risks first** - Review flagged files before making any changes.")
        
        # Based on entrypoints
        if entrypoints:
            recommendations.append(f"📍 **Start with entrypoints** - Begin understanding from: {entrypoints[0].get('file')}")
        
        # Based on size
        total_files = stats.get("total_files", 0)
        if total_files > 100:
            recommendations.append("📦 **Large codebase** - Focus on one module at a time.")
        elif total_files > 20:
            recommendations.append("📁 **Medium codebase** - Can be understood in a few focused sessions.")
        else:
            recommendations.append("✅ **Small codebase** - Can be understood in one session.")
        
        if not recommendations:
            return "This project appears straightforward. Start with the entrypoint files and follow the context plan."
        
        return "\n\n".join(recommendations)
