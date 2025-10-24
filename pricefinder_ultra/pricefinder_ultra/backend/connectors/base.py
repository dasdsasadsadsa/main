from typing import List, Dict, Any
class ProductConnector:
    name = "BASE"; country="XX"; supports_used=False
    def search(self, query: str, max_results: int = 10) -> List[Dict[str, Any]]:
        raise NotImplementedError
