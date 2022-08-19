use memflex::types::{ListEntry, UnicodeString};

memflex::makestruct! {
    struct Teb {
        _pad: [u8; 0x60],
        peb: &'static Peb
    }

    struct Peb {
        _pad: [u8; 0x18],
        ldr: &'static PebLdrData
    }

    struct PebLdrData {
        _pad: [u8; 0x20],
        in_memory_order_list: ListEntry<LdrDataTableEntry>
    }

    struct LdrDataTableEntry {
        in_load_order_links: ListEntry<LdrDataTableEntry>,
        in_memory_order_links: ListEntry<LdrDataTableEntry>,
        in_initialization_order_links: ListEntry<LdrDataTableEntry>,
        dll_base: *const u8,
        entry_point: *const u8,
        image_size: u32,
        full_dll_name: UnicodeString,
        base_dll_name: UnicodeString
    }
}

#[cfg(windows)]
fn main() {
    use memflex::iter_list;

    unsafe {
        #[link(name = "ntdll")]
        extern "C" {
            fn NtCurrentTeb() -> &'static Teb;
        }

        let ldr = NtCurrentTeb().peb.ldr;
        for e in iter_list!(&ldr.in_memory_order_list, LdrDataTableEntry, in_memory_order_links) {
            println!("{}", e.base_dll_name.to_string().unwrap());
        }
        loop {}
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This example can only run on windows");
}