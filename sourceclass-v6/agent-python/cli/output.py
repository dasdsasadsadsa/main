"""
SourceClass V6 - CLI Output Module
"""

import sys
from typing import Dict, List


def print_analysis_summary(project_ir: Dict):
    """Print analysis summary to terminal."""
    project = project_ir.get("project", {})
    stats = project_ir.get("summary_stats", {})
    entrypoints = project_ir.get("entrypoints", [])
    
    print(f"\nProject: {project.get('name', 'Unknown')}")
    print(f"Dominant Language: {project.get('dominant_language', 'Unknown')}")
    print(f"Files analyzed: {stats.get('analyzed_files', 0)}")
    print(f"Entrypoints found: {len(entrypoints)}")
    print(f"Risks found: {stats.get('risk_count', 0)}")
    print(f"Estimated tokens: {project_ir.get('token_estimate', 0):,}")
    
    if entrypoints:
        print("\nTop entrypoints:")
        for i, ep in enumerate(entrypoints[:3], 1):
            print(f"{i}. {ep.get('file', 'Unknown')}")
    
    # Show brain files from file_roles
    file_roles = project_ir.get("file_roles", [])
    brain_files = [fr for fr in file_roles if fr.get("role") == "Brain"]
    
    if brain_files:
        print("\nTop brain files:")
        for i, bf in enumerate(brain_files[:3], 1):
            print(f"{i}. {bf.get('file', 'Unknown')}")


def print_risk_summary(risks: List[Dict]):
    """Print risk summary to terminal."""
    severity_counts = {"Critical": 0, "High": 0, "Medium": 0, "Low": 0}
    
    for risk in risks:
        severity = risk.get("severity", "Low")
        severity_counts[severity] = severity_counts.get(severity, 0) + 1
    
    print(f"Critical: {severity_counts['Critical']}")
    print(f"High: {severity_counts['High']}")
    print(f"Medium: {severity_counts['Medium']}")
    print(f"Low: {severity_counts['Low']}")
    
    # Show files that should never be sent to LLM
    high_risk = [r for r in risks if r.get("severity") in ["Critical", "High"]]
    
    if high_risk:
        print("\nNever send to LLM:")
        for risk in high_risk[:5]:
            print(f"- {risk.get('file', 'Unknown')}")
    
    print("\nRecommended next action:")
    if any(r.get("risk_type") == "env_file" for r in risks):
        print("- Add .env to ignore rules")
    if any(r.get("risk_type") == "secret_leak" for r in risks):
        print("- Redact provider keys")
    if any(r.get("risk_type") == "unsafe_command" for r in risks):
        print("- Review shell execution files")
    if not high_risk:
        print("- No critical issues found")


def print_modify_plan(plan: Dict):
    """Print modification plan to terminal."""
    print(f"Intent: {plan.get('intent', 'Unknown')}\n")
    
    relevant_files = plan.get("relevant_files", [])
    if relevant_files:
        print("Relevant files:")
        for i, f in enumerate(relevant_files[:5], 1):
            print(f"{i}. {f}")
    
    edit_order = plan.get("edit_order", [])
    if edit_order:
        print("\nModify order:")
        for i, f in enumerate(edit_order, 1):
            print(f"{i}. {f}")
    
    warnings = plan.get("warnings", [])
    if warnings:
        print("\nWarnings:")
        for w in warnings:
            print(f"- {w}")
    
    safe_first_step = plan.get("safe_first_step", "")
    if safe_first_step:
        print(f"\nSafe first step:\n{safe_first_step}")
    
    do_not_touch = plan.get("do_not_touch", [])
    if do_not_touch:
        print("\nDo not touch yet:")
        for f in do_not_touch[:3]:
            print(f"- {f}")
