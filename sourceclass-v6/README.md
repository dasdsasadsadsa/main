# SourceClass V6

**A Rust-powered codebase intelligence engine controlled by a Python LLM Agent.**

SourceClass V6 turns unknown code into structure, structure into decisions, and decisions into safe action.

## Philosophy

AI can write code, but many people still cannot understand code. SourceClass exists to close the gap between code generation and code comprehension.

## Quick Start

### 1. Build the Rust Core Engine

```bash
cd sourceclass-v6/core-rust
cargo build --release
```

### 2. Install Python Dependencies (Optional - for AI features)

```bash
cd ../agent-python
pip install openai requests
```

### 3. Run Analysis

```bash
cd ..
python agent-python/main.py analyze /path/to/your/project
```

Or without AI features:

```bash
python agent-python/main.py analyze /path/to/your/project --no-ai
```

## Commands

### Analyze

Full project analysis with Markdown report generation:

```bash
python agent-python/main.py analyze ./my-project
```

### Risk Report

Security and risk analysis:

```bash
python agent-python/main.py risk ./my-project
```

### Modification Plan

Get a safe modification plan for your intent:

```bash
python agent-python/main.py modify ./my-project --intent "add login system"
```

### Context Plan

Get token-aware context planning for LLM usage:

```bash
python agent-python/main.py context ./my-project
```

### Raw Scan

Get raw file metadata:

```bash
python agent-python/main.py scan ./my-project
```

## Architecture

SourceClass V6 is a two-layer system:

### Layer 1: Rust Core Engine (`core-rust/`)

- Fast, deterministic filesystem scanning
- Language detection
- Entrypoint identification
- File role classification
- Risk detection
- Token estimation
- Context planning
- **Output**: Machine-readable Project IR (JSON)

### Layer 2: Python Agent (`agent-python/`)

- CLI interface
- LLM provider integration
- Prompt building
- Report generation
- Session management
- Plugin system
- **Output**: Human-readable reports and recommendations

## Environment Variables

```bash
# LLM Provider selection
export SOURCECLASS_PROVIDER=openai  # or openrouter, local

# API Keys
export OPENAI_API_KEY=sk-...
export OPENROUTER_API_KEY=...

# Model selection
export SOURCECLASS_MODEL=gpt-4o-mini

# Local model URL (for local provider)
export SOURCECLASS_LOCAL_URL=http://localhost:11434
```

## Output Structure

Analysis results are saved to:

```
.sourceclass-v6/
├── reports/
│   └── analysis.md
└── sessions/
    └── <project-hash>.json
```

## Project IR Schema

The Rust core outputs a standardized JSON structure:

```json
{
  "project": {
    "name": "my-project",
    "dominant_language": "Python",
    "languages": ["Python", "JavaScript"],
    "frameworks": ["FastAPI", "React"]
  },
  "files": [...],
  "entrypoints": [...],
  "file_roles": [...],
  "risk_flags": [...],
  "context_plan": [...],
  "summary_stats": {...}
}
```

## File Roles

Files are classified into roles:

- **Brain**: Core logic, analyzers, engines
- **Entry**: Execution entry points
- **Interface**: APIs, routes, commands
- **Connector**: External service integrations
- **Config**: Configuration files
- **Utility**: Helper functions
- **Output**: Report/rendering logic
- **Dangerous**: Security-sensitive files

## Context Priorities

Files are prioritized for LLM context:

- 🔴 **MustSend**: Essential for understanding
- 🟡 **ShouldSend**: Important context
- 🟢 **SendIfNeeded**: Reference when needed
- ⚪ **IgnoreFirst**: Low priority
- 🚫 **NeverSend**: Security risks

## Security

SourceClass V6 is designed with security in mind:

- Read-only by default
- Detects .env files and secrets
- Flags risky files before LLM processing
- Never sends dangerous files to LLM by default
- Redacts sensitive patterns in output

## Roadmap

- **V6** (Current): Core engine + basic agent
- **V7**: Plugin system + session memory
- **V8**: Advanced modification planning
- **V9**: Visualization + HTML reports
- **V10**: Developer OS mode

## License

MIT

---

*"I understand this codebase, and I know exactly what to do next."*
