import App from './App.svelte';
import './styles.css';

new App({ target: document.getElementById('app')! });

// IDLE detection (frontend-only baseline)
let lastActive = Date.now();
['mousemove','mousedown','keydown','wheel','touchstart','touchmove'].forEach(evt=>{
  window.addEventListener(evt, ()=>{ lastActive = Date.now(); }, { passive: true });
});
setInterval(()=>{
  // Future: report to backend if needed
}, 30000);
