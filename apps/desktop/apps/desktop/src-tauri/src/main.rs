mod state; mod bridge; mod idle;
use tauri::{Manager, tray::{TrayIconBuilder, TrayIconMenuBuilder, TrayIconEvent}};
use std::path::PathBuf;
use core_timer::{Engine, Settings as CoreSettings};
use crate::state::{Persist, UiSettings};
use bridge::AppState;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "macos")] fn conf_dir()->PathBuf{ dirs::home_dir().unwrap().join("Library/Application Support/RunePause") }
#[cfg(target_os = "windows")] fn conf_dir()->PathBuf{ dirs::data_dir().unwrap().join("RunePause") }

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(Default::default()))
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .setup(|app|{
      let persist = state::Persist::new(conf_dir());
      let ui = persist.load();
      let core = CoreSettings{
        focus_minutes: ui.focusMin, break_minutes: ui.breakMin,
        micro_every: ui.microEvery, micro_minutes: ui.microMin,
        hard_break: ui.hardBreak, snooze_sec: ui.snoozeSec,
      };
      let engine = Arc::new(Mutex::new(Engine::new(core)));
      let vault = ui.obsidianVault.as_ref().map(|s| PathBuf::from(s));
      app.manage(AppState{ engine, persist, obsidian_vault: vault });
      // idle watcher
      {
        let apph = app.handle();
        let threshold = ui.idleThresholdMin.unwrap_or(5);
        tauri::async_runtime::spawn(async move {
          loop {
            let idle = crate::idle::idle_seconds();
            if idle >= threshold as u64 * 60 {
              let _ = apph.emit_all("force-lock", &serde_json::json!({"reason":"idle","idleSec":idle}));
            }
            tauri::async_runtime::sleep(std::time::Duration::from_secs(15)).await;
          }
        });
      }

      let menu = TrayIconMenuBuilder::new()
        .item("Start", Some("start")).item("Stop", Some("stop")).separator()
        .item("Show", Some("show")).item("Quit", Some("quit")).build()?;
      TrayIconBuilder::new().id("tray").menu(&menu)
        .on_tray_icon_event(|app, ev| if let TrayIconEvent::MenuItemClick{ id, .. } = ev {
          match id.as_str() {
            "start" => { let _ = app.invoke("start_timer", tauri::ipc::InvokeBody::default()); },
            "stop"  => { let _ = app.invoke("stop_timer", tauri::ipc::InvokeBody::default()); },
            "show"  => { let _ = app.get_window("main").map(|w| w.show().ok()); },
            "quit"  => { std::process::exit(0); },
            _=>{}
          }
        }).build(app)?;
      if let Some(overlay) = app.get_window("overlay") { overlay.hide().ok(); }
      Ok(()) }).invoke_handler(tauri::generate_handler![
      bridge::start_timer, bridge::stop_timer,
      bridge::cmd_load_settings, bridge::cmd_save_settings,
      bridge::cmd_snooze, bridge::write_pause_log, bridge::cmd_update_hotkeys
    ]).run();
}
