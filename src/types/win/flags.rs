#![allow(missing_docs)]
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/// Access flags, used to describe access to the process
pub struct ProcessRights(pub u32);

impl ProcessRights {
    pub const SYNCHRONIZE: Self = Self(0x00100000);
    pub const VM_WRITE: Self = Self(0x0020);
    pub const VM_READ: Self = Self(0x0010);
    pub const VM_OPERATION: Self = Self(0x0008);
    pub const TERMINATE: Self = Self(0x0001);
    pub const SUSPEND_RESUME: Self = Self(0x0800);
    pub const SET_QUOTA: Self = Self(0x0100);
    pub const SET_INFORMATION: Self = Self(0x0200);
    pub const QUERY_LIMITED_INFORMATION: Self = Self(0x1000);
    pub const QUERY_INFORMATION: Self = Self(0x0400);
    pub const DUP_HANDLE: Self = Self(0x0040);
    pub const CREATE_THREAD: Self = Self(0x0002);
    pub const CREATE_PROCESS: Self = Self(0x0080);
    pub const ALL_ACCESS: Self = Self(0x000F0000 | 0x00100000 | 0xFFFF);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/// Access flags, used to describe access to the process
pub struct ThreadRights(pub u32);

impl ThreadRights {
    pub const SYNCHRONIZE: Self = Self(0x00100000);
    pub const TERMINATE: Self = Self(0x0001);
    pub const SUSPEND_RESUME: Self = Self(0x0002);
    pub const SET_THREAD_TOKEN: Self = Self(0x0080);
    pub const SET_LIMITED_INFORMATION: Self = Self(0x0400);
    pub const SET_INFORMATION: Self = Self(0x0020);
    pub const SET_CONTEXT: Self = Self(0x0010);
    pub const QUERY_LIMITED_INFORMATION: Self = Self(0x0800);
    pub const QUERY_INFORMATION: Self = Self(0x0040);
    pub const IMPERSONATE: Self = Self(0x0100);
    pub const GET_CONTEXT: Self = Self(0x0008);
    pub const DIRECT_IMPERSONATION: Self = Self(0x0200);
    pub const ALL_ACCESS: Self = Self(0x000F0000 | 0x00100000 | 0xFFFF);
}

/// Protection flags, usually used to describe memory
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MemoryProtection(pub u32);

impl MemoryProtection {
    /// Checks if possible to read.
    pub fn read(&self) -> bool {
        match *self {
            Self::EXECUTE
            | Self::EXECUTE_READ
            | Self::EXECUTE_READWRITE
            | Self::EXECUTE_WRITECOPY
            | Self::READONLY
            | Self::READWRITE
            | Self::WRITECOPY => true,
            _ => false,
        }
    }

    /// Checks if possible to write.
    pub fn write(&self) -> bool {
        match *self {
            Self::EXECUTE_READWRITE
            | Self::EXECUTE_WRITECOPY
            | Self::READWRITE
            | Self::WRITECOPY => true,
            _ => false,
        }
    }

    /// Checks if possible to execute.
    pub fn execute(&self) -> bool {
        match *self {
            Self::EXECUTE
            | Self::EXECUTE_READ
            | Self::EXECUTE_READWRITE
            | Self::EXECUTE_WRITECOPY => true,
            _ => false,
        }
    }
}

impl MemoryProtection {
    pub const EXECUTE: Self = Self(0x10);
    pub const EXECUTE_READ: Self = Self(0x20);
    pub const EXECUTE_READWRITE: Self = Self(0x40);
    pub const EXECUTE_WRITECOPY: Self = Self(0x80);
    pub const NOACCESS: Self = Self(0x01);
    pub const READONLY: Self = Self(0x02);
    pub const READWRITE: Self = Self(0x04);
    pub const WRITECOPY: Self = Self(0x08);
    pub const TARGETS_INVALID: Self = Self(0x40000000);
    pub const TARGETS_NO_UPDATE: Self = Self(0x40000000);
    pub const GUARD: Self = Self(0x100);
    pub const NOCACHE: Self = Self(0x200);
    pub const WRITECOMBINE: Self = Self(0x400);
}

/// Allocation flags, used to describe type of newly allocation region
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AllocationType(pub u32);

impl AllocationType {
    pub const COMMIT: Self = Self(0x00001000);
    pub const RESERVE: Self = Self(0x00002000);
    pub const RESET: Self = Self(0x00080000);
    pub const RESET_UNDO: Self = Self(0x1000000);
    pub const LARGE_PAGES: Self = Self(0x20000000);
    pub const PHYSICAL: Self = Self(0x00400000);
    pub const TOP_DOWN: Self = Self(0x00100000);
}

/// Free flags, used to describe how to free memory
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FreeType(pub u32);

impl FreeType {
    pub const DECOMMIT: Self = Self(0x00004000);
    pub const RELEASE: Self = Self(0x00008000);
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

impl_traits!(AllocationType, ProcessRights, MemoryProtection);
