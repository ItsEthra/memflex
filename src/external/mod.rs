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
    /// Path to the process execute.
    pub path: String,
    /// Name of the process.
    pub name: String,
    /// Id of the parent process.
    pub parent_id: u32,
}

impl ProcessEntry {
    /// Opens process by the entry's process id.
    #[cfg(windows)]
    pub fn open(
        &self,
        inherit_handle: bool,
        access_rights: ProcessRights,
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
pub struct MappedRegion {
    /// Start
    pub from: usize,
    /// End
    pub to: usize,
    /// Prtection
    pub prot: Protection
}
