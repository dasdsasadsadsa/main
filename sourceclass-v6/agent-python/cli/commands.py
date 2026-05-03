"""
SourceClass V6 - CLI Commands Module
"""

import argparse
import sys
import os
import json
import subprocess
from pathlib import Path

# Fix relative imports by using absolute imports when run as script
agent_python_dir = os.path.dirname(os.path.abspath(__file__))
if agent_python_dir not in sys.path:
    sys.path.insert(0, agent_python_dir)

from output import print_analysis_summary, print_risk_summary, print_modify_plan
from agent.controller import AgentController
from session.store import SessionStore


def find_core_binary():
    """Find the sourceclass-core binary."""
    # Try common locations
    possible_paths = [
        Path(__file__).parent.parent.parent / "core-rust" / "target" / "release" / "sourceclass-core",
        Path(__file__).parent.parent.parent / "core-rust" / "target" / "debug" / "sourceclass-core",
        Path.home() / ".cargo" / "bin" / "sourceclass-core",
        "sourceclass-core",  # In PATH
    ]
    
    for path in possible_paths:
        if isinstance(path, str):
            # Check if it's in PATH
            try:
                result = subprocess.run(
                    ["which", path],
                    capture_output=True,
                    text=True
                )
                if result.returncode == 0:
                    return path.strip()
            except:
                pass
        else:
            if path.exists() and os.access(path, os.X_OK):
                return str(path)
    
    return None


def run_core_command(command: str, path: str) -> dict:
    """Run a sourceclass-core command and parse JSON output."""
    core_binary = find_core_binary()
    
    if not core_binary:
        print("Error: sourceclass-core binary not found.", file=sys.stderr)
        print("Please build the Rust core engine first:", file=sys.stderr)
        print("  cd core-rust && cargo build --release", file=sys.stderr)
        sys.exit(1)
    
    try:
        result = subprocess.run(
            [core_binary, command, path],
            capture_output=True,
            text=True,
            timeout=120
        )
        
        if result.returncode != 0:
            print(f"Error running core command: {result.stderr}", file=sys.stderr)
            sys.exit(1)
        
        return json.loads(result.stdout)
    
    except subprocess.TimeoutExpired:
        print("Error: Core command timed out.", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error parsing core output: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


def cmd_analyze(args):
    """Analyze a codebase and generate a report."""
    print(f"SourceClass V6 Analysis\n")
    print(f"Project: {os.path.basename(os.path.abspath(args.path))}")
    
    # Get Project IR from Rust core
    project_ir = run_core_command("ir", args.path)
    
    # Print summary
    print_analysis_summary(project_ir)
    
    # If AI mode is enabled, generate detailed report
    if not args.no_ai:
        controller = AgentController()
        report = controller.generate_analysis_report(project_ir)
        
        # Save report
        output_dir = Path(".sourceclass-v6") / "reports"
        output_dir.mkdir(parents=True, exist_ok=True)
        report_path = output_dir / "analysis.md"
        
        with open(report_path, "w") as f:
            f.write(report)
        
        print(f"\nReport saved:\n{report_path}")
    
    return project_ir


def cmd_risk(args):
    """Generate a risk report."""
    print(f"SourceClass V6 Risk Report\n")
    
    # Get risk data from Rust core
    risks = run_core_command("risk", args.path)
    
    # Print risk summary
    print_risk_summary(risks)
    
    return risks


def cmd_modify(args):
    """Generate a modification plan."""
    print(f"SourceClass V6 Modification Plan\n")
    print(f"Intent: {args.intent}\n")
    
    # Get Project IR from Rust core
    project_ir = run_core_command("ir", args.path)
    
    # Generate modification plan
    controller = AgentController()
    plan = controller.generate_modify_plan(project_ir, args.intent)
    
    # Print plan
    print_modify_plan(plan)
    
    return plan


def cmd_context(args):
    """Get context plan for LLM usage."""
    context_plan = run_core_command("context", args.path)
    
    print(json.dumps(context_plan, indent=2))
    
    return context_plan


def cmd_scan(args):
    """Scan files without analysis."""
    files = run_core_command("scan", args.path)
    
    print(json.dumps(files, indent=2))
    
    return files


def main():
    parser = argparse.ArgumentParser(
        prog="sourceclass",
        description="SourceClass V6 - Codebase Intelligence Engine"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Commands")
    
    # analyze command
    analyze_parser = subparsers.add_parser("analyze", help="Analyze a codebase")
    analyze_parser.add_argument("path", help="Path to the codebase")
    analyze_parser.add_argument("--no-ai", action="store_true", help="Skip AI-generated report")
    analyze_parser.set_defaults(func=cmd_analyze)
    
    # risk command
    risk_parser = subparsers.add_parser("risk", help="Generate risk report")
    risk_parser.add_argument("path", help="Path to the codebase")
    risk_parser.set_defaults(func=cmd_risk)
    
    # modify command
    modify_parser = subparsers.add_parser("modify", help="Generate modification plan")
    modify_parser.add_argument("path", help="Path to the codebase")
    modify_parser.add_argument("--intent", required=True, help="Modification intent")
    modify_parser.set_defaults(func=cmd_modify)
    
    # context command
    context_parser = subparsers.add_parser("context", help="Get context plan")
    context_parser.add_argument("path", help="Path to the codebase")
    context_parser.set_defaults(func=cmd_context)
    
    # scan command
    scan_parser = subparsers.add_parser("scan", help="Scan files")
    scan_parser.add_argument("path", help="Path to the codebase")
    scan_parser.set_defaults(func=cmd_scan)
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    args.func(args)
