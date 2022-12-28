#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

use crate::types::Protection;

#[derive(Debug)]
/// Single process
pub struct ProcessEntry {
    /// Id of the process.
    pub id: u32,
    /// Name of the process.
    pub name: String,
    /// Id of the parent process.
    pub parent_id: u32,
}

#[cfg(windows)]
use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32W;
#[cfg(windows)]
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;

#[cfg(windows)]
impl ProcessEntry {
    fn from(pe: &PROCESSENTRY32W) -> Option<Self> {
        use windows::Win32::Foundation::MAX_PATH;
        use windows::Win32::Foundation::{BOOL, HANDLE};
        use windows::Win32::System::Threading::{
            OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
        };

        Some(Self {
            id: pe.th32ProcessID,
            parent_id: pe.th32ParentProcessID,
            name: String::from_utf16_lossy(unsafe {
                crate::terminated_array(pe.szExeFile.as_ptr(), 0)
            }),
        })
    }
}

impl ProcessEntry {
    /// Opens process by the entry's process id.
    #[cfg(windows)]
    pub fn open(
        &self,
        inherit_handle: bool,
        access_rights: PROCESS_ACCESS_RIGHTS,
    ) -> crate::Result<OwnedProcess> {
        open_process_by_id(self.id, inherit_handle, access_rights)
    }

    /// Opens process by the entry's process id.
    /// # Behavior
    /// Due to the way [`OwnedProcess`] works on unix, this function always
    /// returns Ok(P).
    #[cfg(unix)]
    pub fn open(&self) -> crate::Result<OwnedProcess> {
        Ok(OwnedProcess(self.id))
    }
}

/// Represents a chunk of mapped memory in a process
#[derive(Debug)]
pub struct MemoryRegion {
    /// Start
    pub from: usize,
    /// End
    pub to: usize,
    /// Prtection
    pub prot: Protection,
}
