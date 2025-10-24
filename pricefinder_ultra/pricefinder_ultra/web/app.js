const API="http://127.0.0.1:8000";
async function doSearch(){
  const query=document.getElementById('query').value.trim();
  const ship_to_country=document.getElementById('country').value;
  const settle_currency=document.getElementById('currency').value;
  const include_used=document.getElementById('includeUsed').checked;
  const results=document.getElementById('results');
  results.innerHTML = `<div class="card"><p>검색 중…</p></div>`;
  try{
    const res=await fetch(`${API}/search`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({query,ship_to_country,settle_currency,include_used})});
    const data=await res.json();
    if(!data.offers || data.offers.length===0){
      const note = data.qa_enqueued? `<div class="muted">정확도가 낮아 ${data.qa_enqueued}건을 수동검수 큐에 보냈습니다(데모).</div>`:"";
      results.innerHTML = `<div class="card"><p>정확히 일치하는 제품이 없습니다. GTIN/UPC/MPN을 포함해보세요.</p>${note}</div>`; return;
    }
    const head = `<div class="card"><b>Canonical Key:</b> ${data.canonical_key} &nbsp; <span class="muted">정산통화: ${data.offers[0].breakdown.currency} · 정렬: 총액 오름차순</span></div>`;
    const rows = data.offers.map(r=>{
      const o=r.offer,b=r.breakdown,ok=r.catalog_verified?'✅':'⚠️';
      return `<div class="offer"><div>
        <div><b>${o.store}</b> <span class="muted">[${o.store_country}] ${o.condition}</span> <span class="muted">${ok} 카탈로그검증</span></div>
        <div>${o.title}</div>
        <div class="breakdown">판매가 ${b.item_price.toLocaleString()} ${b.currency}, 배송비 ${b.shipping.toLocaleString()} ${b.currency}, 세금 ${b.taxes.toLocaleString()} ${b.currency}, 관세 ${b.duties.toLocaleString()} ${b.currency}, 환전수수료 ${b.fx_fees.toLocaleString()} ${b.currency}</div>
      </div><div class="price">총액: ${b.total.toLocaleString()} ${b.currency}</div>
      <div><a class="url" href="${o.url}" target="_blank" rel="noopener">구매 페이지 ↗</a></div></div>`;
    }).join("");
    results.innerHTML = head + `<div class="card">${rows}</div>`;
  }catch(e){
    results.innerHTML = `<div class="card"><p>오류: ${e}</p></div>`;
  }
}
document.getElementById('searchBtn').addEventListener('click', doSearch);
document.getElementById('query').addEventListener('keydown', e=>{ if(e.key==='Enter') doSearch(); });
