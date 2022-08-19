use memflex::{
    external::OwnedProcess,
    types::{AllocationType, MemoryProtection, ProcessAccess},
};

fn main() {
    let p = OwnedProcess::open_by_id(13484, false, ProcessAccess::PROCESS_ALL_ACCESS).unwrap();

    dbg!(p
        .protect(
            0x0007FF7F016D53A,
            0x10,
            MemoryProtection::PAGE_EXECUTE_READWRITE
        )
        .unwrap());
    dbg!(p.write::<u64>(0x0007FF7F016D53A, 0x90).unwrap());

    let v = dbg!(p
        .allocate(
            None,
            0x10,
            AllocationType::MEM_COMMIT | AllocationType::MEM_RESERVE,
            MemoryProtection::PAGE_EXECUTE_READWRITE
        )
        .unwrap());

    println!("{:X}", v);
}
