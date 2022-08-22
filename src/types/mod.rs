mod vmt;
pub use vmt::*;
mod strptr;
pub use strptr::*;

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
