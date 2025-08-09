#[cfg(target_os = "macos")]
mod mac {
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(state_id: u32, event_type: i32) -> f64;
    }
    const kCGEventSourceStateCombinedSessionState: u32 = 0;
    const kCGAnyInputEventType: i32 = -1;
    pub fn idle_seconds() -> u64 {
        unsafe {
            let secs = CGEventSourceSecondsSinceLastEventType(kCGEventSourceStateCombinedSessionState, kCGAnyInputEventType);
            if secs.is_nan() || secs.is_infinite() { 0 } else { secs as u64 }
        }
    }
}
#[cfg(target_os = "windows")]
mod win {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetLastInputInfo;
    use std::mem::size_of;
    #[repr(C)]
    struct LASTINPUTINFO { cbSize: u32, dwTime: u32 }
    pub fn idle_seconds() -> u64 {
        unsafe {
            let mut lii = LASTINPUTINFO { cbSize: size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
            if GetLastInputInfo(&mut lii as *mut _ as *mut _) == 0 { return 0; }
            // Approximate using uptime from std since UNIX_EPOCH (fallback)
            let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            let last_ms = lii.dwTime as u64;
            let diff = now_ms.saturating_sub(last_ms);
            diff / 1000
        }
    }
}
#[cfg(target_os = "macos")] pub fn idle_seconds() -> u64 { mac::idle_seconds() }
#[cfg(target_os = "windows")] pub fn idle_seconds() -> u64 { win::idle_seconds() }
#[cfg(not(any(target_os = "macos", target_os = "windows")))] pub fn idle_seconds() -> u64 { 0 }
