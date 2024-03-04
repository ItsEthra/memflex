use core::fmt::{self, Display};

/// Global error type
#[derive(Debug)]
pub enum MfError {
    /// Nt error code
    #[cfg(windows)]
    NtStatus(u32),
    /// Unix errno
    #[cfg(unix)]
    Errno(i32),
    /// Specified process was not found
    ProcessNotFound,
    /// Specified module was not found
    ModuleNotFound,
    /// No threads running in the process
    NoThreads,
    /// String read was not valid UTF-8 or UTF-16 byte sequence
    InvalidString,
    /// Process has died and is no longer available
    ProcessDied,
}

#[allow(dead_code)]
impl MfError {
    #[cfg(all(windows, feature = "std"))]
    pub(crate) fn last<T>() -> Result<T> {
        use windows::Win32::Foundation::GetLastError;

        Err(MfError::NtStatus(unsafe { GetLastError().0 }))
    }

    #[cfg(all(unix, feature = "std"))]
    pub(crate) fn last<T>() -> Result<T> {
        #[cfg(target_os = "linux")]
        unsafe { Err(MfError::Errno(*libc::__errno_location())) }
        #[cfg(target_os = "macos")]
        unsafe { Err(MfError::Errno(*libc::__error())) }
    }
}

impl Display for MfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MfError {}

#[allow(missing_docs)]
pub type Result<T> = core::result::Result<T, MfError>;
