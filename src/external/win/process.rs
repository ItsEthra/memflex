use crate::{
    external::{Handle, NtResult},
    MfError, types::ProtectionFlags,
};
use core::mem::{size_of, zeroed};

#[link(name = "kernel32")]
extern "C" {
    fn ReadProcessMemory(
        hnd: isize,
        addr: usize,
        buf: *mut u8,
        size: usize,
        read: Option<&mut usize>,
    ) -> NtResult;

    fn WriteProcessMemory(
        hnd: isize,
        addr: usize,
        buf: *const u8,
        size: usize,
        written: Option<&mut usize>,
    ) -> NtResult;

    fn VirtualProtectEx(
        hnd: isize,
        addr: usize,
        size: usize,
        new: ProtectionFlags,
        old: &mut ProtectionFlags
    ) -> NtResult;

    fn OpenProcess(access: u32, inherit: i32, id: u32) -> Handle;
}

/// Owned handle to another process
pub struct OwnedProcess(Handle);

impl OwnedProcess {
    /// Takes ownership of handle.
    pub unsafe fn from_handle(h: Handle) -> Self {
        Self(h)
    }

    /// Gives away ownership of the handle.
    pub fn into_handle(self) -> Handle {
        self.0
    }

    /// Closes handle to the process.
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

    /// Reads process memory, returns amount of bytes read.
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
            .expect_nonzero(read)
        }
    }

    /// Reads process memory, returning the value read at the address.
    pub fn read<T: Clone>(&self, address: usize) -> crate::Result<T> {
        unsafe {
            let mut buf: T = zeroed();

            ReadProcessMemory(
                self.0.0,
                address,
                &mut buf as *mut T as _,
                size_of::<T>(),
                None,
            )
            .expect_nonzero(buf)
        }
    }

    /// Writes buffer to the process memory, returning the amount of bytes written.
    pub fn write_buf(&self, address: usize, buf: &[u8]) -> crate::Result<usize> {
        unsafe {
            let mut written: usize = 0;

            WriteProcessMemory(
                self.0.0,
                address,
                buf.as_ptr(),
                buf.len(),
                Some(&mut written)
            ).expect_nonzero(written)
        }
    }

    /// Writes value to the process memory, returning the amount of bytes written.
    pub fn write<T: Clone>(&self, address: usize, value: T) -> crate::Result<usize> {
        unsafe {
            let mut written: usize = 0;

            WriteProcessMemory(
                self.0.0,
                address,
                &value as *const T as _,
                size_of::<T>(),
                Some(&mut written)
            ).expect_nonzero(written)
        }
    }

    /// Changes the protection of memory pages, returning the old protection value.
    pub fn protect(&self, address: usize, size: usize, protection: ProtectionFlags) -> crate::Result<ProtectionFlags> {
        let mut old = ProtectionFlags(0);
        unsafe {
            VirtualProtectEx(
                self.0.0,
                address,
                size,
                protection,
                &mut old
            ).expect_nonzero(old)
        }
    }
}
