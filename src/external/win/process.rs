use super::{ModuleIterator, OwnedThread, ThreadIterator};
use crate::{
    external::{MemoryRegion, ProcessEntry},
    types::{ModuleAdvancedInfo, Protection},
    Matcher, MfError,
};
use core::mem::{size_of, transmute, zeroed};
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE, MAX_PATH},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                TH32CS_SNAPPROCESS,
            },
        },
        Memory::{
            VirtualAllocEx, VirtualFreeEx, VirtualProtectEx, VirtualQueryEx,
            MEMORY_BASIC_INFORMATION, PAGE_NOACCESS, PAGE_PROTECTION_FLAGS,
            VIRTUAL_ALLOCATION_TYPE, VIRTUAL_FREE_TYPE,
        },
        ProcessStatus::K32GetProcessImageFileNameW,
        Threading::{
            CreateRemoteThread, GetProcessId, OpenProcess, TerminateProcess, PROCESS_ACCESS_RIGHTS,
        },
    },
};

/// Owned handle to another process
#[repr(transparent)]
pub struct OwnedProcess(HANDLE);

impl OwnedProcess {
    /// Takes ownership of handle.
    pub unsafe fn from_handle(h: HANDLE) -> Self {
        Self(h)
    }

    /// Gives away ownership of the handle.
    pub fn into_handle(self) -> HANDLE {
        self.0
    }

    /// Closes handle to the process.
    pub fn close(self) -> crate::Result<()> {
        if unsafe { CloseHandle(self.0) }.as_bool() {
            Ok(())
        } else {
            MfError::last()
        }
    }

    /// Returns current mapped memory regions.
    // TODO(ItsEthra): I don't think it's acurate.
    pub fn maps(&self) -> crate::Result<Vec<MemoryRegion>> {
        let mut maps = vec![];

        unsafe {
            let mut address = 0;
            let mut info = MEMORY_BASIC_INFORMATION::default();
            while VirtualQueryEx(
                self.0,
                Some(address as _),
                &mut info,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            ) > 0
            {
                address = info.BaseAddress as usize + info.RegionSize;
                if info.AllocationProtect == PAGE_NOACCESS {
                    continue;
                }

                maps.push(MemoryRegion {
                    from: info.BaseAddress as _,
                    to: address,
                    prot: Protection::from_os(info.Protect).unwrap_or_default(),
                });
            }
        }

        Ok(maps)
    }

    /// Returns the name of the process.
    pub fn name(&self) -> crate::Result<String> {
        let mut image_name = [0; MAX_PATH as usize];
        unsafe {
            let size = K32GetProcessImageFileNameW(self.0, &mut image_name) as usize;
            if size == 0 {
                return Err(MfError::ProcessDied);
            }

            let last = image_name
                .iter()
                .rposition(|c| char::decode_utf16([*c]).next() == Some(Ok('\\')))
                .unwrap_or(0);
            Ok(String::from_utf16_lossy(&image_name[last + 1..size]))
        }
    }

    /// Reads process memory, returns amount of bytes read.
    pub fn read_buf(&self, address: usize, mut buf: impl AsMut<[u8]>) -> crate::Result<usize> {
        let mut read = 0;
        let buf = buf.as_mut();

        unsafe {
            if ReadProcessMemory(
                self.0,
                address as _,
                buf.as_mut_ptr() as _,
                buf.len() as _,
                Some(&mut read as _),
            )
            .as_bool()
            {
                Ok(read)
            } else {
                MfError::last()
            }
        }
    }

