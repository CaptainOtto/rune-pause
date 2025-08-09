use tauri::{AppHandle, Emitter};
use chrono::Local;
use std::{fs::OpenOptions, io::Write, path::PathBuf};
use core_timer::{Engine, Settings as CoreSettings};
use crate::state::{Persist, UiSettings};
use std::sync::{Arc, Mutex};

pub struct AppState { pub engine: Arc<Mutex<Engine>>, pub persist: Persist, pub obsidian_vault: Option<PathBuf> }

#[tauri::command]
pub async fn start_timer(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
  let app2 = app.clone();
  let engine = state.engine.clone();
  tauri::async_runtime::spawn(async move {
    let rx = engine.lock().unwrap().subscribe();
    tauri::async_runtime::spawn({
      let app = app2.clone();
      async move { let mut rx = rx; while let Ok(tick) = rx.recv().await {
          // forward to UI
          let _ = app.emit("tick", &tick);
          // overlay + tray updates
          if let Some(tray) = app.tray_by_id("tray") {
            let _ = tray.set_tooltip(Some(format!("{}  {:02}:{:02}", format!("{:?}", tick.phase), tick.seconds_left/60, tick.seconds_left%60)));
          }
          match tick.phase {
            core_timer::Phase::Break | core_timer::Phase::MicroBreak => {
              if let Some(ov) = app.get_window("overlay") { let _ = ov.show(); let _ = ov.set_focus(); let _ = ov.set_always_on_top(true); }
            },
            core_timer::Phase::Focus => {
              if let Some(ov) = app.get_window("overlay") { let _ = ov.hide(); }
            },
            _ => {}
          }
        } }
    });
    engine.lock().unwrap().run().await;
  }); Ok(())
}
#[tauri::command] pub fn stop_timer(state: tauri::State<'_, AppState>) { state.engine.lock().unwrap().interrupt(); }
#[tauri::command] pub fn cmd_load_settings(state: tauri::State<'_, AppState>) -> Result<UiSettings, String> { Ok(state.persist.load()) }
#[tauri::command] pub fn cmd_save_settings(state: tauri::State<'_, AppState>, settings: UiSettings) -> Result<(), String> {
  state.persist.save(&settings);
  let mut e = state.engine.lock().unwrap();
  e.settings = CoreSettings{
    focus_minutes: settings.focusMin, break_minutes: settings.breakMin,
    micro_every: settings.microEvery, micro_minutes: settings.microMin,
    hard_break: settings.hardBreak, snooze_sec: settings.snoozeSec,
  }; Ok(())
}
#[tauri::command] pub fn cmd_snooze(state: tauri::State<'_, AppState>) -> Result<(), String> { state.engine.lock().unwrap().interrupt(); Ok(()) }
#[tauri::command]
pub fn cmd_update_hotkeys(app: tauri::AppHandle, settings: UiSettings) -> Result<(), String> {
  use tauri_plugin_global_shortcut::Shortcut;
  let mgr = app.global_shortcut(); let _ = mgr.unregister_all();
  if let Some(s) = settings.hotkeys.get("start") { let appc = app.clone(); let _ = mgr.register(&Shortcut::parse(s).map_err(|e| e.to_string())?, move || { let _ = appc.emit("hk-start", &()); }); }
  if let Some(s) = settings.hotkeys.get("stop") { let appc = app.clone(); let _ = mgr.register(&Shortcut::parse(s).map_err(|e| e.to_string())?, move || { let _ = appc.emit("hk-stop", &()); }); }
  if let Some(s) = settings.hotkeys.get("snooze") { let appc = app.clone(); let _ = mgr.register(&Shortcut::parse(s).map_err(|e| e.to_string())?, move || { let _ = appc.emit("hk-snooze", &()); }); }
  Ok(()) }
#[tauri::command]
pub fn write_pause_log(state: tauri::State<'_, AppState>, summary: String) -> Result<(), String> {
  if let Some(vault) = &state.obsidian_vault {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let path = vault.join(format!("Daily/{}.md", date));
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let line = format!("
- ⏸ {} — {}
", Local::now().format("%H:%M"), summary);
    let mut f = OpenOptions::new().create(true).append(true).open(&path).map_err(|e| e.to_string())?;
    f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
  } Ok(()) }
