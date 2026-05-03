"""
SourceClass V6 - LLM Provider Base Class
"""

from abc import ABC, abstractmethod


class LLMProvider(ABC):
    """
    Abstract base class for LLM providers.
    
    All providers must implement:
    - complete(prompt, system) -> str
    """
    
    @abstractmethod
    def complete(self, prompt: str, system: str | None = None) -> str:
        """
        Send a completion request to the LLM.
        
        Args:
            prompt: The user prompt
            system: Optional system prompt
            
        Returns:
            The LLM response text
        """
        raise NotImplementedError
    
    def _validate_api_key(self, env_var: str) -> str:
        """Validate that an API key is set."""
        import os
        key = os.environ.get(env_var)
        if not key:
            raise ValueError(f"API key not found. Set {env_var} environment variable.")
        return key
