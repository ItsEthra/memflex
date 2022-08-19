#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

use crate::MfError;

/// Represens an owned handle
#[repr(transparent)]
pub struct Handle(pub(crate) isize);
impl Handle {
    /// Creates new handle
    /// # Safety
    /// `h` must be a valid handle
    pub unsafe fn new(h: isize) -> Self {
        Self(h)
    }

    /// Closes handle
    pub fn close(self) -> crate::Result<()> {
        #[link(name = "ntdll")]
        extern "C" {
            fn NtClose(h: isize) -> NtResult;
        }

        unsafe { NtClose(self.0).expect_zero(()) }
    }

    /// Checks if the handle is invalid, i.e. if the `value` == 0 || `value` == -1
    pub fn is_invalid(&self) -> bool {
        self.0 == 0 || self.0 == -1
    }
}

impl From<Handle> for isize {
    fn from(h: Handle) -> Self {
        h.0
    }
}

#[repr(transparent)]
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
