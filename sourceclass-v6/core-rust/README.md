# SourceClass V6 Core Engine

A separate, independent Rust-powered codebase intelligence engine for SourceClass V6.

## Architecture

This is the Rust core engine that produces machine-readable Project IR (Intermediate Representation).

### Commands

```bash
# Build
cargo build --release

# Scan a project
sourceclass-core scan ./my-project

# Generate full Project IR
sourceclass-core ir ./my-project

# Get context plan only
sourceclass-core context ./my-project

# Get risk flags only
sourceclass-core risk ./my-project
```

### Output

All commands output JSON to stdout. Logs/errors go to stderr.

## Modules

- **scanner**: Filesystem walking with ignore rules
- **analyzer**: Language detection, entrypoint finding, role classification
- **graph**: Structure graph builder
- **risk**: Secret detection and risk flagging
- **context**: Token estimation and context planning
- **ir**: Project IR schema and builder

## Project IR Schema

The engine outputs a standardized Project IR structure:

```json
{
  "project": { ... },
  "files": [...],
  "entrypoints": [...],
  "dependencies": [...],
  "file_roles": [...],
  "risk_flags": [...],
  "edit_zones": [...],
  "structure_graph": { ... },
  "token_estimate": 0,
  "context_plan": [...],
  "summary_stats": { ... }
}
```

## Building

```bash
cd core-rust
cargo build --release
```

The binary will be at `target/release/sourceclass-core`.

## Design Principles

1. **Fast**: Optimized for speed with minimal allocations
2. **Deterministic**: Same input always produces same output
3. **Standalone**: No external dependencies beyond crates.io
4. **JSON-only**: Clean separation from Python agent
5. **Read-only**: Never modifies target codebases
6. **Security-first**: Detects risks before they reach LLM

## Integration

The Python Agent calls this binary as an external process:

```python
subprocess.run(["sourceclass-core", "ir", "./project"], capture_output=True)
```

No runtime coupling - JSON is the only contract.
