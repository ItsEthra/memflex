use super::CreateToolhelp32Snapshot;
use crate::{
    external::{Handle, NtResult},
    types::win::ThreadRights,
    MfError,
};
use core::mem::{size_of, zeroed};

#[link(name = "kernel32")]
extern "C" {
    fn Thread32First(hnd: isize, lpme: &mut FfiThreadEntry) -> bool;
    fn Thread32Next(hnd: isize, lpme: &mut FfiThreadEntry) -> bool;
    fn OpenThread(access: ThreadRights, inherit: i32, tid: u32) -> Handle;

    fn SuspendThread(hnd: isize) -> u32;
    fn ResumeThread(hnd: isize) -> u32;
    fn TerminateThread(hnd: isize, code: u32) -> NtResult;

    fn GetThreadId(hnd: isize) -> u32;
}

#[link(name = "ntdll")]
extern "C" {
    fn NtQueryInformationThread(
        hnd: isize,
        class: u32,
        buf: *mut u8,
        size: usize,
        out: Option<&mut usize>,
    ) -> NtResult;
}

/// Owned handle to a process's thread
pub struct OwnedThread(Handle);
impl OwnedThread {
    /// Takes ownership of handle.
    /// # Safety
    /// Handle must not be used anywhere else.
    pub unsafe fn from_handle(h: Handle) -> Self {
        Self(h)
    }

    /// Gives away ownership of the process's handle.
    pub fn into_handle(self) -> Handle {
        self.0
    }

    /// Closes handle to the thread.
    pub fn close(self) -> crate::Result<()> {
        self.into_handle().close()
    }

    /// Returns the start address of the thread
    pub fn start_address(&self) -> crate::Result<usize> {
        unsafe {
            let mut addr = 0;
            NtQueryInformationThread(
                self.0 .0,
                0x9,
                &mut addr as *mut usize as _,
                size_of::<usize>(),
                None,
            )
            .expect_zero(addr)
        }
    }

    /// Suspends thread
    pub fn suspend(&self) -> crate::Result<u32> {
        unsafe {
            let i = SuspendThread(self.0 .0);
            if i == u32::MAX {
                MfError::last()
            } else {
                Ok(i)
            }
        }
    }

    /// Resumes thread
    pub fn resume(&self) -> crate::Result<u32> {
        unsafe {
            let i = ResumeThread(self.0 .0);
            if i == u32::MAX {
                MfError::last()
            } else {
                Ok(i)
            }
        }
    }

    /// Terminates the thread with the specified code
    pub fn terminate(&self, exit_code: u32) -> crate::Result<()> {
        unsafe { TerminateThread(self.0 .0, exit_code).expect_nonzero(()) }
    }

    /// Returns the id of the thread
    pub fn id(&self) -> u32 {
        unsafe { GetThreadId(self.0 .0) }
    }
}

#[repr(C)]
struct FfiThreadEntry {
    size: u32,
    usage: u32,
    tid: u32,
    pid: u32,
    base_pri: i32,
    delta_pri: i32,
    flags: u32,
}

/// Thread represents the thread in a process.
#[derive(Debug, Clone)]
pub struct ThreadEntry {
    /// Thread id
    pub id: u32,
    /// Thread's priority
    pub base_priority: i32,
}

impl ThreadEntry {
    /// Tried to open thread by entry's thread id.
    pub fn open(
        &self,
        inherit_handle: bool,
        access_rights: ThreadRights,
    ) -> crate::Result<OwnedThread> {
        open_thread_by_id(self.id, inherit_handle, access_rights)
    }
}

impl From<&FfiThreadEntry> for ThreadEntry {
    fn from(te: &FfiThreadEntry) -> Self {
        Self {
            id: te.tid,
            base_priority: te.base_pri,
        }
    }
}

/// Iterator over threads in a process
pub struct ThreadIterator {
    h: Handle,
    entry: FfiThreadEntry,
    stop: bool,
    pid: u32,
}

impl ThreadIterator {
    /// Creates an iterator over threads in the process.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let h = CreateToolhelp32Snapshot(0x4, process_id);
            if h.is_invalid() {
                return MfError::last();
            }

            let mut this = Self {
                h,
                entry: zeroed(),
                stop: false,
                pid: process_id,
            };

            this.entry.size = size_of::<FfiThreadEntry>() as _;

            if !Thread32First(this.h.0, &mut this.entry) {
                return MfError::last();
            }

            while this.entry.pid != this.pid || this.stop {
                this.stop = !Thread32Next(this.h.0, &mut this.entry);
            }

            if this.stop {
                return Err(MfError::NoThreads);
            }

            Ok(this)
        }
    }
}

impl Iterator for ThreadIterator {
    type Item = ThreadEntry;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            Some(loop {
                if self.stop {
                    return None;
                }

                let current = ThreadEntry::from(&self.entry);
                self.stop = !Thread32Next(self.h.0, &mut self.entry);

                if self.entry.pid == self.pid {
                    break current;
                }
            })
        }
    }
}

/// Tries to open thread by its id.
pub fn open_thread_by_id(
    thread_id: u32,
    inherit_handle: bool,
    access_rights: ThreadRights,
) -> crate::Result<OwnedThread> {
    unsafe {
        let hnd = OpenThread(access_rights, inherit_handle as i32, thread_id);
        if hnd.is_invalid() {
            MfError::last()
        } else {
            Ok(OwnedThread(hnd))
        }
    }
}
