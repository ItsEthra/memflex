use crate::{
    external::{MemoryRegion, ProcessEntry},
    sizeof,
    types::{ModuleAdvancedInfo, Protection},
    Matcher, MfError,
};
use core::{
    mem::zeroed,
    slice::{from_raw_parts, from_raw_parts_mut},
};
use std::fs;

/// Represents a single process in the system.
/// # Details
/// There is no such concept as 'owned' procses in unix. (i think).
/// The name is the same as on windows to reduce the hasle of cross-platform code.
#[derive(Debug)]
#[repr(transparent)]
pub struct OwnedProcess(pub(crate) u32);

impl OwnedProcess {
    /// Returns the id of the process.
    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    /// Returns full path to the process.
    pub fn path(&self) -> crate::Result<String> {
        Ok(fs::read_link(format!("/proc/{}/exe", self.0))
            .map_err(|_| MfError::ProcessDied)?
            .to_string_lossy()
            .into_owned())
    }

    /// Returns the name of the process
    pub fn name(&self) -> crate::Result<String> {
        Ok(fs::read_link(format!("/proc/{}/exe", self.0))
            .map_err(|_| MfError::ProcessDied)?
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default())
    }

    /// Reads process memory, returning amount of bytes read.
    pub fn read_buf(&self, address: usize, buf: &mut [u8]) -> crate::Result<usize> {
        unsafe {
            let read = libc::process_vm_readv(
                self.0 as _,
                &libc::iovec {
                    iov_base: buf.as_mut_ptr() as _,
                    iov_len: buf.len(),
                },
                1,
                &libc::iovec {
                    iov_base: address as _,
                    iov_len: buf.len(),
                },
                1,
                0,
            );

            if read == -1 {
                MfError::last()
            } else {
                Ok(read as usize)
            }
        }
    }

    /// Reads a value of type `T` at `address`.
    pub fn read<T>(&self, address: usize) -> crate::Result<T> {
        unsafe {
            let mut buf: T = zeroed();
            self.read_buf(
                address,
                from_raw_parts_mut(&mut buf as *mut T as *mut u8, sizeof!(T)),
            )?;
            Ok(buf)
        }
    }

    /// Reads zero terminated string at `address`.
    pub fn read_str(&self, address: usize) -> crate::Result<String> {
        const BUF_SIZE: usize = 4;

        let mut out = vec![];
        let mut offset = 0;

        loop {
            let buf = self.read::<[u8; BUF_SIZE]>(address + offset)?;

            if let Some(i) = buf.iter().position(|b| *b == 0) {
                out.extend_from_slice(&buf[..i]);
                break;
            } else {
                out.extend_from_slice(&buf);
            }

            offset += BUF_SIZE
        }

        Ok(String::from_utf8(out).map_err(|_| MfError::InvalidString)?)
    }

    /// Writes process memory, returning amount of bytes written.
    pub fn write_buf(&self, address: usize, buf: &[u8]) -> crate::Result<usize> {
        unsafe {
            let written = libc::process_vm_writev(
                self.0 as _,
                &libc::iovec {
                    iov_base: buf.as_ptr() as _,
                    iov_len: buf.len(),
                },
                1,
                &libc::iovec {
                    iov_base: address as _,
                    iov_len: buf.len(),
                },
                1,
                0,
            );

            if written == -1 {
                MfError::last()
            } else {
                Ok(written as usize)
            }
        }
    }

    /// Writes `value` at `address` in the process's memory, returning amount of bytes written.
    pub fn write<T>(&self, address: usize, value: T) -> crate::Result<usize> {
        unsafe {
            self.write_buf(
                address,
                from_raw_parts(&value as *const T as *const u8, sizeof!(T)),
            )
        }
    }

