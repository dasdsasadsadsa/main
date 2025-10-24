import re
def normalize_text(s: str) -> str:
    if not s: return ""
    s = s.lower()
    s = re.sub(r"[/()\[\]\-_,.]+", " ", s)
    s = re.sub(r"\s+", " ", s).strip()
    return s
def tokens(s: str) -> set: return set(normalize_text(s).split())
def clean_gtin(gtin: str) -> str:
    if not gtin: return ""
    return re.sub(r"\D+", "", gtin)
