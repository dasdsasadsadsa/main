from .normalize import normalize_text, tokens, clean_gtin
def canonical_key(attrs):
    parts=[(attrs.get("brand") or "").lower(),
           (attrs.get("model") or "").lower(),
           str(attrs.get("capacity_gb") or ""),
           (attrs.get("color") or "").lower(),
           (attrs.get("mpn") or "").lower()]
    return "|".join([p for p in parts if p])
def exactness_score(query, offer):
    q = normalize_text(query); q_tokens = tokens(q)
    title_tokens = tokens(offer.get("title",""))
    attrs = offer.get("attributes",{})
    gtin_q = clean_gtin(next((t for t in q_tokens if t.isdigit() and len(t) in (8,12,13,14)), ""))
    gtin_o = clean_gtin(str(attrs.get("gtin") or ""))
    if gtin_q and gtin_o and gtin_q == gtin_o: return 0.999
    brand=(attrs.get("brand") or "").lower(); model=(attrs.get("model") or "").lower()
    required = tokens(brand + " " + model)
    if not required or not required.issubset(q_tokens.union(title_tokens)): return 0.0
    variant=[]; 
    if attrs.get("capacity_gb"): variant += [str(attrs["capacity_gb"]),"gb"]
    if attrs.get("color"): variant += list(tokens(attrs["color"]))
    if attrs.get("mpn"): variant += list(tokens(attrs["mpn"]))
    variant_tokens=set(variant)
    coverage=len(q_tokens.intersection(title_tokens))/max(1,len(q_tokens))
    var_cov=len(variant_tokens.intersection(title_tokens.union(q_tokens)))/max(1,len(variant_tokens)) if variant_tokens else 1.0
    ck_tokens=set(canonical_key(attrs).replace("|"," ").split()); ck_cov=len(ck_tokens.intersection(title_tokens))/max(1,len(ck_tokens)) if ck_tokens else 0.0
    return 0.5*coverage + 0.3*var_cov + 0.2*ck_cov
def is_ambiguous(score: float)->bool: return 0.85<=score<0.92