    /// Returns an iterator over process's modules.
    pub fn modules(&self) -> crate::Result<impl Iterator<Item = ModuleAdvancedInfo>> {
        use std::{collections::HashMap, path::PathBuf};

        let s = fs::read_to_string(format!("/proc/{}/maps", self.0))
            .map_err(|_| MfError::ProcessDied)?;
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

    /// Searches for the specified module in the process.
    /// # Case
    /// Search is done case insensetive.
    pub fn find_module(&self, name: &str) -> crate::Result<ModuleAdvancedInfo> {
        self.modules()?
            .find(|m| m.name.eq_ignore_ascii_case(name))
            .ok_or(MfError::ModuleNotFound)
    }

    /// Finds all occurences of the pattern in a given range.
    // TODO: Can be optimized
    pub fn find_pattern<'a>(
        &'a self,
        pat: impl Matcher + 'a,
        start: usize,
        len: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        let mut offset = 0;
        let mut buf = vec![0; pat.len()];

        std::iter::from_fn(move || {
            loop {
                if self.read_buf(start + offset, &mut buf[..]).is_err() {
                    return None;
                }

                if pat.matches(&buf[..]) {
                    break;
                }

                offset += 1;

                if offset >= len {
                    return None;
                }
            }

            offset += 1;
            Some(start + offset - 1)
        })
        .fuse()
    }

    /// Returns an iterator over mapped regions in the process.
    pub fn maps(&self) -> crate::Result<Vec<MemoryRegion>> {
        Ok(fs::read_to_string(format!("/proc/{}/maps", self.0))
            .map_err(|_| MfError::ProcessDied)?
            .lines()
            .map(|l| {
                let mut iter = l.split(' ');
                let (from, to) = iter.next().unwrap().split_once('-').unwrap();

                let from = usize::from_str_radix(from, 16).unwrap();
                let to = usize::from_str_radix(to, 16).unwrap();

                let prot = Protection::parse(&iter.next().unwrap()[0..3]);

                MemoryRegion { from, to, prot }
            })
            .collect())
    }

    /// Queryies protection for the specified address.
    /// `None` if no mappings were found for this address.
    pub fn query(&self, address: usize) -> crate::Result<Option<Protection>> {
        Ok(self
            .maps()?
            .iter()
            .find(|r| r.from <= address && r.to <= address)
            .map(|r| r.prot))
    }

    /// Resolves multilevel pointer
    pub fn resolve_multilevel(&self, mut base: usize, offsets: &[usize]) -> crate::Result<usize> {
        for &o in offsets {
            base = self.read(base + o)?;
        }

        Ok(base)
    }
}

/// Iterator over all processes in the system.
pub struct ProcessIterator(Box<dyn Iterator<Item = ProcessEntry>>);

impl ProcessIterator {
    /// Creates new iterator over all processes in the system.
    /// # Unix
    /// Always returns Ok(I).
    pub fn new() -> crate::Result<Self> {
        let iter = fs::read_dir("/proc")
            .unwrap()
            .flatten()
            .filter(|de| {
                de.file_type().map(|t| t.is_dir()).unwrap_or_default()
                    && de
                        .file_name()
                        .to_string_lossy()
                        .chars()
                        .all(|c| c.is_numeric())
            })
            .filter_map(|de| {
                let id = de.file_name().to_string_lossy().parse::<u32>().unwrap();
                Some(id)
            })
            .map(|id| ProcessEntry {
                id,
                name: path.rsplit_once('/').unwrap().1.to_owned(),
                parent_id: hella_cringe(fs::read_to_string(format!("/proc/{id}/stat")).unwrap()),
            });

        Ok(Self(Box::new(iter)))
    }
}

// sigh, am i really this stupid?
// I don't know how to make it better pls help me
fn hella_cringe(mut stat: String) -> u32 {
    let (from, to) = (stat.find('(').unwrap(), stat.find(')').unwrap());

    // Example from /proc/id/stat
    // 123 (Socket Process) 123 123
    //            ^
    //            |
    // This is why I have severe depression
    stat.drain(from..=(to + 1));
    stat.split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

impl Iterator for ProcessIterator {
    type Item = ProcessEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Searches for the specified process by its name.
pub fn find_process_by_name(name: &str) -> crate::Result<OwnedProcess> {
    ProcessIterator::new()?
        .find_map(|pe| {
            if pe.name.eq_ignore_ascii_case(name) {
                Some(pe.open())
            } else {
                None
            }
        })
        .ok_or(MfError::ProcessNotFound)?
}

/// Searches for the specified process by its id.
pub fn find_process_by_id(id: u32) -> crate::Result<OwnedProcess> {
    if fs::metadata(format!("/proc/{id}")).is_err() {
        return Err(MfError::ProcessNotFound);
    }

    Ok(OwnedProcess(id))
}
