use crate::{external::Handle, MfError};
use super::CreateToolhelp32Snapshot;
use core::mem::{zeroed, size_of};

extern "C" {
    fn Thread32First(hnd: isize, lpme: &mut FfiThreadEntry) -> bool;
    fn Thread32Next(hnd: isize, lpme: &mut FfiThreadEntry) -> bool;
}

#[repr(C)]
struct FfiThreadEntry {
    size: u32,
    usage: u32,
    tid: u32,
    pid: u32,
    base_pri: i32,
    delta_pri: i32,
    flags: u32
}

/// Thread represents the thread in a process.
#[derive(Debug, Clone)]
pub struct ThreadEntry {
    /// Thread id
    pub id: u32,
    /// Thread's priority
    pub base_priority: i32,
    /// Thread's start address
    pub start_address: usize
}

impl From<&FfiThreadEntry> for ThreadEntry {
    fn from(te: &FfiThreadEntry) -> Self {
        Self {
            id: te.tid,
            base_priority: te.base_pri,
            start_address: 0,
        }
    }
}

/// Iterator over threads in a process
pub struct ThreadIterator {
    h: Handle,
    entry: FfiThreadEntry,
    stop: bool,
    pid: u32
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