use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::{sync::broadcast, time::sleep};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Phase { Focus, MicroBreak, Break, Idle }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
  pub focus_minutes: u64,
  pub break_minutes: u64,
  pub micro_every: u64,
  pub micro_minutes: u64,
  pub hard_break: bool,
  pub snooze_sec: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tick { pub phase: Phase, pub seconds_left: u64, pub cycle: u64 }

pub struct Engine {
  pub settings: Settings,
  tx: broadcast::Sender<Tick>,
  stop_tx: broadcast::Sender<()>,
  pub cycle: u64,
}

impl Engine {
  pub fn new(settings: Settings) -> Self {
    let (tx, _) = broadcast::channel(64);
    let (stop_tx, _) = broadcast::channel(4);
    Self { settings, tx, stop_tx, cycle: 0 }
  }
  pub fn subscribe(&self) -> broadcast::Receiver<Tick> { self.tx.subscribe() }
  pub fn stop_sub(&self) -> broadcast::Receiver<()> { self.stop_tx.subscribe() }
  pub fn interrupt(&self) { let _ = self.stop_tx.send(()); }

  pub async fn run(&mut self) {
    loop {
      self.cycle += 1;
      self.phase(Phase::Focus, self.settings.focus_minutes*60).await;
      if self.settings.micro_every > 0 && self.cycle % self.settings.micro_every == 0 {
        self.phase(Phase::MicroBreak, self.settings.micro_minutes*60).await;
      } else {
        self.phase(Phase::Break, self.settings.break_minutes*60).await;
      }
    }
  }

  async fn phase(&self, phase: Phase, mut secs: u64) {
    let mut stop_rx = self.stop_sub();
    while secs > 0 {
      let _ = self.tx.send(Tick{ phase, seconds_left: secs, cycle: self.cycle });
      tokio::select! {
        _ = sleep(Duration::from_secs(1)) => { secs -= 1; },
        _ = stop_rx.recv() => { break; }
      }
    }
  }
}
