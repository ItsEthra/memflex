use crate::{external::{Handle, NtResult}, MfError};
use core::mem::{size_of, zeroed};

#[link(name = "kernel32")]
extern "C" {
    fn ReadProcessMemory(
        hnd: isize,
        base: usize,
        buf: *mut u8,
        size: usize,
        read: Option<&mut usize>,
    ) -> NtResult;

    fn OpenProcess(
        access: u32,
        inherit: i32,
        id: u32
    ) -> Handle;
}

/// Owned handle to another process
pub struct OwnedProcess(Handle);

impl OwnedProcess {
    /// Takes ownership of handle
    pub unsafe fn from_handle(h: Handle) -> Self {
        Self(h)
    }

    /// Gives away ownership of the handle
    pub fn into_handle(self) -> Handle {
        self.0
    }

    /// Closes handle to the process
    pub fn close(self) -> crate::Result<()> {
        self.into_handle().close()
    }

    /// Opens process by its id.
    pub fn open_by_id(id: u32, inherit_handle: bool, access_rights: u32) -> crate::Result<Self> {
        unsafe {
            let h = OpenProcess(access_rights, inherit_handle as i32, id);
            if h.is_invalid() {
                MfError::last()
            } else {
                Ok(Self(h))
            }
        }
    }

    /// Reads process memory, returns amount of bytes read
    pub fn read_buf(&self, address: usize, buf: &mut [u8]) -> crate::Result<usize> {
        let mut read = 0;
        unsafe {
            ReadProcessMemory(
                self.0 .0,
                address,
                buf.as_mut_ptr(),
                buf.len(),
                Some(&mut read),
            )
            .into(read)
        }
    }

    /// Reads process memory, returning the value read at the address
    pub fn read<T: Clone>(&self, address: usize) -> crate::Result<T> {
        unsafe {
            let mut buf: T = zeroed();

            ReadProcessMemory(
                self.0 .0,
                address,
                &mut buf as *mut T as _,
                size_of::<T>(),
                None,
            )
            .into(buf)
        }
    }
}
