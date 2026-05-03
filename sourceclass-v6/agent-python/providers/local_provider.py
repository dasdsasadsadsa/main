"""
SourceClass V6 - Local Provider (Ollama, etc.)
"""

import os
import requests
from typing import Optional

from .base import LLMProvider


class LocalProvider(LLMProvider):
    """
    Local LLM provider (Ollama, LM Studio, etc.).
    
    Requires SOURCECLASS_LOCAL_URL environment variable.
    Default: http://localhost:11434
    """
    
    def __init__(self, model: str | None = None, base_url: str | None = None):
        self.model = model or os.environ.get("SOURCECLASS_MODEL", "llama2")
        self.base_url = base_url or os.environ.get("SOURCECLASS_LOCAL_URL", "http://localhost:11434")
    
    def complete(self, prompt: str, system: str | None = None) -> str:
        """Send completion request to local LLM."""
        
        # Try Ollama format first
        try:
            return self._ollama_complete(prompt, system)
        except Exception:
            # Fallback to generic format
            return self._generic_complete(prompt, system)
    
    def _ollama_complete(self, prompt: str, system: str | None = None) -> str:
        """Ollama-specific completion."""
        url = f"{self.base_url}/api/generate"
        
        full_prompt = prompt
        if system:
            full_prompt = f"{system}\n\n{prompt}"
        
        payload = {
            "model": self.model,
            "prompt": full_prompt,
            "stream": False,
        }
        
        response = requests.post(url, json=payload, timeout=300)
        response.raise_for_status()
        data = response.json()
        
        return data.get("response", "")
    
    def _generic_complete(self, prompt: str, system: str | None = None) -> str:
        """Generic OpenAI-compatible completion."""
        url = f"{self.base_url}/v1/chat/completions"
        
        headers = {"Content-Type": "application/json"}
        
        messages = []
        
        if system:
            messages.append({
                "role": "system",
                "content": system
            })
        
        messages.append({
            "role": "user",
            "content": prompt
        })
        
        payload = {
            "model": self.model,
            "messages": messages,
            "temperature": 0.3,
            "max_tokens": 4096,
        }
        
        response = requests.post(url, headers=headers, json=payload, timeout=120)
        response.raise_for_status()
        data = response.json()
        
        return data["choices"][0]["message"]["content"]
