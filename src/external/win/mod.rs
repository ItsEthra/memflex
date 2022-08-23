mod process;
use core::ptr::NonNull;

pub use process::*;
mod module;
pub use module::*;
mod thread;
pub use thread::*;

use crate::MfError;

#[link(name = "user32")]
extern "C" {
    fn FindWindowW(class: Option<NonNull<u16>>, name: Option<NonNull<u16>>) -> Handle;
    fn GetWindowThreadProcessId(hwnd: Handle, pproc_id: &mut u32) -> u32;
}

#[link(name = "ntdll")]
extern "C" {
    fn NtClose(h: isize) -> NtResult;
}

/// Represens an owned handle
/// # Behavior
/// When dropped will call `NtClose` on the handle.
#[repr(transparent)]
pub struct Handle(pub(crate) isize);
impl Handle {
    /// Creates new handle
    /// # Safety
    /// `h` must be a valid handle
    pub unsafe fn new(h: isize) -> Self {
        Self(h)
    }

    /// Closes handle, providers an error code if closing failed.
    pub fn close(self) -> crate::Result<()> {
        let val = self.0;
        std::mem::forget(self);
        unsafe { NtClose(val).expect_zero(()) }
    }

    /// Checks if the handle is invalid, i.e. if the `value` == 0 || `value` == -1
    pub fn is_invalid(&self) -> bool {
        self.0 == 0 || self.0 == -1
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { NtClose(self.0); }
    }
}

impl From<Handle> for isize {
    fn from(h: Handle) -> Self {
        h.0
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct NtResult(u32);
impl NtResult {
    fn expect_nonzero<T>(self, val: T) -> crate::Result<T> {
        #[link(name = "kernel32")]
        extern "C" {
            fn GetLastError() -> u32;
        }

        if self.0 != 0 {
            Ok(val)
        } else {
            Err(MfError::NtStatus(unsafe { GetLastError() }))
        }
    }

    fn expect_zero<T>(self, val: T) -> crate::Result<T> {
        #[link(name = "kernel32")]
        extern "C" {
            fn GetLastError() -> u32;
        }

        if self.0 == 0 {
            Ok(val)
        } else {
            Err(MfError::NtStatus(unsafe { GetLastError() }))
        }
    }
}

/// Tries to find its window by `class_name` and/or `window_name`
pub fn find_window(class_name: Option<&str>, window_name: Option<&str>) -> crate::Result<Handle> {
    let cn = class_name.map(|o| format!("{o}\0").encode_utf16().collect::<Vec<_>>());
    let wn = window_name.map(|o| format!("{o}\0").encode_utf16().collect::<Vec<_>>());

    unsafe {
        let h = FindWindowW(
            cn.as_ref().map(|v| NonNull::new(v.as_ptr() as _).unwrap()),
            wn.as_ref().map(|v| NonNull::new(v.as_ptr() as _).unwrap())
        );

        if h.is_invalid() {
            MfError::last()
        } else {
            Ok(h)
        }
    }
}

/// Searches for the process, thread that created the specified window.
/// Returns (process_id, thread_id) on success.
pub fn find_window_process_thread(hwnd: Handle) -> crate::Result<(u32, u32)> {
    unsafe {
        let mut pid = 0;
        let tid = GetWindowThreadProcessId(hwnd, &mut pid);
        if tid == 0 {
            MfError::last()
        } else {
            Ok((pid, tid))
        }
    }
}