#!/usr/bin/env python3
"""
SourceClass V4 - Main Entry Point

A mapping-based search engine with LLM-powered code intelligence.
"""

import sys
import os
import argparse
import json
import subprocess
from pathlib import Path

# Add bridge-python to path
bridge_dir = Path(__file__).parent.absolute()
if str(bridge_dir) not in sys.path:
    sys.path.insert(0, str(bridge_dir))


def find_core_binary():
    """Find the sourceclass-v4-core binary."""
    possible_paths = [
        bridge_dir.parent / "core-rust" / "target" / "release" / "sourceclass-v4-core",
        bridge_dir.parent / "core-rust" / "target" / "debug" / "sourceclass-v4-core",
        Path.home() / ".cargo" / "bin" / "sourceclass-v4-core",
        "sourceclass-v4-core",
    ]
    
    for path in possible_paths:
        if isinstance(path, str):
            try:
                result = subprocess.run(["which", path], capture_output=True, text=True)
                if result.returncode == 0:
                    return path.strip()
            except:
                pass
        else:
            if path.exists() and os.access(path, os.X_OK):
                return str(path)
    
    return None


def run_core_command(command: str, args: list) -> dict:
    """Run a core command and parse JSON output."""
    core_binary = find_core_binary()
    
    if not core_binary:
        print("Error: sourceclass-v4-core binary not found.", file=sys.stderr)
        print("Please build the Rust core engine first:", file=sys.stderr)
        print("  cd core-rust && cargo build --release", file=sys.stderr)
        sys.exit(1)
    
    try:
        result = subprocess.run(
            [core_binary, command] + args,
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


def cmd_map(args):
    """Build and display the project map."""
    print(f"Building project map for: {args.path}\n")
    
    project_map = run_core_command("map", [args.path])
    
    # Display summary
    stats = project_map.get("stats", {})
    print(f"Project: {project_map.get('root_path', 'unknown')}")
    print(f"Files: {stats.get('total_files', 0)}")
    print(f"Directories: {stats.get('total_directories', 0)}")
    print(f"Symbols: {stats.get('total_symbols', 0)}")
    print(f"Total Lines: {stats.get('total_lines', 0)}")
    print()
    
    # Show top files by centrality
    files = sorted(project_map.get("files", []), key=lambda f: f.get("centrality_score", 0), reverse=True)
    print("Top 10 Files by Centrality:")
    print("-" * 60)
    for i, f in enumerate(files[:10], 1):
        role = f.get("role", "Unknown").replace("_", " ").title()
        centrality = f.get("centrality_score", 0) * 100
        print(f"{i:2}. [{role:12}] {f['relative_path'][:50]:50} ({centrality:.0f}%)")
    
    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(project_map, f, indent=2)
        print(f"\nMap saved to: {output_path}")
    
    return project_map


def cmd_summary(args):
    """Get structural summary of a file."""
    print(f"File Summary: {args.file}\n")
    
    summary = run_core_command("summary", [args.path, args.file])
    
    file_info = summary.get("file", {})
    print(f"Path: {file_info.get('relative_path', 'unknown')}")
    print(f"Language: {file_info.get('language', 'unknown')}")
    print(f"Lines: {file_info.get('line_count', 0)}")
    print(f"Size: {file_info.get('size_bytes', 0)} bytes")
    print(f"Role: {file_info.get('role', 'Unknown').replace('_', ' ').title()}")
    print(f"Centrality: {file_info.get('centrality_score', 0) * 100:.1f}%")
    print(f"Symbols: {file_info.get('symbol_count', 0)}")
    print()
    
    print("Summary:", summary.get("one_line_summary", "N/A"))
    print()
    
    # Show symbols
    symbols = summary.get("symbols", [])
    if symbols:
        print(f"Symbols ({len(symbols)}):")
        print("-" * 60)
        for s in symbols[:20]:
            kind = s.get("kind", "unknown").replace("_", " ").title()
            print(f"  [{kind:12}] {s['name'][:40]}")
        if len(symbols) > 20:
            print(f"  ... and {len(symbols) - 20} more")
    
    # Show imports
    imports = summary.get("imports", [])
    if imports:
        print(f"\nImports ({len(imports)}):")
        external = [i for i in imports if i.get("is_external")]
        internal = [i for i in imports if not i.get("is_external")]
        if external:
            print(f"  External: {len(external)}")
        if internal:
            print(f"  Internal: {len(internal)}")
    
    return summary


def cmd_symbols(args):
    """Extract symbols from a file."""
    symbols = run_core_command("symbols", [args.path, args.file])
    
    print(f"Symbols in {args.file}:")
    print("-" * 60)
    
    for s in symbols:
        kind = s.get("kind", "unknown").replace("_", " ").title()
        visibility = s.get("visibility", "public").title()
        line_range = f"L{s.get('start_line', 0)}-{s.get('end_line', 0)}"
        print(f"[{kind:12}] [{visibility:8}] {line_range:8} {s['name']}")
    
    return symbols


def cmd_context(args):
    """Get context plan for LLM usage."""
    context_plan = run_core_command("context", [args.path])
    
    print("Context Plan (files prioritized for LLM):")
    print("-" * 80)
    print(f"{'Rank':>4} {'Priority':12} {'File':50} {'Tokens':>8}")
    print("-" * 80)
    
    for item in context_plan[:30]:
        priority_emoji = {
            "MustSend": "🔴",
            "ShouldSend": "🟡",
            "SendIfNeeded": "🟢",
            "IgnoreFirst": "⚪",
        }.get(item["priority"], "⚪")
        
        print(f"{item['rank']:>4} {priority_emoji} {item['priority']:11} {item['file'][:48]:48} {item['estimated_tokens']:>8}")
    
    if len(context_plan) > 30:
        print(f"... and {len(context_plan) - 30} more files")
    
    return context_plan


def cmd_explain(args):
    """Generate explanation context for an element."""
    explain_context = run_core_command("explain", [args.path, args.file, args.element])
    
    element = explain_context.get("element", {})
    centrality = explain_context.get("centrality", {})
    
    print(f"Element: {element.get('name', 'unknown')}")
    print(f"Kind: {element.get('kind', 'unknown').replace('_', ' ').title()}")
    print(f"File: {element.get('file_path', 'unknown')}")
    print(f"Lines: {element.get('start_line', 0)}-{element.get('end_line', 0)}")
    print()
    
    print("Centrality Analysis:")
    print(f"  Level: {centrality.get('level', 'unknown').title()}")
    print(f"  Score: {centrality.get('score', 0) * 100:.1f}%")
    print(f"  Incoming edges: {centrality.get('incoming_edges', 0)}")
    print(f"  Outgoing edges: {centrality.get('outgoing_edges', 0)}")
    print(f"  Description: {centrality.get('description', 'N/A')}")
    print()
    
    print("Signature:")
    print(f"  {element.get('signature', 'N/A')}")
    print()
    
    print("Project Skeleton (for LLM context):")
    skeleton = explain_context.get("project_skeleton", {})
    print(f"  Root: {skeleton.get('root_name', 'unknown')}")
    print(f"  Top-level dirs: {len(skeleton.get('top_level_dirs', []))}")
    print(f"  Key files: {len(skeleton.get('key_files', []))}")
    print(f"  Symbol index: {len(skeleton.get('symbol_index', []))}")
    
    # If LLM provider is configured, generate actual explanation
    if os.environ.get("OPENAI_API_KEY") or os.environ.get("SOURCECLASS_LOCAL_URL"):
        print("\nGenerating LLM explanation...")
        from llm.explainer import generate_explanation
        
        explanation = generate_explanation(explain_context)
        print("\n" + "=" * 60)
        print("LLM EXPLANATION")
        print("=" * 60)
        print(explanation)
    else:
        print("\n[LLM explanation not generated - set OPENAI_API_KEY or SOURCECLASS_LOCAL_URL]")
    
    return explain_context


def main():
    parser = argparse.ArgumentParser(
        prog="sourceclass-v4",
        description="SourceClass V4 - Mapping Search Engine with LLM Intelligence"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Commands")
    
    # map command
    map_parser = subparsers.add_parser("map", help="Build and display project map")
    map_parser.add_argument("path", help="Path to the codebase")
    map_parser.add_argument("--output", "-o", help="Save map to JSON file")
    map_parser.set_defaults(func=cmd_map)
    
    # summary command
    summary_parser = subparsers.add_parser("summary", help="Get file structural summary")
    summary_parser.add_argument("path", help="Path to the codebase")
    summary_parser.add_argument("--file", "-f", required=True, help="File to summarize")
    summary_parser.set_defaults(func=cmd_summary)
    
    # symbols command
    symbols_parser = subparsers.add_parser("symbols", help="Extract symbols from file")
    symbols_parser.add_argument("path", help="Path to the codebase")
    symbols_parser.add_argument("--file", "-f", required=True, help="File to extract from")
    symbols_parser.set_defaults(func=cmd_symbols)
    
    # context command
    context_parser = subparsers.add_parser("context", help="Get context plan for LLM")
    context_parser.add_argument("path", help="Path to the codebase")
    context_parser.set_defaults(func=cmd_context)
    
    # explain command
    explain_parser = subparsers.add_parser("explain", help="Explain a code element")
    explain_parser.add_argument("path", help="Path to the codebase")
    explain_parser.add_argument("--file", "-f", required=True, help="File containing element")
    explain_parser.add_argument("--element", "-e", required=True, help="Element name to explain")
    explain_parser.set_defaults(func=cmd_explain)
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    args.func(args)


if __name__ == "__main__":
    main()
