from fastapi import APIRouter
from ..connectors.registry import get_connectors
router = APIRouter()
@router.get("/connectors")
def connectors():
    cs = get_connectors()
    return [{"name":c.name,"country":c.country,"supports_used":c.supports_used} for c in cs]
