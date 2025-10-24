from fastapi import APIRouter
router = APIRouter()
@router.get("/qa/queue")
def get_queue(): return []
