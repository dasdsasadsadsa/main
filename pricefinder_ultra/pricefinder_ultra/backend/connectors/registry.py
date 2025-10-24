from typing import List
from .base import ProductConnector
from .filter_json import FilterJSONConnector
def get_connectors() -> List[ProductConnector]:
    return [
        # KR retail
        FilterJSONConnector("Coupang", country="KR", supports_used=False),
        FilterJSONConnector("11st", country="KR", supports_used=False),
        FilterJSONConnector("Gmarket", country="KR", supports_used=False),
        FilterJSONConnector("Danawa", country="KR", supports_used=False),
        FilterJSONConnector("NaverShopping", country="KR", supports_used=False),
        FilterJSONConnector("SSG", country="KR", supports_used=False),
        # Cross-border
        FilterJSONConnector("AliExpress", country="CN", supports_used=False),
        FilterJSONConnector("Temu", country="CN", supports_used=False),
        FilterJSONConnector("Amazon", country="US", supports_used=False),
        # Used KR
        FilterJSONConnector("Karrot", country="KR", supports_used=True),
        FilterJSONConnector("Bunjang", country="KR", supports_used=True),
        FilterJSONConnector("Joonggonara", country="KR", supports_used=True),
    ]
