import json, os
from typing import List, Dict, Any
from .base import ProductConnector
DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "data")
class FilterJSONConnector(ProductConnector):
    def __init__(self, store_name: str, country: str = "GLOBAL", supports_used: bool = True):
        self.name = store_name; self.country = country; self.supports_used = supports_used
    def search(self, query: str, max_results: int = 10) -> List[Dict[str, Any]]:
        offers = json.load(open(os.path.join(DATA_DIR,"sample_offers.json"),"r",encoding="utf-8"))
        q = query.lower()
        res = [o for o in offers if o.get("store")==self.name and
               all(w in (o["title"].lower() + " " + (o["attributes"].get("mpn","") or "").lower()
                         + " " + (o["attributes"].get("gtin","") or "")) for w in q.split())]
        return res[:max_results]
