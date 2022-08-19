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
#[test]
fn test_linked() {
    unsafe {
        #[link(name = "ntdll")]
        extern "C" {
            fn NtCurrentTeb() -> &'static Teb;
        }

        let ldr = NtCurrentTeb().peb.ldr;
        println!("{ldr:p}");
        let head = ldr.in_memory_order_list.next
            .cast::<u8>()
            .sub(memoffset::offset_of!(LdrDataTableEntry, in_memory_order_links))
            .cast::<LdrDataTableEntry>()
            .read();
        println!("{:?}", head.base_dll_name.to_string());
        loop {}
    }
}