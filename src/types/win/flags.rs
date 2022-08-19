#![allow(missing_docs)]

pub const PROCESS_SYNCHRONIZE: u32 = 0x00100000;
pub const PROCESS_VM_WRITE: u32 = 0x0020;
pub const PROCESS_VM_READ: u32 = 0x0010;
pub const PROCESS_VM_OPERATION: u32 = 0x0008;
pub const PROCESS_TERMINATE: u32 = 0x0001;
pub const PROCESS_SUSPEND_RESUME: u32 = 0x0800;
pub const PROCESS_SET_QUOTA: u32 = 0x0100;
pub const PROCESS_SET_INFORMATION: u32 = 0x0200;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
pub const PROCESS_DUP_HANDLE: u32 = 0x0040;
pub const PROCESS_CREATE_THREAD: u32 = 0x0002;
pub const PROCESS_CREATE_PROCESS: u32 = 0x0080;
pub const PROCESS_ALL_ACCESS: u32 = 0x000F0000 | 0x00100000 | 0xFFFF;

/// Protection flags, usually used to describe memory
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ProtectionFlags(pub u32);

impl ProtectionFlags {
    /// Checks if possible to read.
    pub fn read(&self) -> bool {
        match *self {
            Self::PAGE_EXECUTE
            | Self::PAGE_EXECUTE_READ
            | Self::PAGE_EXECUTE_READWRITE
            | Self::PAGE_EXECUTE_WRITECOPY
            | Self::PAGE_READONLY
            | Self::PAGE_READWRITE
            | Self::PAGE_WRITECOPY => true,
            _ => false
        }
    }

    /// Checks if possible to write.
    pub fn write(&self) -> bool {
        match *self {
            Self::PAGE_EXECUTE_READWRITE
            | Self::PAGE_EXECUTE_WRITECOPY
            | Self::PAGE_READWRITE
            | Self::PAGE_WRITECOPY => true,
            _ => false
        }
    }

    /// Checks if possible to execute.
    pub fn execute(&self) -> bool {
        match *self {
            Self::PAGE_EXECUTE
            | Self::PAGE_EXECUTE_READ
            | Self::PAGE_EXECUTE_READWRITE
            | Self::PAGE_EXECUTE_WRITECOPY => true,
            _ => false
        }
    }
}

impl ProtectionFlags {
    pub const PAGE_EXECUTE: Self = Self(0x10);
    pub const PAGE_EXECUTE_READ: Self = Self(0x20);
    pub const PAGE_EXECUTE_READWRITE: Self = Self(0x40);
    pub const PAGE_EXECUTE_WRITECOPY: Self = Self(0x80);
    pub const PAGE_NOACCESS: Self = Self(0x01);
    pub const PAGE_READONLY: Self = Self(0x02);
    pub const PAGE_READWRITE: Self = Self(0x04);
    pub const PAGE_WRITECOPY: Self = Self(0x08);
    pub const PAGE_TARGETS_INVALID: Self = Self(0x40000000);
    pub const PAGE_TARGETS_NO_UPDATE: Self = Self(0x40000000);
}