    /// Reads process memory, returning the value read at the `address`.
    pub fn read<T>(&self, address: usize) -> crate::Result<T> {
        unsafe {
            let mut buf: T = zeroed();

            if ReadProcessMemory(
                self.0,
                address as _,
                &mut buf as *mut T as _,
                size_of::<T>() as _,
                None,
            )
            .as_bool()
            {
                Ok(buf)
            } else {
                MfError::last()
            }
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

    /// Writes buffer to the process memory, returning the amount of bytes written.
    pub fn write_buf(&self, address: usize, buf: impl AsRef<[u8]>) -> crate::Result<usize> {
        let mut written: usize = 0;
        let buf = buf.as_ref();

        unsafe {
            if WriteProcessMemory(
                self.0,
                address as _,
                buf.as_ptr() as _,
                buf.len() as _,
                Some(&mut written),
            )
            .as_bool()
            {
                Ok(written)
            } else {
                MfError::last()
            }
        }
    }

    /// Writes value to the process memory, returning the amount of bytes written.
    pub fn write<T>(&self, address: usize, value: T) -> crate::Result<usize> {
        let mut written: usize = 0;

        unsafe {
            if WriteProcessMemory(
                self.0,
                address as _,
                &value as *const T as _,
                size_of::<T>() as _,
                Some(&mut written),
            )
            .as_bool()
            {
                Ok(written)
            } else {
                MfError::last()
            }
        }
    }

    /// Writes string to the specified address, putting 0 at the end
    pub fn write_str(&self, address: usize, text: impl AsRef<str>) -> crate::Result<usize> {
        let text = text.as_ref();
        let mut wrote = self.write_buf(address, text.as_bytes())?;
        wrote += self.write(address + wrote, 0)?;
        Ok(wrote)
    }

    /// Changes the protection of memory pages, returning the old protection value.
    pub fn protect(
        &self,
        address: usize,
        size: usize,
        protection: PAGE_PROTECTION_FLAGS,
    ) -> crate::Result<PAGE_PROTECTION_FLAGS> {
        let mut old = PAGE_PROTECTION_FLAGS(0);
        unsafe {
            if VirtualProtectEx(self.0, address as _, size, protection, &mut old).as_bool() {
                Ok(old)
            } else {
                MfError::last()
            }
        }
    }

    /// Allocates new region of memory, returning pointer to it.
    pub fn allocate(
        &self,
        desired_address: Option<usize>,
        size: usize,
        alloc_type: VIRTUAL_ALLOCATION_TYPE,
        protection: PAGE_PROTECTION_FLAGS,
    ) -> crate::Result<usize> {
        unsafe {
            let addr = VirtualAllocEx(
                self.0,
                Some(desired_address.unwrap_or_default() as _),
                size,
                alloc_type,
                protection,
            );

            if addr.is_null() {
                MfError::last()
            } else {
                Ok(addr as _)
            }
        }
    }

    /// Frees region of memory.
    /// # Note
    /// If `free_type` is `MEM_RELEASE` then `size` must be 0.
    pub fn free(
        &self,
        address: usize,
        size: usize,
        free_type: VIRTUAL_FREE_TYPE,
    ) -> crate::Result<()> {
        unsafe {
            if VirtualFreeEx(self.0, address as _, size, free_type).as_bool() {
                MfError::last()
            } else {
                Ok(())
            }
        }
    }

    /// Creates thread running in the process's context.
    pub fn create_thread(
        &self,
        stack_size: Option<usize>,
        start_address: usize,
        param: usize,
        suspended: bool,
    ) -> crate::Result<OwnedThread> {
        unsafe {
            if let Ok(h) = CreateRemoteThread(
                self.0,
                None,
                stack_size.unwrap_or_default(),
                transmute(start_address),
                Some(param as _),
                if suspended { 0x4 } else { 0 },
                None,
            ) {
                Ok(OwnedThread::from_handle(h))
            } else {
                MfError::last()
            }
        }
    }

    /// Returns the id of the process.
    pub fn id(&self) -> u32 {
        unsafe { GetProcessId(self.0) }
    }

    /// Returns an iterator over process's modules.
    pub fn modules(&self) -> crate::Result<impl Iterator<Item = ModuleAdvancedInfo>> {
        ModuleIterator::new(self.id())
    }

    /// Returns an iterator over process's threads.
    pub fn threads(&self) -> crate::Result<ThreadIterator> {
        ThreadIterator::new(self.id())
    }

    /// Searches for the module in the process.
    pub fn find_module(&self, module_name: &str) -> crate::Result<ModuleAdvancedInfo> {
        self.modules()?
            .find(|me| me.name.eq_ignore_ascii_case(module_name))
            .ok_or(MfError::ModuleNotFound)
    }

    /// Finds all occurences of the pattern in a given range.
    // @TODO: Can be optimized
    pub fn find_pattern<'a>(
        &'a self,
        pat: impl Matcher + 'a,
        start: usize,
        len: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        let mut offset = 0;
        let mut buf = vec![0; pat.size()];

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

    /// Finds all occurences of the pattern in the specified module.
    pub fn find_pattern_in_module<'a>(
        &'a self,
        pat: impl Matcher + 'a,
        module_name: &str,
    ) -> crate::Result<impl Iterator<Item = usize> + 'a> {
        let module = self.find_module(module_name)?;
        Ok(self.find_pattern(pat, module.base as _, module.size))
    }

