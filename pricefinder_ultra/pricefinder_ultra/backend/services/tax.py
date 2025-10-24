from typing import Dict
from .fx import convert
def landed_cost(offer: Dict, settle_currency: str, ship_to_country: str) -> Dict:
    ccy = offer["currency"]
    item = convert(offer["base_price"], ccy, settle_currency)
    ship = convert(offer.get("shipping_cost",0.0), ccy, settle_currency)
    if offer["store_country"] == ship_to_country:
        taxes = 0.10*(item+ship); duties=0.0; fx_fee=0.0
    else:
        taxes=0.0; duties=0.08*item; fx_fee=(0.02*(item+ship)) if ccy!=settle_currency else 0.0
    total=item+ship+taxes+duties+fx_fee
    total=(int(total*100))/100.0
    return {"item_price":round(item,2),"shipping":round(ship,2),"taxes":round(taxes,2),
            "duties":round(duties,2),"fx_fees":round(fx_fee,2),"total":round(total,2),
            "currency":settle_currency}
