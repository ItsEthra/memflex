mod process;
pub use process::*;
mod module;
pub use module::*;
mod thread;
pub use thread::*;

use crate::MfError;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId},
    },
};

/// Tries to find its window by `class_name` and/or `window_name`
pub fn find_window(class_name: Option<&str>, window_name: Option<&str>) -> crate::Result<HWND> {
    let cn = class_name.map(|o| format!("{o}\0").encode_utf16().collect::<Vec<_>>());
    let wn = window_name.map(|o| format!("{o}\0").encode_utf16().collect::<Vec<_>>());

    unsafe {
        let h = FindWindowW(
            cn.as_ref().map(|v| PCWSTR(v.as_ptr() as _)),
            wn.as_ref().map(|v| PCWSTR(v.as_ptr() as _)),
        );

        if h.0 == 0 || h.0 == -1 {
            MfError::last()
        } else {
            Ok(h)
        }
    }
}

/// Searches for the process, thread that created the specified window.
/// Returns (process_id, thread_id) on success.
pub fn find_window_process_thread(hwnd: HWND) -> crate::Result<(u32, u32)> {
    unsafe {
        let mut pid = 0;
        let tid = GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if tid == 0 {
            MfError::last()
        } else {
            Ok((pid, tid))
        }
    }
}