    /// Resolves multilevel pointer
    pub fn resolve_multilevel(&self, mut base: usize, offsets: &[usize]) -> crate::Result<usize> {
        for &o in offsets {
            base = self.read(base + o)?;
        }

        Ok(base)
    }

    /// Terminates the process with the specified code.
    pub fn terminate(&self, exit_code: u32) -> crate::Result<()> {
        unsafe {
            if TerminateProcess(self.0, exit_code).as_bool() {
                Ok(())
            } else {
                MfError::last()
            }
        }
    }

    /// Suspends the process with `NtSuspendProcess`
    pub fn suspend(&self) -> crate::Result<()> {
        #[link(name = "ntdll")]
        extern "C" {
            fn NtSuspendProcess(h: HANDLE) -> i32;
        }

        unsafe {
            if NtSuspendProcess(self.0) == 0 {
                Ok(())
            } else {
                MfError::last()
            }
        }
    }

    /// Resumes the process with `NtResumeProcess`
    pub fn resume(&self) -> crate::Result<()> {
        #[link(name = "ntdll")]
        extern "C" {
            fn NtResumeProcess(h: HANDLE) -> i32;
        }

        unsafe {
            if NtResumeProcess(self.0) == 0 {
                Ok(())
            } else {
                MfError::last()
            }
        }
    }
}

/// Iterator over all processes in the system.
pub struct ProcessIterator {
    h: HANDLE,
    entry: PROCESSENTRY32W,
    stop: bool,
}

impl ProcessIterator {
    /// Creates new iterator over processes.
    pub fn new() -> crate::Result<Self> {
        unsafe {
            if let Ok(h) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                let mut this = Self {
                    h,
                    entry: zeroed(),
                    stop: false,
                };
                this.entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

                if Process32FirstW(this.h, &mut this.entry).as_bool() {
                    Ok(this)
                } else {
                    MfError::last()
                }
            } else {
                MfError::last()
            }
        }
    }
}

impl Iterator for ProcessIterator {
    type Item = ProcessEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let pe = loop {
            unsafe {
                let current = ProcessEntry::from(&self.entry);
                self.stop = !Process32NextW(self.h, &mut self.entry).as_bool();

                if current.is_some() {
                    break current;
                }
            }
        };

        pe
    }
}

/// Tried to open process by name
pub fn open_process_by_name(
    name: &str,
    inherit_handle: bool,
    access_rights: PROCESS_ACCESS_RIGHTS,
) -> crate::Result<OwnedProcess> {
    ProcessIterator::new()?
        .find_map(|pe| {
            if pe.path.contains(name) {
                Some(pe.open(inherit_handle, access_rights))
            } else {
                None
            }
        })
        .ok_or(MfError::ProcessNotFound)?
}

/// Tried to open process by id
pub fn open_process_by_id(
    id: u32,
    inherit_handle: bool,
    access_rights: PROCESS_ACCESS_RIGHTS,
) -> crate::Result<OwnedProcess> {
    unsafe {
        if let Ok(h) = OpenProcess(access_rights, BOOL(inherit_handle as _), id) {
            Ok(OwnedProcess(h))
        } else {
            MfError::last()
        }
    }
}
