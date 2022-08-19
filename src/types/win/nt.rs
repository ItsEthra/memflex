#![allow(missing_docs)]

use super::{ListEntry, UnicodeString};
use crate::iter_list;

crate::makestruct! {
    /// Thread environment block
    pub struct Teb {
        _pad: [u8; 0x60],
        pub peb: &'static Peb
    }

    /// Process environment block
    pub struct Peb {
        _pad: [u8; 0x18],
        pub ldr: &'static PebLdrData
    }

    pub struct PebLdrData {
        _pad: [u8; 0x20],
        pub in_memory_order_list: ListEntry<LdrDataTableEntry>
    }

    pub struct LdrDataTableEntry {
        pub in_load_order_links: ListEntry<LdrDataTableEntry>,
        pub in_memory_order_links: ListEntry<LdrDataTableEntry>,
        pub in_initialization_order_links: ListEntry<LdrDataTableEntry>,
        pub dll_base: *const u8,
        pub entry_point: *const u8,
        pub image_size: u32,
        pub full_dll_name: UnicodeString,
        pub base_dll_name: UnicodeString
    }
}

impl Teb {
    pub fn current() -> &'static Teb {
        unsafe {
            let mut out: *const Teb;
            core::arch::asm! {
                "mov {}, qword ptr gs:[0x30]",
                out(reg) out
            };
            &*out
        }
    }
}

impl PebLdrData {
    pub fn iter(&self) -> impl Iterator<Item = LdrDataTableEntry> + '_ {
        iter_list!(
            &self.in_memory_order_list,
            LdrDataTableEntry,
            in_memory_order_links
        )
    }
}
