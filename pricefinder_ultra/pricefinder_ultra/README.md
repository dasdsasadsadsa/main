# 글로벌 최저가 탐색기 — ULTRA (요청 12개 마켓 포함)

요청하신 **쿠팡·당근·번개·중고나라·11번가·G마켓·다나와·네이버쇼핑·SSG·알리익스프레스·테무·아마존**을
**커넥터 레지스트리**로 모두 묶은 실행 가능한 데모입니다. (샘플 데이터 기반)

## 실행
### 백엔드
```bash
cd backend
python -m venv .venv && source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install fastapi uvicorn pydantic
uvicorn app:app --reload --port 8000
```

### 프론트엔드
```bash
cd ../web
python -m http.server 5500
# 사용자:  http://127.0.0.1:5500
# 관리자:  http://127.0.0.1:5500/admin/index.html
```

## 사용법
- GTIN/UPC/MPN 혹은 **브랜드+모델(+용량/색상)**으로 검색 → **정확매칭(≥0.92)**만 총액 오름차순 노출
- 중고 포함 체크 시 Karrot/번개/중고나라 오퍼도 포함

## 구조 요약
- `backend/connectors/registry.py` — 12개 마켓 커넥터 등록(데모: FilterJSONConnector)
- `backend/data/sample_offers.json` — 3개 대표 SKU에 대한 각 마켓 샘플 오퍼
- `backend/core/matching.py` — GTIN/MPN/CanonicalKey 기반 정확도
- `backend/services/{fx,tax,catalog}.py` — 환율·총액·카탈로그 대조(치수/무게 허용오차)
- `backend/routes/search.py` — /search API
- `web/` — 사용자 UI / `web/admin/` — 커넥터 리스트

## 실제 연동시 (가이드)
- **공식 API 우선** (아마존 SP‑API, 네이버/SSG/11번가 파트너 API 등), 허용 범위내 **HTML schema.org** 파싱 보강
- **정확매칭 강화**: GTIN/EAN/UPC, 브랜드+MPN 강제 일치, 변형요소(용량/색상) 검증
- **총액 엔진**: HS Code/FTA/면세한도/배송수단 반영, 실시간 환율·수수료, 쿠폰/프로모션 적용
- **수동검수**: 애매한 후보는 큐에 적재 후 승인/병합/거절

> 데모는 구조·동작 확인용이며, 실제 크롤링/호출 전 각 마켓 **TOS/robots**를 반드시 준수하세요.
