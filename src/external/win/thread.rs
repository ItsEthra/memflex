use crate::MfError;
use core::mem::{size_of, zeroed};
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        Threading::{
            GetThreadId, NtQueryInformationThread, OpenThread, ResumeThread, SuspendThread,
            TerminateThread, THREADINFOCLASS, THREAD_ACCESS_RIGHTS,
        },
    },
};

/// Owned handle to a process's thread
pub struct OwnedThread(HANDLE);
impl OwnedThread {
    /// Takes ownership of handle.
    /// # Safety
    /// Handle must not be used anywhere else.
    pub unsafe fn from_handle(h: HANDLE) -> Self {
        Self(h)
    }

    /// Gives away ownership of the process's handle.
    pub fn into_handle(self) -> HANDLE {
        self.0
    }

    /// Closes handle to the thread.
    pub fn close(self) -> crate::Result<()> {
        unsafe {
            if CloseHandle(self.0).as_bool() {
                Ok(())
            } else {
                MfError::last()
            }
        }
    }

    /// Returns the start address of the thread
    pub fn start_address(&self) -> crate::Result<usize> {
        unsafe {
            let mut addr = 0;
            NtQueryInformationThread(
                self.0,
                THREADINFOCLASS(0x9),
                &mut addr as *mut usize as _,
                size_of::<usize>() as _,
                0 as _,
            )
            .map(|_| addr)
            .map_err(|_| MfError::last::<()>().unwrap_err())
        }
    }

    /// Suspends thread
    pub fn suspend(&self) -> crate::Result<u32> {
        unsafe {
            let i = SuspendThread(self.0);
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
            let i = ResumeThread(self.0);
            if i == u32::MAX {
                MfError::last()
            } else {
                Ok(i)
            }
        }
    }

    /// Terminates the thread with the specified code
    pub fn terminate(&self, exit_code: u32) -> crate::Result<()> {
        unsafe {
            if TerminateThread(self.0, exit_code).as_bool() {
                Ok(())
            } else {
                MfError::last()
            }
        }
    }

    /// Returns the id of the thread
    pub fn id(&self) -> u32 {
        unsafe { GetThreadId(self.0) }
    }
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
        access_rights: THREAD_ACCESS_RIGHTS,
    ) -> crate::Result<OwnedThread> {
        open_thread_by_id(self.id, inherit_handle, access_rights)
    }
}

impl From<&THREADENTRY32> for ThreadEntry {
    fn from(te: &THREADENTRY32) -> Self {
        Self {
            id: te.th32ThreadID,
            base_priority: te.tpBasePri,
        }
    }
}

/// Iterator over threads in a process
pub struct ThreadIterator {
    h: HANDLE,
    entry: THREADENTRY32,
    stop: bool,
    pid: u32,
}

impl ThreadIterator {
    /// Creates an iterator over threads in the process.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            if let Ok(h) = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process_id) {
                let mut this = Self {
                    h,
                    entry: zeroed(),
                    stop: false,
                    pid: process_id,
                };

                this.entry.dwSize = size_of::<THREADENTRY32>() as _;

                if !Thread32First(this.h, &mut this.entry).as_bool() {
                    return MfError::last();
                }

                while this.entry.th32OwnerProcessID != this.pid || this.stop {
                    this.stop = !Thread32Next(this.h, &mut this.entry).as_bool();
                }

                if this.stop {
                    return Err(MfError::NoThreads);
                }

                Ok(this)
            } else {
                MfError::last()
            }
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
                self.stop = !Thread32Next(self.h, &mut self.entry).as_bool();

                if self.entry.th32OwnerProcessID == self.pid {
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
    access_rights: THREAD_ACCESS_RIGHTS,
) -> crate::Result<OwnedThread> {
    unsafe {
        if let Ok(h) = OpenThread(access_rights, BOOL(inherit_handle as _), thread_id) {
            Ok(OwnedThread(h))
        } else {
            MfError::last()
        }
    }
}
