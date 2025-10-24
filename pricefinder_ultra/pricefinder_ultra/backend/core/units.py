def mm_to_mm(x): return float(x)
def cm_to_mm(x): return float(x) * 10.0
def inch_to_mm(x): return float(x) * 25.4
def g_to_g(x): return float(x)
def kg_to_g(x): return float(x) * 1000.0
def lb_to_g(x): return float(x) * 453.59237
def to_mm(val, unit):
    unit = (unit or "").lower()
    if unit in ("mm",): return mm_to_mm(val)
    if unit in ("cm",): return cm_to_mm(val)
    if unit in ("in","inch","inches"): return inch_to_mm(val)
    raise ValueError(f"Unsupported length unit: {unit}")
def to_g(val, unit):
    unit = (unit or "").lower()
    if unit in ("g",): return g_to_g(val)
    if unit in ("kg",): return kg_to_g(val)
    if unit in ("lb","lbs","pound","pounds"): return lb_to_g(val)
    raise ValueError(f"Unsupported weight unit: {unit}")
