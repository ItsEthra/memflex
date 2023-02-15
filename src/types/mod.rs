mod vmt;
pub use vmt::*;
mod tstr;
pub use tstr::*;
mod prot;
pub use prot::*;

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
pub struct ModuleInfo {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
}

/// More information about module
#[derive(Debug, Clone)]
#[cfg(all(feature = "alloc", feature = "internal"))]
pub struct ModuleInfoWithName {
    /// Module's base
    pub base: *const u8,
    /// Module's size
    pub size: usize,
    /// Module's name
    pub name: String,
}

#[cfg(all(windows, feature = "std"))]
impl From<&windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W> for ModuleInfoWithName {
    fn from(me: &windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32W) -> Self {
        Self {
            base: me.modBaseAddr as _,
            size: me.modBaseSize as _,
            name: String::from_utf16_lossy(unsafe {
                crate::terminated_array(me.szModule.as_ptr(), 0)
            }),
        }
    }
}
