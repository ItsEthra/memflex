use super::CreateToolhelp32Snapshot;
use crate::{external::Handle, terminated_array, MfError};
use core::mem::{size_of, zeroed};

extern "C" {
    fn Module32FirstW(hnd: isize, lpme: &mut FfiModuleEntry) -> bool;
    fn Module32NextW(hnd: isize, lpme: &mut FfiModuleEntry) -> bool;
}

#[repr(C)]
struct FfiModuleEntry {
    size: u32,
    module_id: u32,
    process_id: u32,
    glbl: u32,
    proccnt: u32,
    base: usize,
    mod_size: u32,
    hndl: usize,
    name: [u16; 256],
    path: [u16; 260],
}

#[derive(Debug, Clone)]
/// Entry reprenses a single module in a process.
pub struct ModuleEntry {
    /// Module's base address
    pub base: usize,
    /// Module's size
    pub size: usize,
    /// Module's name
    pub name: String,
    /// Module's path
    pub path: String,
}

impl From<&FfiModuleEntry> for ModuleEntry {
    fn from(me: &FfiModuleEntry) -> Self {
        unsafe {
            Self {
                base: me.base,
                size: me.mod_size as _,
                name: String::from_utf16_lossy(terminated_array(me.name.as_ptr(), 0)),
                path: String::from_utf16_lossy(terminated_array(me.path.as_ptr(), 0)),
            }
        }
    }
}

/// Iterator over all modules in a process.
pub struct ModuleIterator {
    h: Handle,
    entry: FfiModuleEntry,
    stop: bool,
}

impl ModuleIterator {
    /// Creates new iterator over process's modules.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            let h = CreateToolhelp32Snapshot(0x8 | 0x10, process_id);
            if h.is_invalid() {
                return MfError::last();
            }

            let mut this = Self {
                h,
                entry: zeroed(),
                stop: false,
            };

            this.entry.size = size_of::<FfiModuleEntry>() as _;

            if !Module32FirstW(this.h.0, &mut this.entry) {
                return MfError::last();
            }

            Ok(this)
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = ModuleEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let current = ModuleEntry::from(&self.entry);
        unsafe {
            self.stop = !Module32NextW(self.h.0, &mut self.entry);
        }
        Some(current)
    }
}
