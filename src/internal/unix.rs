use crate::{types::Protection, MfError};

/// Changes the protection of a memory region
pub fn protect(address: usize, len: usize, prot: Protection) -> crate::Result<()> {
    unsafe {
        if libc::mprotect(address as _, len, prot.to_os()) == 0 {
            Ok(())
        } else {
            MfError::last()
        }
    }
}

/// Allocates virtual memory
pub fn allocate(address: Option<usize>, len: usize, prot: Protection) -> crate::Result<*mut u8> {
    unsafe {
        let addr = libc::mmap(
            address.unwrap_or(0) as _,
            len,
            prot.to_os(),
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if !addr.is_null() {
            Ok(addr as _)
        } else {
            MfError::last()
        }
    }
}

/// Frees virtual memory
pub fn free(address: usize, len: usize) -> crate::Result<()> {
    unsafe {
        if libc::munmap(address as _, len) == 0 {
            Ok(())
        } else {
            MfError::last()
        }
    }
}

/// Returns the id of the current process
pub fn pid() -> u32 {
    unsafe { libc::getpid() as _ }
}
