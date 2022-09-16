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
    /// Value was null
    #[cfg(feature = "nightly")]
    Null,
}

#[cfg(feature = "nightly")]
impl crate::ptr::NullError for MfError {
    fn null() -> Self {
        Self::Null
    }
}

#[allow(dead_code)]
impl MfError {
    #[cfg(windows)]
    pub(crate) fn last<T>() -> Result<T> {
        use windows::Win32::Foundation::GetLastError;

        Err(MfError::NtStatus(unsafe { GetLastError().0 }))
    }

    #[cfg(unix)]
    pub(crate) fn last<T>() -> Result<T> {
        unsafe { Err(MfError::Errno(*libc::__errno_location())) }
    }
}

impl Display for MfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MfError {}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, MfError>;
