/// Represents a single process in the system.
/// # Details
/// There is no such concept as 'owned' procses in unix. (i think).
/// The name is the same as on windows to reduce the hasle of cross-platform code.
#[repr(transparent)]
pub struct OwnedProcess(u32);

impl OwnedProcess {
    /// Returns the id of the process.
    pub fn id(&self) -> u32 {
        self.0
    }
}
