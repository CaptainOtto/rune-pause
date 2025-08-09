use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiSettings {
  pub focusMin:u64, pub breakMin:u64, pub microEvery:u64, pub microMin:u64, pub hardBreak:bool,
  pub theme:String, pub obsidianVault:Option<String>, pub enableIdle:bool, pub snoozeSec:u64,
  pub idleThresholdMin: Option<u64>, pub forceBlocks: Option<Vec<String>>, pub hotkeys: std::collections::HashMap<String,String>,
  pub webhookUrl: Option<String>,
}

impl Default for UiSettings { fn default()->Self{
  let mut hk = std::collections::HashMap::new();
  hk.insert("start".into(),"CmdOrCtrl+Shift+P".into());
  hk.insert("stop".into(),"CmdOrCtrl+Shift+O".into());
  hk.insert("snooze".into(),"CmdOrCtrl+Shift+S".into());
  Self{ focusMin:50, breakMin:10, microEvery:2, microMin:2, hardBreak:true, theme:"system".into(), obsidianVault:None, enableIdle:true, snoozeSec:60, idleThresholdMin: Some(5), forceBlocks: Some(vec![]), hotkeys: hk, webhookUrl: None }
}}

pub struct Persist { pub dir: PathBuf }
impl Persist {
  pub fn new(dir: PathBuf)->Self{ Self{ dir } }
  pub fn path(&self)->PathBuf{ self.dir.join("settings.json") }
  pub fn load(&self)->UiSettings{
    fs::create_dir_all(&self.dir).ok();
    let p = self.path();
    if let Ok(data) = fs::read_to_string(&p) { serde_json::from_str(&data).unwrap_or_default() }
    else {
      let mut d = UiSettings::default();
      if let Some(home) = dirs::home_dir() { d.obsidianVault = Some(home.join("Obsidian").to_string_lossy().to_string()); }
      d
    }
  }
  pub fn save(&self, s:&UiSettings){
    fs::create_dir_all(&self.dir).ok();
    let _ = fs::write(self.path(), serde_json::to_string_pretty(&s).unwrap());
  }
}
