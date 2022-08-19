use memflex::{external::OwnedProcess, types::{ProcessAccess, ProtectionFlags}};

fn main() {
    let p = OwnedProcess::open_by_id(13484, false, ProcessAccess::PROCESS_ALL_ACCESS)
        .unwrap();

    dbg!(p.protect(0x0007FF7F016D53A, 0x10, ProtectionFlags::PAGE_EXECUTE_READWRITE).unwrap());
    dbg!(p.write::<u64>(0x0007FF7F016D53A, 0x90).unwrap());
}