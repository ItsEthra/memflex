mod process;
pub use process::*;
mod module;
pub use module::*;
mod thread;
pub use thread::*;

use crate::MfError;

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
