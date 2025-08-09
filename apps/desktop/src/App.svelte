<script lang="ts">
  import { onMount } from 'svelte';
  import { applyTheme } from '$lib/theme';
  import { mmss } from '$lib/format';
  import { settings, loadSettings, saveSettings, updateHotkeys, phase, secondsLeft, start, stop, logPause } from '$lib/store';
  import { EXERCISES } from '$lib/exercises';
  import { listen } from '@tauri-apps/api/event';

  let s:any; settings.subscribe(v=>{ s=v; applyTheme(v.theme); });
  let p='Focus'; phase.subscribe(v=>p=v);
  let left=0; secondsLeft.subscribe(v=>left=v);

  onMount(async ()=>{
    await loadSettings();
    listen('tick', (e:any)=>{ const t=e.payload; phase.set(t.phase); secondsLeft.set(t.seconds_left); });
    listen('hk-start', ()=> start());
    listen('hk-stop', ()=> stop());
    listen('hk-snooze', ()=> (window as any).__TAURI_INVOKE__('cmd_snooze'));
  });

  function onSave(){ saveSettings(s); }
</script>

<div class="card" style="min-width:320px">
  <div class="h1">Rune Pause</div>
  <div class="row" style="justify-content:space-between;margin-top:8px">
    <div class="small">Fas: {p}</div>
    <div class="timer">{mmss(left)}</div>
  </div>
  <div class="row" style="margin-top:10px;gap:8px">
    <button on:click={start}>Start</button>
    <button on:click={stop}>Stop</button>
    <button on:click={()=>logPause(`Paus klar (${p})`)}>Logga i Obsidian</button>
  </div>

  <div class="card" style="margin-top:12px">
    <div class="row" style="gap:12px;flex-wrap:wrap">
      <label>Fokus <input type="number" bind:value={s.focusMin} min="10" max="120"> min</label>
      <label>Paus <input type="number" bind:value={s.breakMin} min="1" max="30"> min</label>
      <label>Micro var <input type="number" bind:value={s.microEvery} min="0" max="6"></label>
      <label>Micro längd <input type="number" bind:value={s.microMin} min="1" max="10"> min</label>
      <label class="row"><input type="checkbox" bind:checked={s.hardBreak}> Hard break</label>
    </div>
    <div class="row" style="gap:12px;margin-top:8px;flex-wrap:wrap">
      <label>Obsidian vault <input placeholder="/path/to/Vault" bind:value={s.obsidianVault}></label>
      <label>Webhook URL <input placeholder="https://homeassistant.local:8123/api/webhook/..." bind:value={s.webhookUrl}></label>
      <label>Theme
        <select bind:value={s.theme}>
          <option value="system">System</option>
          <option value="dark">Dark</option>
          <option value="light">Light</option>
        </select>
      </label>
      <button on:click={onSave}>Spara</button>
    </div>
  </div>

  <div class="card" style="margin-top:12px">
    <div class="h1" style="font-size:16px">Lås-block & genvägar</div>
    <div class="row" style="gap:12px;flex-wrap:wrap">
      <label>Idle-tröskel <input type="number" bind:value={s.idleThresholdMin} min="1" max="60"> min</label>
      <label>Blocks (komma-sep) <input placeholder="12:00-13:00, 21:00-21:30" bind:value={(s.forceBlocksStr ??= (s.forceBlocks||[]).join(', '))} on:change={() => { s.forceBlocks = (s.forceBlocksStr||'').split(',').map((x:string)=>x.trim()).filter(Boolean); }}></label>
    </div>
    <div class="row" style="gap:12px;flex-wrap:wrap;margin-top:6px">
      <label>Start HK <input bind:value={s.hotkeys.start} placeholder="CmdOrCtrl+Shift+P"></label>
      <label>Stop HK <input bind:value={s.hotkeys.stop} placeholder="CmdOrCtrl+Shift+O"></label>
      <label>Snooze HK <input bind:value={s.hotkeys.snooze} placeholder="CmdOrCtrl+Shift+S"></label>
      <button on:click={()=>{ onSave(); updateHotkeys(s); }}>Uppdatera genvägar</button>
    </div>
  </div>

  <div class="card" style="margin-top:12px">
    <div class="h1" style="font-size:16px">Övningar</div>
    <ul>
      {#each EXERCISES as ex}
        <li class="small"><b>{ex.title}</b> — {ex.steps.join(' · ')}</li>
      {/each}
    </ul>
  </div>
</div>
