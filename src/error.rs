/// Global error type
#[derive(Debug)]
pub enum MfError {
    /// Nt error code
    #[cfg(windows)]
    NtStatus(u32),
}

impl MfError {
    #[cfg(windows)]
    pub(crate) fn last<T>() -> Result<T> {
        extern "C" {
            fn GetLastError() -> u32;
        }

        Err(MfError::NtStatus(unsafe { GetLastError() }))
    }
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, MfError>;
