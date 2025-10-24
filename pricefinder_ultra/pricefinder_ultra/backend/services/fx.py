import json, os
DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "data")
def load_rates():
    return json.load(open(os.path.join(DATA_DIR,"fx_rates.json"),"r",encoding="utf-8"))
def convert(amount: float, frm: str, to: str)->float:
    r = load_rates(); key=f"{frm}_{to}"
    if key not in r: raise ValueError(f"No FX {frm}->{to}")
    return amount*float(r[key])
