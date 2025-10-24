from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from .routes import search as search_routes
from .routes import connectors as conn_routes
from .routes import qa as qa_routes
app = FastAPI(title="Global Lowest-Price Finder — ULTRA", version="1.0.0")
app.add_middleware(CORSMiddleware, allow_origins=["*"], allow_credentials=True, allow_methods=["*"], allow_headers=["*"])
@app.get("/health")
def health(): return {"status":"ok"}
app.include_router(search_routes.router, tags=["search"])
app.include_router(conn_routes.router, tags=["connectors"])
app.include_router(qa_routes.router, tags=["qa"])
