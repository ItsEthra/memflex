use crate::{types::ModuleAdvancedInfo, MfError};

/// Represents a single process in the system.
/// # Details
/// There is no such concept as 'owned' procses in unix. (i think).
/// The name is the same as on windows to reduce the hasle of cross-platform code.
#[repr(transparent)]
pub struct OwnedProcess(u32);

impl OwnedProcess {
    /// Creates a new `OwnedProcess` from id.
    ///
    /// This function DOESN'T creates a new process.
    /// `OwnedProcess` just provides functions to interact with already
    /// existing process
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the id of the process.
    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    /// Reads process memory, returning amount of bytes read.
    pub fn read_buf(&self, address: usize, buf: &mut [u8]) -> crate::Result<usize> {
        unsafe {
            let read = libc::process_vm_readv(self.0 as _,
                &libc::iovec {
                    iov_base: buf.as_mut_ptr() as _,
                    iov_len: buf.len(),
                }, 1,
                &libc::iovec {
                    iov_base: address as _,
                    iov_len: buf.len()
                }, 1,
                0
            );

            if read == -1 {
                MfError::last()
            } else {
                Ok(read as usize)
            }
        }
    }

    /// Returns an iterator over process's modules.
    pub fn modules(&self) -> crate::Result<impl Iterator<Item = ModuleAdvancedInfo>> {
        use std::{fs, path::PathBuf, collections::HashMap};
        
        let s = fs::read_to_string(format!("/proc/{}/maps", self.0)).map_err(|_| MfError::ProcessNotFound)?;
        let mut maps: HashMap<String, (usize, usize)> = HashMap::new();

        for l in s.lines() {
            let map = l.split(' ').filter(|v| !v.is_empty()).collect::<Vec<_>>();
            if map.len() != 6 {
                continue;
            }

            let lib = map[5];

            let (from, to) = map[0].split_once('-').unwrap();
            let from = usize::from_str_radix(from, 16).unwrap();
            let to = usize::from_str_radix(to, 16).unwrap();

            if fs::metadata(lib).is_ok() {
                let ent = maps.entry(lib.to_owned()).or_insert_with(|| (from, to));

                if from < ent.0 {
                    ent.0 = from;
                } else if to > ent.1 {
                    ent.1 = to;
                }
            }
        }

        Ok(maps.into_iter().filter_map(|(k, (from, to))| {
            let path = PathBuf::from(k);
            Some(ModuleAdvancedInfo {
                name: path.file_name()?.to_string_lossy().into_owned(),
                path: path.to_string_lossy().into_owned(),
                base: from as *const u8,
                size: to - from,
            })
        }))
    }
}
