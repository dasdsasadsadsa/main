"""
SourceClass V6 - OpenRouter Provider
"""

import os
import requests
from typing import Optional

from .base import LLMProvider


class OpenRouterProvider(LLMProvider):
    """
    OpenRouter API provider.
    
    Requires OPENROUTER_API_KEY environment variable.
    """
    
    def __init__(self, model: str | None = None):
        self.model = model or os.environ.get("SOURCECLASS_MODEL", "openai/gpt-4o-mini")
        self.api_key = self._validate_api_key("OPENROUTER_API_KEY")
        self.base_url = "https://openrouter.ai/api/v1"
    
    def complete(self, prompt: str, system: str | None = None) -> str:
        """Send completion request to OpenRouter."""
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://github.com/sourceclass",
        }
        
        messages = []
        
        if system:
            messages.append({
                "role": "system",
                "content": system
            })
        else:
            messages.append({
                "role": "system",
                "content": "You are SourceClass V6 Agent, analyzing a codebase using Project IR."
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
        
        response = requests.post(
            f"{self.base_url}/chat/completions",
            headers=headers,
            json=payload,
            timeout=120,
        )
        
        response.raise_for_status()
        data = response.json()
        
        return data["choices"][0]["message"]["content"]
