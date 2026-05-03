"""
SourceClass V6 - OpenAI Provider
"""

import os
from typing import Optional

from .base import LLMProvider


class OpenAIProvider(LLMProvider):
    """
    OpenAI API provider.
    
    Requires OPENAI_API_KEY environment variable.
    """
    
    def __init__(self, model: str | None = None):
        self.model = model or os.environ.get("SOURCECLASS_MODEL", "gpt-4o-mini")
        self.api_key = self._validate_api_key("OPENAI_API_KEY")
        
        # Lazy import to avoid dependency if not used
        self._client = None
    
    @property
    def client(self):
        if self._client is None:
            try:
                from openai import OpenAI
                self._client = OpenAI(api_key=self.api_key)
            except ImportError:
                raise ImportError(
                    "OpenAI package not installed. Install with: pip install openai"
                )
        return self._client
    
    def complete(self, prompt: str, system: str | None = None) -> str:
        """Send completion request to OpenAI."""
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
        
        response = self.client.chat.completions.create(
            model=self.model,
            messages=messages,
            temperature=0.3,
            max_tokens=4096,
        )
        
        return response.choices[0].message.content
