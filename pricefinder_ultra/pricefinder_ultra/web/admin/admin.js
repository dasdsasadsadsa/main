const API="http://127.0.0.1:8000";
async function load(){
  const res=await fetch(`${API}/connectors`); const data=await res.json();
  const el=document.getElementById('list');
  el.innerHTML = data.map(c=>`<div class="offer"><div><b>${c.name}</b></div><div>${c.country}</div><div>${c.supports_used?'중고 지원':'신품전용'}</div></div>`).join("");
}
document.getElementById('refresh').addEventListener('click', load); load();
