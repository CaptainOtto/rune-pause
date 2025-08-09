import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export type Settings = {
  focusMin:number; breakMin:number; microEvery:number; microMin:number; hardBreak:boolean;
  theme:'system'|'dark'|'light';
  obsidianVault?: string;
  enableIdle:boolean; idleThresholdMin?: number; snoozeSec:number;
  hotkeys:{ start:string; stop:string; snooze:string };
  forceBlocks?: string[]; // ["12:00-13:00","21:00-21:30"]
  webhookUrl?: string;
};

export const defaults: Settings = {
  focusMin:50, breakMin:10, microEvery:2, microMin:2, hardBreak:true,
  theme:'system', enableIdle:true, idleThresholdMin:5, snoozeSec:60,
  hotkeys: { start:'CmdOrCtrl+Shift+P', stop:'CmdOrCtrl+Shift+O', snooze:'CmdOrCtrl+Shift+S' },
  forceBlocks: [], webhookUrl: ''
};

export const settings = writable<Settings>(defaults);
export const phase = writable<'Focus'|'MicroBreak'|'Break'|'Idle'>('Focus');
export const secondsLeft = writable(0);

export async function loadSettings(){
  const s = await invoke<Settings>('cmd_load_settings');
  settings.set({ ...defaults, ...s });
}
export async function saveSettings(s:Settings){
  settings.set(s); await invoke('cmd_save_settings', { settings: s });
}
export async function updateHotkeys(s:Settings){
  await invoke('cmd_update_hotkeys', { settings: s });
}

export async function start(){ await invoke('start_timer'); }
export async function stop(){ await invoke('stop_timer'); }
export async function snooze(){ await invoke('cmd_snooze'); }
export async function logPause(summary:string){ await invoke('write_pause_log', { summary }); }
