use super::{open_process_by_id, ProcessIterator, ThreadIterator};
use crate::{types::ModuleInfoWithName, MfError};
use core::mem::{size_of, zeroed};
use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
            TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
        },
        Threading::PROCESS_QUERY_INFORMATION,
    },
};

/// Iterator over all modules in a process.
pub struct ModuleIterator {
    h: HANDLE,
    entry: MODULEENTRY32W,
    stop: bool,
}

impl ModuleIterator {
    /// Creates new iterator over process's modules.
    pub fn new(process_id: u32) -> crate::Result<Self> {
        unsafe {
            if let Ok(h) =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id)
            {
                let mut this = Self {
                    h,
                    entry: zeroed(),
                    stop: false,
                };
                this.entry.dwSize = size_of::<MODULEENTRY32W>() as _;

                if !Module32FirstW(this.h, &mut this.entry).as_bool() {
                    return MfError::last();
                }

                Ok(this)
            } else {
                MfError::last()
            }
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = ModuleInfoWithName;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let current = ModuleInfoWithName::from(&self.entry);
        unsafe {
            self.stop = !Module32NextW(self.h, &mut self.entry).as_bool();
        }
        Some(current)
    }
}

/// Helper function for iterating over processes
/// # Panics
/// If failed to create an iterator over processes. Refer to [`ProcessIterator::new`]
pub fn processes() -> ProcessIterator {
    ProcessIterator::new().unwrap()
}

/// Helper function for iterating over process's modules.
/// # Panics
/// * If failed to open the process. Refer to [`open_process_by_id`].
/// * If failed to create iterator over process's modules. Refer to [`ModuleIterator::new`]
pub fn modules(process_id: u32) -> impl Iterator<Item = ModuleInfoWithName> {
    open_process_by_id(process_id, false, PROCESS_QUERY_INFORMATION)
        .expect("Faild to open the process")
        .modules()
        .expect("Faild to create an iterator over process's modules")
}

/// Helper function for iterating over process's threads.
/// # Panics
/// * If failed to open the process. Refer to [`open_process_by_id`].
/// * If failed to create iterator over process's threads. Refer to [`ThreadIterator::new`]
pub fn threads(process_id: u32) -> ThreadIterator {
    open_process_by_id(process_id, false, PROCESS_QUERY_INFORMATION)
        .expect("Faild to open the process")
        .threads()
        .expect("Faild to create an iterator over process's modules")
}
