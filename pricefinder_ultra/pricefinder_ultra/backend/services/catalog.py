import json, os
from typing import Tuple
from ..core.units import to_mm, to_g
from ..core.matching import canonical_key
DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "data")
def load_catalog(): return json.load(open(os.path.join(DATA_DIR,"brand_catalog.json"),"r",encoding="utf-8"))
def verify_against_catalog(attrs) -> Tuple[bool,str]:
    cat = load_catalog(); ck = canonical_key(attrs)
    if ck not in cat: return False,"No catalog entry"
    entry = cat[ck]
    for k in ("brand","model","mpn"):
        if entry.get(k) and attrs.get(k) and str(entry[k]).lower()!=str(attrs[k]).lower(): return False,f"{k} mismatch"
    if entry.get("capacity_gb") and attrs.get("capacity_gb"):
        if int(entry["capacity_gb"]) != int(attrs["capacity_gb"]): return False,"capacity_gb mismatch"
    if entry.get("color") and attrs.get("color"):
        if entry["color"].lower() != (attrs["color"] or "").lower(): return False,"color mismatch"
    dims = attrs.get("dimensions")
    if dims:
        e = entry.get("dimensions") or {}
        ok = all(abs(to_mm(dims.get(k), dims.get("unit")) - to_mm(e.get(k), e.get("unit"))) <= 2.0 for k in ("w","h","d"))
        if not ok: return False,"dimensions outside tolerance"
    wt = attrs.get("weight")
    if wt:
        e = entry.get("weight") or {}
        w1 = to_g(wt.get("value"), wt.get("unit")); w2 = to_g(e.get("value"), e.get("unit")); tol = 0.05*w2
        if abs(w1 - w2) > tol: return False,"weight outside tolerance"
    return True,"verified"
