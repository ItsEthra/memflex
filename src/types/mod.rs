mod vmt;
pub use vmt::*;
mod tstr;
pub use tstr::*;

/// Windows datatypes
#[cfg(windows)]
pub mod win;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

/// Basic information about module
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "internal")]
pub struct ModuleBasicInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
}

/// More information about module
#[derive(Debug, Clone)]
#[cfg(all(feature = "alloc", feature = "internal"))]
pub struct ModuleAdvancedInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
    /// Module's name
    pub name: String,
    /// Module's full path
    pub path: String,
}

#[cfg(windows)]
impl From<&windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W> for ModuleAdvancedInfo {
    fn from(me: &windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W) -> Self {
        Self {
            base: me.modBaseAddr as _,
            size: me.modBaseSize as _,
            name: String::from_utf16_lossy(unsafe {
                crate::terminated_array(me.szModule.as_ptr(), 0)
            }),
            path: String::from_utf16_lossy(unsafe {
                crate::terminated_array(me.szExePath.as_ptr(), 0)
            }),
        }
    }
}

bitflags::bitflags! {
    /// Protection of a memory region.
    #[derive(Default)]
    pub struct Protection : u8 {
        /// Read
        const R = 0b001;
        /// Write
        const W = 0b010;
        /// Execute
        const X = 0b100;
        /// Read | Write
        const RW = 0b011;
        /// Read | Execute
        const RX = 0b101;
        /// Read | Write | Execute
        const RWX = 0b111;
    }
}

impl Protection {
    /// Can read?
    #[inline]
    pub fn read(&self) -> bool {
        self.contains(Self::R)
    }

    /// Can write?
    #[inline]
    pub fn write(&self) -> bool {
        self.contains(Self::W)
    }

    /// Can execute?
    #[inline]
    pub fn execute(&self) -> bool {
        self.contains(Self::X)
    }

    /// Parses string of kind `r-x`.
    /// # Panics
    /// * `s.len()` isn't 3.
    /// * s\[0\] != r | -
    /// * s\[1\] != w | -
    /// * s\[2\] != x | -
    pub fn parse(s: &str) -> Self {
        assert!(
            s.len() == 3
                && s.is_ascii()
                && s.chars()
                    .all(|c| c == '-' || c == 'r' || c == 'w' || c == 'x')
        );

        let mut prot = Self::empty();
        if s.as_bytes()[0] == b'r' {
            prot |= Self::R;
        }

        if s.as_bytes()[1] == b'w' {
            prot |= Self::W;
        }

        if s.as_bytes()[2] == b'x' {
            prot |= Self::X;
        }

        prot
    }

    /// Converts from os protection type.
    #[cfg(windows)]
    pub fn from_os(value: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS) -> Option<Self> {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
        };

        match value {
            PAGE_NOACCESS => Some(Self::empty()),
            PAGE_READONLY => Some(Self::R),
            PAGE_READWRITE => Some(Self::RW),
            PAGE_EXECUTE_READWRITE => Some(Self::RWX),
            PAGE_EXECUTE_READ => Some(Self::RX),
            _ => None,
        }
    }
}
