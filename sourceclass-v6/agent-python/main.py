#!/usr/bin/env python3
"""
SourceClass V6 - Python Agent CLI
A codebase intelligence engine controlled by a Python LLM Agent.
"""

import sys
import os

# Add the agent-python directory to path
agent_python_dir = os.path.dirname(os.path.abspath(__file__))
if agent_python_dir not in sys.path:
    sys.path.insert(0, agent_python_dir)

# Now import from cli.commands
from cli.commands import main

if __name__ == "__main__":
    main()
