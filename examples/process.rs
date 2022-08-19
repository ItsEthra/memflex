use memflex::{external::open_process_by_id, types::ProcessAccess, ida_pat};

fn main() {
    let p = open_process_by_id(2396, false, ProcessAccess::PROCESS_ALL_ACCESS).unwrap();

    for sig in p.find_pattern_in_module(ida_pat!("4D 0F 42 D3"), "ntdll.dll").unwrap() {
        println!("{sig:#X}");
    }
    println!("Stop");
}
