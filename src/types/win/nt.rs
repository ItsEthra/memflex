#![allow(missing_docs)]

use super::{ListEntry, UnicodeString};
use crate::assert_offset;
use core::ptr::NonNull;

crate::makestruct! {
    /// Thread environment block
    pub struct Teb {
        _pad: [u8; 0x60],
        pub peb: Option<NonNull<Peb>>
    }

    /// Process environment block
    pub struct Peb {
        _pad: [u8; 0x18],
        pub ldr: Option<NonNull<PebLdrData>>,
    }

    pub struct PebLdrData {
        _pad: [u8; 0x20],
        pub in_memory_order_list: ListEntry<0x20, LdrDataTableEntry>
    }

    pub struct LdrDataTableEntry {
        pub in_load_order_links: ListEntry<0x00, LdrDataTableEntry>,
        pub in_memory_order_links: ListEntry<0x10, LdrDataTableEntry>,
        pub in_initialization_order_links: ListEntry<0x20, LdrDataTableEntry>,
        pub dll_base: *const u8,
        pub entry_point: *const u8,
        pub image_size: u32,
        pub full_dll_name: UnicodeString,
        pub base_dll_name: UnicodeString
    }
}

assert_offset!(PebLdrData, in_memory_order_list, 0x20);
assert_offset!(
    LdrDataTableEntry,
    in_load_order_links,
    0x00,
    in_memory_order_links,
    0x10,
    in_initialization_order_links,
    0x20
);

impl Teb {
    pub unsafe fn get<'r>() -> &'r Teb {
        let mut out: *const Teb;
        core::arch::asm! {
            "mov {}, qword ptr gs:[0x30]",
            out(reg) out
        };
        &*out
    }

    pub unsafe fn get_mut<'r>() -> &'r mut Teb {
        let mut out: *mut Teb;
        core::arch::asm! {
            "mov {}, qword ptr gs:[0x30]",
            out(reg) out
        };
        &mut *out
    }
}

impl PebLdrData {
    pub unsafe fn iter(&self) -> impl Iterator<Item = &LdrDataTableEntry> + '_ {
        self.in_memory_order_list.iter()
    }
}
