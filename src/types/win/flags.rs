#![allow(missing_docs)]
use core::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/// Access flags, used to describe access to the process
pub struct ProcessAccess(pub u32);

impl ProcessAccess {
    pub const PROCESS_SYNCHRONIZE: Self = Self(0x00100000);
    pub const PROCESS_VM_WRITE: Self = Self(0x0020);
    pub const PROCESS_VM_READ: Self = Self(0x0010);
    pub const PROCESS_VM_OPERATION: Self = Self(0x0008);
    pub const PROCESS_TERMINATE: Self = Self(0x0001);
    pub const PROCESS_SUSPEND_RESUME: Self = Self(0x0800);
    pub const PROCESS_SET_QUOTA: Self = Self(0x0100);
    pub const PROCESS_SET_INFORMATION: Self = Self(0x0200);
    pub const PROCESS_QUERY_LIMITED_INFORMATION: Self = Self(0x1000);
    pub const PROCESS_QUERY_INFORMATION: Self = Self(0x0400);
    pub const PROCESS_DUP_HANDLE: Self = Self(0x0040);
    pub const PROCESS_CREATE_THREAD: Self = Self(0x0002);
    pub const PROCESS_CREATE_PROCESS: Self = Self(0x0080);
    pub const PROCESS_ALL_ACCESS: Self = Self(0x000F0000 | 0x00100000 | 0xFFFF);
}

/// Protection flags, usually used to describe memory
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MemoryProtection(pub u32);

impl MemoryProtection {
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
            _ => false,
        }
    }

    /// Checks if possible to write.
    pub fn write(&self) -> bool {
        match *self {
            Self::PAGE_EXECUTE_READWRITE
            | Self::PAGE_EXECUTE_WRITECOPY
            | Self::PAGE_READWRITE
            | Self::PAGE_WRITECOPY => true,
            _ => false,
        }
    }

    /// Checks if possible to execute.
    pub fn execute(&self) -> bool {
        match *self {
            Self::PAGE_EXECUTE
            | Self::PAGE_EXECUTE_READ
            | Self::PAGE_EXECUTE_READWRITE
            | Self::PAGE_EXECUTE_WRITECOPY => true,
            _ => false,
        }
    }
}

impl MemoryProtection {
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
    pub const PAGE_GUARD: Self = Self(0x100);
    pub const PAGE_NOCACHE: Self = Self(0x200);
    pub const PAGE_WRITECOMBINE: Self = Self(0x400);
}

/// Allocation flags, used to describe type of newly allocation region
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AllocationType(pub u32);

impl AllocationType {
    pub const MEM_COMMIT: Self = Self(0x00001000);
    pub const MEM_RESERVE: Self = Self(0x00002000);
    pub const MEM_RESET: Self = Self(0x00080000);
    pub const MEM_RESET_UNDO: Self = Self(0x1000000);
    pub const MEM_LARGE_PAGES: Self = Self(0x20000000);
    pub const MEM_PHYSICAL: Self = Self(0x00400000);
    pub const MEM_TOP_DOWN: Self = Self(0x00100000);
}

macro_rules! impl_traits {
    ($($ty:ty),*) => {
        $(
            impl BitOr for $ty {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self::Output {
                    Self(self.0 | rhs.0)
                }
            }

            impl BitOrAssign for $ty {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0 |= rhs.0
                }
            }

            impl BitAnd for $ty {
                type Output = Self;

                fn bitand(self, rhs: Self) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }

            impl BitAndAssign for $ty {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0 &= rhs.0
                }
            }
        )*
    };
}

impl_traits!(AllocationType, ProcessAccess, MemoryProtection);
