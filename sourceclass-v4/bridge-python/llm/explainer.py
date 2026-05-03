"""
LLM Explainer Module

Generates natural language explanations for code elements.
"""

import os
import json
import hashlib
from pathlib import Path
from typing import Optional, Dict, Any


def get_cache_path(element_hash: str) -> Path:
    """Get the cache path for an element explanation."""
    cache_dir = Path(".sourceclass-v4") / "cache" / "explanations"
    cache_dir.mkdir(parents=True, exist_ok=True)
    return cache_dir / f"{element_hash}.json"


def compute_element_hash(element: Dict[str, Any]) -> str:
    """Compute a hash for caching element explanations."""
    content = json.dumps({
        "name": element.get("name"),
        "kind": element.get("kind"),
        "signature": element.get("signature"),
        "file_path": element.get("file_path"),
    }, sort_keys=True)
    return hashlib.sha256(content.encode()).hexdigest()[:16]


def get_cached_explanation(element: Dict[str, Any]) -> Optional[str]:
    """Get cached explanation if available."""
    element_hash = compute_element_hash(element)
    cache_path = get_cache_path(element_hash)
    
    if cache_path.exists():
        try:
            with open(cache_path) as f:
                data = json.load(f)
                return data.get("explanation")
        except:
            pass
    
    return None


def cache_explanation(element: Dict[str, Any], explanation: str):
    """Cache an explanation for future use."""
    element_hash = compute_element_hash(element)
    cache_path = get_cache_path(element_hash)
    
    with open(cache_path, "w") as f:
        json.dump({
            "element": element,
            "explanation": explanation,
        }, f, indent=2)


def build_prompt(explain_context: Dict[str, Any]) -> str:
    """Build the LLM prompt for explaining a code element."""
    element = explain_context.get("element", {})
    centrality = explain_context.get("centrality", {})
    skeleton = explain_context.get("project_skeleton", {})
    file_content = explain_context.get("file_content", "")
    
    # Get surrounding context (sliding window around the element)
    start_line = element.get("start_line", 0)
    end_line = element.get("end_line", 0)
    lines = file_content.split("\n")
    
    # Include some context before and after
    context_start = max(0, start_line - 10)
    context_end = min(len(lines), end_line + 10)
    surrounding_context = "\n".join(lines[context_start:context_end])
    
    # Build compact project skeleton
    skeleton_text = f"Project: {skeleton.get('root_name', 'unknown')}\n"
    skeleton_text += f"Key files ({len(skeleton.get('key_files', []))}):\n"
    for kf in skeleton.get("key_files", [])[:10]:
        skeleton_text += f"  - [{kf['role']}] {kf['path']} (centrality: {kf['centrality']*100:.0f}%)\n"
    
    prompt = f"""You are analyzing a code element in a project. Provide a concise, practical explanation.

PROJECT CONTEXT:
{skeleton_text}

ELEMENT TO EXPLAIN:
- Name: {element.get('name', 'unknown')}
- Kind: {element.get('kind', 'unknown')}
- File: {element.get('file_path', 'unknown')}
- Lines: {start_line}-{end_line}
- Signature: {element.get('signature', 'N/A')}

CENTRALITY ANALYSIS:
- Level: {centrality.get('level', 'unknown')}
- Incoming references: {centrality.get('incoming_edges', 0)}
- Outgoing calls: {centrality.get('outgoing_edges', 0)}
- Description: {centrality.get('description', 'N/A')}

SURROUNDING CODE CONTEXT:
```
{surrounding_context}
```

Provide your explanation in this exact format:

MEANING:
[Plain English definition of what this element is and does]

ROLE:
[What responsibility this element fulfills in the current file]

CONTRIBUTION:
[Quantitative/qualitative measure of its impact on the whole project]

CONFIDENCE: [High/Medium/Low]
[Brief note about what evidence supports this analysis]

Keep each section to 2-3 sentences. Be specific and grounded in the actual code."""

    return prompt


def call_llm(prompt: str) -> str:
    """Call the configured LLM provider."""
    # Check for local model first
    local_url = os.environ.get("SOURCECLASS_LOCAL_URL")
    if local_url:
        return call_local_model(local_url, prompt)
    
    # Fall back to OpenAI
    api_key = os.environ.get("OPENAI_API_KEY")
    if api_key:
        return call_openai(api_key, prompt)
    
    raise RuntimeError("No LLM provider configured. Set OPENAI_API_KEY or SOURCECLASS_LOCAL_URL")


def call_openai(api_key: str, prompt: str) -> str:
    """Call OpenAI API."""
    try:
        import openai
        from openai import OpenAI
        
        client = OpenAI(api_key=api_key)
        
        model = os.environ.get("SOURCECLASS_MODEL", "gpt-4o-mini")
        
        response = client.chat.completions.create(
            model=model,
            messages=[
                {"role": "system", "content": "You are a code analysis expert who explains code clearly and concisely."},
                {"role": "user", "content": prompt}
            ],
            max_tokens=500,
            temperature=0.3,
        )
        
        return response.choices[0].message.content
    
    except ImportError:
        raise RuntimeError("OpenAI package not installed. Run: pip install openai")
    except Exception as e:
        return f"[Error calling OpenAI: {e}]"


def call_local_model(base_url: str, prompt: str) -> str:
    """Call a local model (e.g., Ollama, LM Studio)."""
    try:
        import requests
        
        model = os.environ.get("SOURCECLASS_MODEL", "llama3.1:8b")
        
        response = requests.post(
            f"{base_url}/v1/chat/completions",
            json={
                "model": model,
                "messages": [
                    {"role": "system", "content": "You are a code analysis expert who explains code clearly and concisely."},
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 500,
                "temperature": 0.3,
            },
            timeout=30,
        )
        
        response.raise_for_status()
        return response.json()["choices"][0]["message"]["content"]
    
    except ImportError:
        raise RuntimeError("requests package not installed. Run: pip install requests")
    except Exception as e:
        return f"[Error calling local model: {e}]"


def generate_explanation(explain_context: Dict[str, Any]) -> str:
    """Generate an explanation for a code element."""
    element = explain_context.get("element", {})
    
    # Check cache first
    cached = get_cached_explanation(element)
    if cached:
        return f"[CACHED]\n\n{cached}"
    
    # Build prompt
    prompt = build_prompt(explain_context)
    
    # Call LLM
    explanation = call_llm(prompt)
    
    # Cache the result
    cache_explanation(element, explanation)
    
    return explanation
