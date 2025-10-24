from fastapi import APIRouter
from pydantic import BaseModel
from typing import List, Dict, Any
from ..connectors.registry import get_connectors
from ..core.matching import exactness_score, is_ambiguous, canonical_key
from ..services.tax import landed_cost
from ..services.catalog import verify_against_catalog
import os, json
router = APIRouter()
DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "data")
class SearchRequest(BaseModel):
    query: str
    ship_to_country: str = "KR"
    settle_currency: str = "KRW"
    include_used: bool = False
    max_results_per_store: int = 10
@router.post("/search")
def search(req: SearchRequest):
    all_offers: List[Dict[str, Any]] = []; ambiguous: List[Dict[str, Any]] = []
    for conn in get_connectors():
        offers = conn.search(req.query, max_results=req.max_results_per_store)
        for o in offers:
            if not req.include_used and o.get("condition")!="new": continue
            score = exactness_score(req.query, o); o["exactness_score"]=score
            if score >= 0.92: all_offers.append(o)
            elif is_ambiguous(score): ambiguous.append(o)
    if not all_offers:
        return {"normalized_query": req.query, "canonical_key": None, "offers": [], "qa_enqueued": len(ambiguous)}
    from collections import Counter
    ck = Counter([o.get("canonical_key") for o in all_offers if o.get("canonical_key")]).most_common(1)[0][0]
    exacts = [o for o in all_offers if o.get("canonical_key")==ck]
    ranked = []
    for o in exacts:
        breakdown = landed_cost(o, req.settle_currency, req.ship_to_country)
        verified, note = verify_against_catalog(o.get("attributes", {}))
        ranked.append({"offer":o,"breakdown":breakdown,"exactness_score":o["exactness_score"],
                       "catalog_verified":verified,"catalog_note":note})
    ranked.sort(key=lambda r:(r["breakdown"]["total"], -r["offer"]["seller_rating"], r["offer"]["shipping_days_max"]))
    return {"normalized_query":req.query, "canonical_key":ck, "offers":ranked}
