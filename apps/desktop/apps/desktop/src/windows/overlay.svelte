<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { mmss } from '$lib/format';
  let sec = 0; let title = 'Paus'; let canSnooze = true;
  onMount(async ()=>{
    listen('tick', (e:any)=>{
      const p = e.payload.phase as string;
      title = p==='MicroBreak' ? 'Mikropaus' : 'Paus';
      sec = e.payload.seconds_left;
    });
    listen('force-lock', (_e:any)=>{ title='Fokuslås'; canSnooze=false; });
  });
  async function snooze(){
    if(!canSnooze) return; canSnooze=false; await (window as any).__TAURI_INVOKE__('cmd_snooze');
  }
</script>

<div class="wrap">
  <div class="box">
    <div class="t">{title}</div>
    <div class="time">{mmss(sec)}</div>
    <div class="hint">Sträck på dig, blunda, andas långsamt.</div>
    <div class="row" style="justify-content:center;gap:12px;margin-top:12px">
      <button on:click={snooze} disabled={!canSnooze}>Snooze 1 min</button>
    </div>
  </div>
</div>

<style>
  .wrap{position:fixed;inset:0;background:rgba(0,0,0,0.8);display:flex;align-items:center;justify-content:center;backdrop-filter: blur(4px)}
  .box{background:var(--card);border:1px solid var(--border);padding:40px;border-radius:20px;text-align:center;min-width:420px}
  .t{font-size:20px;opacity:.8}
  .time{font-size:64px;font-weight:800;margin-top:8px}
  .hint{opacity:.8;margin-top:6px}
  button:disabled{opacity:.4}
</style>
