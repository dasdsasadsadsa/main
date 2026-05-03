# SourceClass V4

**A mapping-based search engine with LLM-powered code intelligence.**

SourceClass V4 turns any codebase into an explorable, interactive map where every structural element can be instantly understood at human scale.

## Philosophy

> "I don't just browse the code. I walk through a live map. Every time I wonder 'what does this do?', I underline it and the system tells me — in plain English, with its importance to the whole project."

## Core Features

### 1. Mapping Search Engine (GUI/CLI)
- Project-wide map: collapsible tree of directories, files, classes, functions, and key symbols
- Fast text filtering and fuzzy search across all mapped symbols
- Instantaneous navigation with fully pre-built static analysis

### 2. File-Level Structural Summary
When a file is selected:
- Number of classes/functions/lines
- Top-level definitions with signatures
- Imports grouped by external vs internal
- Role badge (Entry, Brain, Utility, Data Model, Configuration, Connector, etc.)
- Mini dependency graph

### 3. Code Classification & Rendering
- Rich, color-coded blocks for each class/object/struct
- Methods/functions nested inside their owning class
- Interfaces/protocols visually distinguished from implementations
- Connection lines between related classes

### 4. Underline → LLM Explanation
Select any symbol or block to get:
1. **MEANING**: Plain-English definition
2. **ROLE**: Responsibility in the current file
3. **CONTRIBUTION**: Impact on the whole project (e.g., "Calls 12 other functions; medium centrality")

### 5. LLM Caching & Smart Context
- Explanations cached by element hash
- Minimized context sent to LLM (sliding window, class signature, compact project skeleton)
- Generation target: under 1.5 seconds with streaming

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interface                            │
│         (Terminal TUI / Canvas GUI / CLI)                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Rust Core Engine                           │
│  - Builds project map (file/folder/class/function graph)    │
│  - Classifies files by role                                  │
│  - Extracts symbol tables                                    │
│  - Maintains fast interactive index in memory               │
│  - Prepares context JSON for LLM                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Python/Node.js Bridge                           │
│  - Handles LLM orchestration                                 │
│  - Prompt construction with smart context                    │
│  - Response streaming                                        │
│  - Explanation caching                                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   LLM Provider                               │
│         (OpenAI / OpenRouter / Local)                        │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### 1. Build the Rust Core Engine

```bash
cd sourceclass-v4/core-rust
cargo build --release
```

### 2. Install Python Dependencies

```bash
cd ../bridge-python
pip install openai ratelimit
```

### 3. Run the TUI

```bash
cd ..
python bridge-python/main.py tui /path/to/your/project
```

Or CLI mode:

```bash
python bridge-python/main.py map /path/to/your/project
```

## Commands

### Map
Generate and display the project map:
```bash
python bridge-python/main.py map ./my-project
```

### Explain
Get LLM explanation for a specific element:
```bash
python bridge-python/main.py explain ./my-project --file src/main.rs --symbol "process_request"
```

### Search
Fuzzy search across all mapped symbols:
```bash
python bridge-python/main.py search ./my-project --query "database"
```

### Summary
Get structural summary of a file:
```bash
python bridge-python/main.py summary ./my-project --file src/handler.rs
```

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

## File Roles

Files are classified into roles:
- **Brain**: Core logic, analyzers, engines
- **Entry**: Execution entry points
- **Interface**: APIs, routes, commands
- **Connector**: External service integrations
- **Config**: Configuration files
- **Utility**: Helper functions
- **Data Model**: Schema, types, models
- **Output**: Report/rendering logic

## Centrality Metrics

Elements are scored for impact:
- **High**: Called by 10+ other elements, writes critical state
- **Medium**: Called by 3-9 elements, moderate impact
- **Low**: Called by 0-2 elements, incidental

## Output Structure

Analysis results are cached in:
```
.sourceclass-v4/
├── cache/
│   └── explanations/<project-hash>/<element-hash>.json
└── maps/
    └── <project-hash>.json
```

## Technical Constraints

- Map building: < 100ms for 1000 files
- File summary: < 20ms
- LLM explanation: < 1.5s (with streaming)
- Memory: Entire map in RAM for instant traversal
- Large file handling: 5000+ lines without lag

## Roadmap

- **V4** (Current): Interactive map + LLM explanations
- **V5**: Modification planning on top of the map
- **V5.5**: Project strategy (build/rewrite/abandon) + value extraction

## License

MIT

---

*"Turn the codebase into a living, interactive map where every piece of code can be instantly understood."*
